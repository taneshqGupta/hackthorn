#![allow(dead_code)]

use crate::error::AppError;
use crate::structs::*;
use axum::{
    extract::{Path, Query, State},
    response::Json,
};
use serde::Serialize;
use sqlx::PgPool;
use tower_sessions::Session;
use uuid::Uuid;

// Helper function to get current user from session
async fn get_session_user(session: &Session, pool: &PgPool) -> Result<User, AppError> {
    let user_id: Uuid = session
        .get("user_id")
        .await?
        .ok_or(AppError::Unauthorized)?;

    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_one(pool)
        .await?;

    Ok(user)
}

// Helper function to check if user is admin
fn require_admin(user: &User) -> Result<(), AppError> {
    if user.role != UserRole::Admin {
        return Err(AppError::Forbidden);
    }
    Ok(())
}

// ============================================================================
// GET ALL USERS (Admin only)
// ============================================================================
pub async fn get_all_users(
    State(pool): State<PgPool>,
    session: Session,
    Query(filters): Query<UserFilters>,
) -> Result<Json<ApiResponse<Vec<UserListResponse>>>, AppError> {
    let user = get_session_user(&session, &pool).await?;
    require_admin(&user)?;

    let page = filters.page.unwrap_or(1).max(1);
    let limit = filters.limit.unwrap_or(50).clamp(1, 100);
    let offset = (page - 1) * limit;

    let mut query = String::from("SELECT * FROM users WHERE 1=1");

    // Apply filters
    if let Some(role) = &filters.role {
        query.push_str(&format!(" AND role = '{:?}'", role).to_lowercase());
    }
    if let Some(status) = &filters.status {
        query.push_str(&format!(" AND status = '{:?}'", status).to_lowercase());
    }
    if let Some(ref search) = filters.search {
        query.push_str(&format!(
            " AND (first_name ILIKE '%{}%' OR last_name ILIKE '%{}%' OR email ILIKE '%{}%')",
            search, search, search
        ));
    }

    query.push_str(" ORDER BY created_at DESC");
    query.push_str(&format!(" LIMIT {} OFFSET {}", limit, offset));

    let users = sqlx::query_as::<_, User>(&query)
        .fetch_all(&pool)
        .await?;

    let responses: Vec<UserListResponse> = users.into_iter().map(UserListResponse::from).collect();

    Ok(Json(ApiResponse {
        success: true,
        data: Some(responses),
        message: None,
    }))
}

// ============================================================================
// GET USER BY ID (Admin only)
// ============================================================================
pub async fn get_user_by_id(
    State(pool): State<PgPool>,
    session: Session,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<UserListResponse>>, AppError> {
    let current_user = get_session_user(&session, &pool).await?;
    require_admin(&current_user)?;

    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(UserListResponse::from(user)),
        message: None,
    }))
}

// ============================================================================
// UPDATE USER ROLE (Admin only)
// ============================================================================
pub async fn update_user_role(
    State(pool): State<PgPool>,
    session: Session,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUserRoleRequest>,
) -> Result<Json<ApiResponse<UserListResponse>>, AppError> {
    let admin_user = get_session_user(&session, &pool).await?;
    require_admin(&admin_user)?;

    // Prevent admin from changing their own role (safety measure)
    if admin_user.id == id {
        return Err(AppError::BadRequest(
            "Cannot change your own role".to_string(),
        ));
    }

    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await?
        .ok_or(AppError::NotFound)?;

    sqlx::query("UPDATE users SET role = $1 WHERE id = $2")
        .bind(&payload.role)
        .bind(id)
        .execute(&pool)
        .await?;

    // Log the action
    sqlx::query(
        "INSERT INTO audit_logs (user_id, action, metadata) VALUES ($1, $2, $3)",
    )
    .bind(admin_user.id)
    .bind("UPDATE_USER_ROLE")
    .bind(serde_json::json!({
        "target_user_id": id,
        "old_role": user.role,
        "new_role": payload.role,
    }))
    .execute(&pool)
    .await?;

    let updated_user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(&pool)
        .await?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(UserListResponse::from(updated_user)),
        message: Some("User role updated successfully".to_string()),
    }))
}

// ============================================================================
// UPDATE CURRENT USER ROLE (For dev/testing - allows self role change)
// ============================================================================
pub async fn update_own_role(
    State(pool): State<PgPool>,
    session: Session,
    Json(payload): Json<UpdateUserRoleRequest>,
) -> Result<Json<ApiResponse<UserResponse>>, AppError> {
    let user = get_session_user(&session, &pool).await?;

    sqlx::query("UPDATE users SET role = $1 WHERE id = $2")
        .bind(&payload.role)
        .bind(user.id)
        .execute(&pool)
        .await?;

    // Log the action
    sqlx::query(
        "INSERT INTO audit_logs (user_id, action, metadata) VALUES ($1, $2, $3)",
    )
    .bind(user.id)
    .bind("UPDATE_OWN_ROLE")
    .bind(serde_json::json!({
        "old_role": user.role,
        "new_role": payload.role,
    }))
    .execute(&pool)
    .await?;

    let updated_user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(user.id)
        .fetch_one(&pool)
        .await?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(UserResponse::from(updated_user)),
        message: Some("Your role has been updated".to_string()),
    }))
}

