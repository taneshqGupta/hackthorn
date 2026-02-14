use crate::error::AppError;
use crate::structs::{ApiResponse, GoogleUserInfo, User, UserResponse, UserRole, UserStatus};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Json,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::env;
use tower_sessions::Session as TowerSession;
use uuid::Uuid;

const SESSION_USER_ID_KEY: &str = "user_id";

#[derive(Debug, Deserialize)]
pub struct GoogleCallbackQuery {
    code: String,
    state: Option<String>,
}

#[derive(Debug, Serialize)]
struct GoogleTokenRequest {
    code: String,
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    grant_type: String,
}

#[derive(Debug, Deserialize)]
struct GoogleTokenResponse {
    access_token: String,
}

pub async fn google_login_initiate(
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> impl IntoResponse {
    let client_id = env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set");
    let redirect_uri = env::var("GOOGLE_REDIRECT_URL").expect("GOOGLE_REDIRECT_URL must be set");
    
    // Get the frontend origin from query parameter or default to FRONTEND_URL
    let frontend_origin = params.get("origin").cloned()
        .unwrap_or_else(|| env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:4173".to_string()));
    
    let auth_url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}&response_type=code&scope=openid%20email%20profile&state={}",
        client_id, redirect_uri, urlencoding::encode(&frontend_origin)
    );
    
    Redirect::to(&auth_url)
}

pub async fn google_callback(
    Query(query): Query<GoogleCallbackQuery>,
    State(pool): State<PgPool>,
    session: TowerSession,
) -> Result<impl IntoResponse, AppError> {
    let client_id = env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set");
    let client_secret = env::var("GOOGLE_CLIENT_SECRET").expect("GOOGLE_CLIENT_SECRET must be set");
    let redirect_uri = env::var("GOOGLE_REDIRECT_URL").expect("GOOGLE_REDIRECT_URL must be set");

    let client = reqwest::Client::new();
    
    let token_response = client
        .post("https://oauth2.googleapis.com/token")
        .json(&GoogleTokenRequest {
            code: query.code,
            client_id,
            client_secret,
            redirect_uri,
            grant_type: "authorization_code".to_string(),
        })
        .send()
        .await
        .map_err(|e| AppError::HttpError(StatusCode::INTERNAL_SERVER_ERROR, e.into()))?
        .json::<GoogleTokenResponse>()
        .await
        .map_err(|e| AppError::HttpError(StatusCode::INTERNAL_SERVER_ERROR, e.into()))?;

    let user_info = client
        .get("https://www.googleapis.com/oauth2/v3/userinfo")
        .bearer_auth(&token_response.access_token)
        .send()
        .await
        .map_err(|e| AppError::HttpError(StatusCode::INTERNAL_SERVER_ERROR, e.into()))?
        .json::<GoogleUserInfo>()
        .await
        .map_err(|e| AppError::HttpError(StatusCode::INTERNAL_SERVER_ERROR, e.into()))?;

    if !user_info.email.ends_with("@iitmandi.ac.in") && !user_info.email.ends_with("@students.iitmandi.ac.in") {
        // Redirect to dedicated error page for non-institute emails
        let frontend_url = query.state.as_ref()
            .and_then(|s| urlencoding::decode(s).ok())
            .map(|s| s.to_string())
            .unwrap_or_else(|| env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:4173".to_string()));
        let error_msg = urlencoding::encode("Only IIT Mandi email addresses are allowed");
        let error_details = urlencoding::encode("Please use your @iitmandi.ac.in or @students.iitmandi.ac.in email address");
        return Ok(Redirect::to(&format!("{}/auth-error?error={}&details={}", frontend_url, error_msg, error_details)));
    }

    let user = match sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE google_id = $1"
    )
    .bind(&user_info.sub)
    .fetch_optional(&pool)
    .await?
    {
        Some(mut existing_user) => {
            sqlx::query("UPDATE users SET last_login_at = NOW() WHERE id = $1")
                .bind(existing_user.id)
                .execute(&pool)
                .await?;
            existing_user.last_login_at = Some(Utc::now());
            existing_user
        }
        None => {
            let default_role = if user_info.email.ends_with("@students.iitmandi.ac.in") {
                UserRole::Student
            } else {
                UserRole::Faculty
            };

            sqlx::query_as::<_, User>(
                r#"
                INSERT INTO users (email, google_id, role, status, first_name, last_name, profile_picture, last_login_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, NOW())
                RETURNING *
                "#
            )
            .bind(&user_info.email)
            .bind(&user_info.sub)
            .bind(default_role)
            .bind(UserStatus::Active)
            .bind(&user_info.given_name)
            .bind(&user_info.family_name)
            .bind(&user_info.picture)
            .fetch_one(&pool)
            .await?
        }
    };

    session.insert(SESSION_USER_ID_KEY, user.id).await
        .map_err(|e| AppError::Internal(e.into()))?;

    sqlx::query(
        "INSERT INTO audit_logs (user_id, action, metadata) VALUES ($1, $2, $3)"
    )
    .bind(user.id)
    .bind("login")
    .bind(serde_json::json!({"method": "google"}))
    .execute(&pool)
    .await?;

    // Use state parameter to redirect to the correct frontend origin
    let frontend_url = query.state.as_ref()
        .and_then(|s| urlencoding::decode(s).ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:4173".to_string()));
    Ok(Redirect::to(&format!("{}/dashboard", frontend_url)))
}

pub async fn logout(
    session: TowerSession,
    State(pool): State<PgPool>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    if let Some(user_id) = session.get::<Uuid>(SESSION_USER_ID_KEY).await
        .map_err(|e| AppError::Internal(e.into()))? 
    {
        sqlx::query(
            "INSERT INTO audit_logs (user_id, action) VALUES ($1, $2)"
        )
        .bind(user_id)
        .bind("logout")
        .execute(&pool)
        .await?;
    }

    session.delete().await
        .map_err(|e| AppError::Internal(e.into()))?;

    Ok(Json(ApiResponse {
        success: true,
        data: None,
        message: Some("Logged out successfully".to_string()),
    }))
}

pub async fn get_current_user(
    session: TowerSession,
    State(pool): State<PgPool>,
) -> Result<Json<ApiResponse<UserResponse>>, AppError> {
    let user_id = session.get::<Uuid>(SESSION_USER_ID_KEY).await
        .map_err(|e| AppError::Internal(e.into()))?
        .ok_or_else(|| AppError::HttpError(StatusCode::UNAUTHORIZED, anyhow::anyhow!("Not authenticated")))?;

    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE id = $1"
    )
    .bind(user_id)
    .fetch_one(&pool)
    .await?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(UserResponse::from(user)),
        message: None,
    }))
}
