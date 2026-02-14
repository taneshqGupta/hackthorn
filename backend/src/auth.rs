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
    tracing::info!("CALLBACK_START: Received callback from Google");
    tracing::info!("CALLBACK_STATE: state={:?}", query.state);
    
    let client_id = env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set");
    let client_secret = env::var("GOOGLE_CLIENT_SECRET").expect("GOOGLE_CLIENT_SECRET must be set");
    let redirect_uri = env::var("GOOGLE_REDIRECT_URL").expect("GOOGLE_REDIRECT_URL must be set");

    tracing::info!("TOKEN_EXCHANGE: Starting token exchange with Google");
    let client = reqwest::Client::new();
    
    let token_response = client
        .post("https://oauth2.googleapis.com/token")
        .json(&GoogleTokenRequest {
            code: query.code.clone(),
            client_id,
            client_secret,
            redirect_uri,
            grant_type: "authorization_code".to_string(),
        })
        .send()
        .await
        .map_err(|e| {
            tracing::error!("TOKEN_EXCHANGE_ERROR: Failed to send token request: {}", e);
            AppError::HttpError(StatusCode::INTERNAL_SERVER_ERROR, e.into())
        })?
        .json::<GoogleTokenResponse>()
        .await
        .map_err(|e| {
            tracing::error!("TOKEN_PARSE_ERROR: Failed to parse token response: {}", e);
            AppError::HttpError(StatusCode::INTERNAL_SERVER_ERROR, e.into())
        })?;

    tracing::info!("TOKEN_EXCHANGE: Success");
    tracing::info!("USER_INFO: Fetching user info from Google");
    
    let user_info_response = client
        .get("https://www.googleapis.com/oauth2/v3/userinfo")
        .bearer_auth(&token_response.access_token)
        .send()
        .await
        .map_err(|e| {
            tracing::error!("USER_INFO_ERROR: Failed to fetch user info: {}", e);
            AppError::HttpError(StatusCode::INTERNAL_SERVER_ERROR, e.into())
        })?;
    
    let status = user_info_response.status();
    tracing::info!("USER_INFO: Response status: {}", status);
    
    let response_text = user_info_response.text().await
        .map_err(|e| {
            tracing::error!("USER_INFO_TEXT_ERROR: Failed to read response text: {}", e);
            AppError::HttpError(StatusCode::INTERNAL_SERVER_ERROR, e.into())
        })?;
    
    tracing::info!("USER_INFO: Raw response body: {}", response_text);
    
    let user_info: GoogleUserInfo = serde_json::from_str(&response_text)
        .map_err(|e| {
            tracing::error!("USER_INFO_PARSE_ERROR: Failed to parse user info JSON: {}", e);
            tracing::error!("USER_INFO_PARSE_ERROR: Response was: {}", response_text);
            AppError::HttpError(StatusCode::INTERNAL_SERVER_ERROR, e.into())
        })?;

    tracing::info!("USER_INFO: Received email={}", user_info.email);
    tracing::info!("EMAIL_CHECK: Checking if email domain is allowed");
    
    let ends_with_iitmandi = user_info.email.ends_with("@iitmandi.ac.in");
    let ends_with_students = user_info.email.ends_with("@students.iitmandi.ac.in");
    tracing::info!("EMAIL_CHECK: ends_with @iitmandi.ac.in = {}", ends_with_iitmandi);
    tracing::info!("EMAIL_CHECK: ends_with @students.iitmandi.ac.in = {}", ends_with_students);

    if !ends_with_iitmandi && !ends_with_students {
        tracing::warn!("========================================");
        tracing::warn!("EMAIL_REJECTED: Non-institute email detected");
        tracing::warn!("EMAIL_REJECTED: Email = {}", user_info.email);
        tracing::warn!("EMAIL_REJECTED: Starting redirect to auth-error page");
        tracing::warn!("========================================");
        
        let state_value = query.state.as_ref();
        tracing::info!("STATE_DECODE: state parameter = {:?}", state_value);
        
        let frontend_url = state_value
            .and_then(|s| {
                tracing::info!("STATE_DECODE: Attempting to decode state: {}", s);
                let decoded = urlencoding::decode(s);
                tracing::info!("STATE_DECODE: Decode result = {:?}", decoded);
                decoded.ok()
            })
            .map(|s| {
                let url = s.to_string();
                tracing::info!("STATE_DECODE: Decoded frontend URL = {}", url);
                url
            })
            .unwrap_or_else(|| {
                let fallback = env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:4173".to_string());
                tracing::info!("STATE_DECODE: No state found, using fallback = {}", fallback);
                fallback
            });
        
        tracing::info!("REDIRECT_BUILD: Frontend URL = {}", frontend_url);
        
        let error_msg = urlencoding::encode("Only IIT Mandi email addresses are allowed");
        let error_details = urlencoding::encode("Please use your @iitmandi.ac.in or @students.iitmandi.ac.in email address");
        
        tracing::info!("REDIRECT_BUILD: error_msg (encoded) = {}", error_msg);
        tracing::info!("REDIRECT_BUILD: error_details (encoded) = {}", error_details);
        
        let redirect_url = format!("{}/auth-error?error={}&details={}", frontend_url, error_msg, error_details);
        
        tracing::warn!("========================================");
        tracing::warn!("REDIRECT_FINAL: Redirecting to: {}", redirect_url);
        tracing::warn!("REDIRECT_FINAL: This should be the auth-error page");
        tracing::warn!("========================================");
        tracing::info!("CALLBACK_END: Rejected non-institute email");
        return Ok(Redirect::to(&redirect_url));
    }
    
    tracing::info!("EMAIL_CHECK: Email domain approved");
    tracing::info!("DB_QUERY: Looking up user by google_id");

    let user = match sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE google_id = $1"
    )
    .bind(&user_info.sub)
    .fetch_optional(&pool)
    .await?
    {
        Some(mut existing_user) => {
            tracing::info!("DB_QUERY: Found existing user id={}", existing_user.id);
            sqlx::query("UPDATE users SET last_login_at = NOW() WHERE id = $1")
                .bind(existing_user.id)
                .execute(&pool)
                .await?;
            existing_user.last_login_at = Some(Utc::now());
            tracing::info!("DB_UPDATE: Updated last_login_at for user id={}", existing_user.id);
            existing_user
        }
        None => {
            tracing::info!("DB_QUERY: No existing user found, creating new user");
            let default_role = if user_info.email.ends_with("@students.iitmandi.ac.in") {
                UserRole::Student
            } else {
                UserRole::Faculty
            };
            tracing::info!("USER_CREATE: Role assigned={:?}", default_role);

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
            .bind(&user_info.family_name.as_deref().unwrap_or(""))
            .bind(&user_info.picture)
            .fetch_one(&pool)
            .await?
        }
    };

    tracing::info!("SESSION: Inserting user_id={} into session", user.id);
    session.insert(SESSION_USER_ID_KEY, user.id).await
        .map_err(|e| {
            tracing::error!("SESSION_ERROR: Failed to insert into session: {}", e);
            AppError::Internal(e.into())
        })?;
    tracing::info!("SESSION: Successfully stored user_id in session");

    tracing::info!("AUDIT_LOG: Recording login event");
    sqlx::query(
        "INSERT INTO audit_logs (user_id, action, metadata) VALUES ($1, $2, $3)"
    )
    .bind(user.id)
    .bind("login")
    .bind(serde_json::json!({"method": "google"}))
    .execute(&pool)
    .await?;
    tracing::info!("AUDIT_LOG: Login event recorded");

    tracing::info!("REDIRECT: Preparing redirect to frontend");
    // Use state parameter to redirect to the correct frontend origin
    let frontend_url = query.state.as_ref()
        .and_then(|s| {
            tracing::info!("REDIRECT: Decoding state for redirect: {}", s);
            urlencoding::decode(s).ok()
        })
        .map(|s| s.to_string())
        .unwrap_or_else(|| {
            let fallback = env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:4173".to_string());
            tracing::info!("REDIRECT: Using fallback URL: {}", fallback);
            fallback
        });
    
    let redirect_url = format!("{}/dashboard", frontend_url);
    tracing::info!("REDIRECT: Redirecting to {}", redirect_url);
    tracing::info!("CALLBACK_END: Success");
    Ok(Redirect::to(&redirect_url))
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