// ============================================================================
// UPDATE USER STATUS (Admin only)
// ============================================================================
pub async fn update_user_status(
    State(pool): State<PgPool>,
    session: Session,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUserStatusRequest>,
) -> Result<Json<ApiResponse<UserListResponse>>, AppError> {
    let admin_user = get_session_user(&session, &pool).await?;
    require_admin(&admin_user)?;

    // Prevent admin from changing their own status
    if admin_user.id == id {
        return Err(AppError::BadRequest(
            "Cannot change your own status".to_string(),
        ));
    }

    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await?
        .ok_or(AppError::NotFound)?;

    sqlx::query("UPDATE users SET status = $1 WHERE id = $2")
        .bind(&payload.status)
        .bind(id)
        .execute(&pool)
        .await?;

    // Log the action
    sqlx::query(
        "INSERT INTO audit_logs (user_id, action, metadata) VALUES ($1, $2, $3)",
    )
    .bind(admin_user.id)
    .bind("UPDATE_USER_STATUS")
    .bind(serde_json::json!({
        "target_user_id": id,
        "old_status": user.status,
        "new_status": payload.status,
    }))
    .execute(&pool)
    .await?;

    let updated_user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(&pool)
        .await?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(UserListResponse::from(updated_user)),
        message: Some("User status updated successfully".to_string()),
    }))
}

// ============================================================================
// GET AUDIT LOGS (Admin only)
// ============================================================================
pub async fn get_audit_logs(
    State(pool): State<PgPool>,
    session: Session,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<ApiResponse<Vec<AuditLogResponse>>>, AppError> {
    let user = get_session_user(&session, &pool).await?;
    require_admin(&user)?;

    let limit: i64 = params
        .get("limit")
        .and_then(|l| l.parse().ok())
        .unwrap_or(100)
        .clamp(1, 500);

    let offset: i64 = params
        .get("offset")
        .and_then(|o| o.parse().ok())
        .unwrap_or(0);

    let logs = sqlx::query_as::<_, AuditLog>(
        "SELECT * FROM audit_logs ORDER BY created_at DESC LIMIT $1 OFFSET $2",
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(&pool)
    .await?;

    let mut responses = Vec::new();
    for log in logs {
        let log_user = if let Some(user_id) = log.user_id {
            sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
                .bind(user_id)
                .fetch_optional(&pool)
                .await?
                .map(UserResponse::from)
        } else {
            None
        };

        responses.push(AuditLogResponse {
            id: log.id,
            user: log_user,
            action: log.action,
            metadata: log.metadata,
            created_at: log.created_at,
        });
    }

    Ok(Json(ApiResponse {
        success: true,
        data: Some(responses),
        message: None,
    }))
}

// ============================================================================
// GET SYSTEM STATS (Admin only)
// ============================================================================
#[derive(Debug, Serialize)]
pub struct SystemStats {
    pub total_users: i64,
    pub active_users: i64,
    pub total_grievances: i64,
    pub pending_grievances: i64,
    pub resolved_grievances: i64,
    pub users_by_role: serde_json::Value,
}

pub async fn get_system_stats(
    State(pool): State<PgPool>,
    session: Session,
) -> Result<Json<ApiResponse<SystemStats>>, AppError> {
    let user = get_session_user(&session, &pool).await?;
    require_admin(&user)?;

    let total_users: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(&pool)
        .await?;

    let active_users: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE status = 'active'")
            .fetch_one(&pool)
            .await?;

    let total_grievances: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM grievances")
        .fetch_optional(&pool)
        .await?
        .unwrap_or(0);

    let pending_grievances: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM grievances WHERE status IN ('submitted', 'under_review', 'in_progress')",
    )
    .fetch_optional(&pool)
    .await?
    .unwrap_or(0);

    let resolved_grievances: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM grievances WHERE status = 'resolved'")
            .fetch_optional(&pool)
            .await?
            .unwrap_or(0);

    // Get user counts by role
    #[derive(sqlx::FromRow)]
    struct RoleCount {
        role: UserRole,
        count: i64,
    }

    let role_counts = sqlx::query_as::<_, RoleCount>("SELECT role, COUNT(*) as count FROM users GROUP BY role")
        .fetch_all(&pool)
        .await?;

    let users_by_role = serde_json::json!(
        role_counts.iter().map(|rc| (format!("{:?}", rc.role).to_lowercase(), rc.count)).collect::<std::collections::HashMap<_, _>>()
    );

    Ok(Json(ApiResponse {
        success: true,
        data: Some(SystemStats {
            total_users,
            active_users,
            total_grievances,
            pending_grievances,
            resolved_grievances,
            users_by_role,
        }),
        message: None,
    }))
}

// ============================================================================
// SEED DUMMY USERS (Dev only)
// ============================================================================
pub async fn seed_dummy_users(
    State(pool): State<PgPool>,
) -> Result<Json<ApiResponse<String>>, AppError> {
    let dummy_users = vec![
        ("Dr. Strange", "strange@iitmandi.ac.in", UserRole::Faculty),
        ("Tony Stark", "stark@iitmandi.ac.in", UserRole::Authority),
        ("Steve Rogers", "rogers@students.iitmandi.ac.in", UserRole::Student),
    ];

    for (name, email, role) in dummy_users {
        // Check if exists
        let exists = sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)")
            .bind(email)
            .fetch_one(&pool)
            .await?;

        if !exists {
            let parts: Vec<&str> = name.split_whitespace().collect();
            let first_name = parts[0];
            let last_name = parts.get(1).unwrap_or(&"");

            sqlx::query(
                r#"
                INSERT INTO users (email, google_id, role, status, first_name, last_name, profile_picture)
                VALUES ($1, $2, $3, 'active', $4, $5, '')
                "#
            )
            .bind(email)
            .bind(Uuid::new_v4().to_string()) // Random dummy google_id
            .bind(role)
            .bind(first_name)
            .bind(last_name)
            .execute(&pool)
            .await?;
        }
    }

    Ok(Json(ApiResponse {
        success: true,
        data: Some("Dummy users created. You can now assign them tasks.".to_string()),
        message: None,
    }))
}