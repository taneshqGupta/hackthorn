use crate::cloudinary::{CloudinaryConfig, CloudinaryService};
use crate::error::AppError;
use crate::structs::*;
use axum::{
    extract::{Path, Query, State, Multipart},
    http::StatusCode,
    response::Json,
};
use sqlx::PgPool;
use tower_sessions::Session;
use uuid::Uuid;
use base64::prelude::*;

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

// Helper function to check if user can view grievance details
fn can_view_grievance(user: &User, grievance: &Grievance) -> bool {
    match user.role {
        UserRole::Admin | UserRole::Authority => true,
        UserRole::Faculty => grievance.assigned_to == Some(user.id),
        UserRole::Student => {
            grievance.submitted_by == Some(user.id) || !grievance.is_anonymous
        }
    }
}

// Helper function to check if user can modify grievance
fn can_modify_grievance(user: &User, grievance: &Grievance) -> bool {
    match user.role {
        UserRole::Admin => true,
        UserRole::Authority => grievance.assigned_to == Some(user.id) || grievance.assigned_to.is_none(),
        UserRole::Student => grievance.submitted_by == Some(user.id),
        _ => false,
    }
}

// Generate anonymous identifier
fn generate_anonymous_identifier() -> String {
    format!("ANON-{}", Uuid::new_v4().to_string()[..8].to_uppercase())
}

// ============================================================================
// CREATE GRIEVANCE
// ============================================================================
pub async fn create_grievance(
    State(pool): State<PgPool>,
    session: Session,
    Json(payload): Json<CreateGrievanceRequest>,
) -> Result<Json<ApiResponse<GrievanceResponse>>, AppError> {
    let user = get_session_user(&session, &pool).await?;

    // Validate user role - students and faculty can submit grievances
    if !matches!(user.role, UserRole::Student | UserRole::Faculty) {
        return Err(AppError::Forbidden);
    }

    let submitted_by = if payload.is_anonymous { None } else { Some(user.id) };
    let anonymous_identifier = if payload.is_anonymous {
        Some(generate_anonymous_identifier())
    } else {
        None
    };

    let grievance = sqlx::query_as::<_, Grievance>(
        r#"
        INSERT INTO grievances (
            submitted_by, is_anonymous, anonymous_identifier,
            title, description, category, priority,
            location_type, location_details
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING *
        "#,
    )
    .bind(submitted_by)
    .bind(payload.is_anonymous)
    .bind(anonymous_identifier)
    .bind(&payload.title)
    .bind(&payload.description)
    .bind(&payload.category)
    .bind(&payload.priority)
    .bind(&payload.location_type)
    .bind(&payload.location_details)
    .fetch_one(&pool)
    .await?;

    // Log the creation
    sqlx::query(
        "INSERT INTO audit_logs (user_id, action, metadata) VALUES ($1, $2, $3)"
    )
    .bind(user.id)
    .bind("CREATE_GRIEVANCE")
    .bind(serde_json::json!({
        "grievance_id": grievance.id,
        "category": grievance.category,
        "priority": grievance.priority,
    }))
    .execute(&pool)
    .await?;

    let response = GrievanceResponse {
        id: grievance.id,
        submitter: None,
        is_anonymous: grievance.is_anonymous,
        title: grievance.title,
        description: grievance.description,
        category: grievance.category,
        priority: grievance.priority,
        status: grievance.status,
        location_type: grievance.location_type,
        location_details: grievance.location_details,
        photo_urls: grievance.photo_urls.unwrap_or_default(),
        assigned_to: None,
        assigned_department: grievance.assigned_department,
        resolution_notes: grievance.resolution_notes,
        resolved_at: grievance.resolved_at,
        view_count: grievance.view_count,
        upvote_count: grievance.upvote_count,
        user_has_upvoted: false,
        created_at: grievance.created_at,
        updated_at: grievance.updated_at,
    };

    Ok(Json(ApiResponse {
        success: true,
        data: Some(response),
        message: Some("Grievance created successfully".to_string()),
    }))
}

// ============================================================================
// GET ALL GRIEVANCES (WITH FILTERS)
// ============================================================================
pub async fn get_grievances(
    State(pool): State<PgPool>,
    session: Session,
    Query(filters): Query<GrievanceFilters>,
) -> Result<Json<ApiResponse<Vec<GrievanceResponse>>>, AppError> {
    let user = get_session_user(&session, &pool).await?;

    let page = filters.page.unwrap_or(1).max(1);
    let limit = filters.limit.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * limit;

    // Build dynamic query based on filters
    // Public feed: Everyone sees all grievances (like Reddit)
    // RBAC is enforced on ACTIONS (update, assign, delete), not viewing
    let mut query = String::from(
        r#"
        SELECT g.* FROM grievances g
        WHERE 1=1
        "#,
    );

    // Apply filters
    if let Some(status) = &filters.status {
        query.push_str(&format!(" AND g.status = '{:?}'", status).to_lowercase());
    }
    if let Some(category) = &filters.category {
        query.push_str(&format!(" AND g.category = '{:?}'", category).to_lowercase());
    }
    if let Some(priority) = &filters.priority {
        query.push_str(&format!(" AND g.priority = '{:?}'", priority).to_lowercase());
    }
    if let Some(assigned_to) = filters.assigned_to {
        query.push_str(&format!(" AND g.assigned_to = '{}'", assigned_to));
    }
    if let Some(ref dept) = filters.assigned_department {
        query.push_str(&format!(" AND g.assigned_department = '{}'", dept));
    }
    if let Some(ref search) = filters.search {
        query.push_str(&format!(
            " AND (g.title ILIKE '%{}%' OR g.description ILIKE '%{}%')",
            search, search
        ));
    }

    query.push_str(" ORDER BY g.created_at DESC");
    query.push_str(&format!(" LIMIT {} OFFSET {}", limit, offset));

    let grievances = sqlx::query_as::<_, Grievance>(&query)
        .fetch_all(&pool)
        .await?;

    // Fetch related data for responses
    let mut responses = Vec::new();
    for grievance in grievances {
        let submitter = if let Some(submitter_id) = grievance.submitted_by {
            sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
                .bind(submitter_id)
                .fetch_optional(&pool)
                .await?
                .map(UserResponse::from)
        } else {
            None
        };

        let assigned_user = if let Some(assigned_id) = grievance.assigned_to {
            sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
                .bind(assigned_id)
                .fetch_optional(&pool)
                .await?
                .map(UserResponse::from)
        } else {
            None
        };

        // Check if user has upvoted
        let user_has_upvoted = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM grievance_upvotes WHERE grievance_id = $1 AND user_id = $2)"
        )
        .bind(grievance.id)
        .bind(user.id)
        .fetch_one(&pool)
        .await?;

        responses.push(GrievanceResponse {
            id: grievance.id,
            submitter,
            is_anonymous: grievance.is_anonymous,
            title: grievance.title,
            description: grievance.description,
            category: grievance.category,
            priority: grievance.priority,
            status: grievance.status,
            location_type: grievance.location_type,
            location_details: grievance.location_details,
            photo_urls: grievance.photo_urls.unwrap_or_default(),
            assigned_to: assigned_user,
            assigned_department: grievance.assigned_department,
            resolution_notes: grievance.resolution_notes,
            resolved_at: grievance.resolved_at,
            view_count: grievance.view_count,
            upvote_count: grievance.upvote_count,
            user_has_upvoted,
            created_at: grievance.created_at,
            updated_at: grievance.updated_at,
        });
    }

    Ok(Json(ApiResponse {
        success: true,
        data: Some(responses),
        message: None,
    }))
}

// ============================================================================
// GET SINGLE GRIEVANCE BY ID
// ============================================================================
pub async fn get_grievance_by_id(
    State(pool): State<PgPool>,
    session: Session,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<GrievanceResponse>>, AppError> {
    let user = get_session_user(&session, &pool).await?;

    let grievance = sqlx::query_as::<_, Grievance>("SELECT * FROM grievances WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await?
        .ok_or(AppError::NotFound)?;

    // Check if user can view this grievance
    if !can_view_grievance(&user, &grievance) {
        return Err(AppError::Forbidden);
    }

    // Increment view count
    sqlx::query("UPDATE grievances SET view_count = view_count + 1 WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await?;

    // Fetch related data
    let submitter = if let Some(submitter_id) = grievance.submitted_by {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(submitter_id)
            .fetch_optional(&pool)
            .await?
            .map(UserResponse::from)
    } else {
        None
    };

    let assigned_user = if let Some(assigned_id) = grievance.assigned_to {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(assigned_id)
            .fetch_optional(&pool)
            .await?
            .map(UserResponse::from)
    } else {
        None
    };

    let user_has_upvoted = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM grievance_upvotes WHERE grievance_id = $1 AND user_id = $2)",
    )
    .bind(id)
    .bind(user.id)
    .fetch_one(&pool)
    .await?;

    let response = GrievanceResponse {
        id: grievance.id,
        submitter,
        is_anonymous: grievance.is_anonymous,
        title: grievance.title.clone(),
        description: grievance.description.clone(),
        category: grievance.category.clone(),
        priority: grievance.priority.clone(),
        status: grievance.status.clone(),
        location_type: grievance.location_type.clone(),
        location_details: grievance.location_details.clone(),
        photo_urls: grievance.photo_urls.clone().unwrap_or_default(),
        assigned_to: assigned_user,
        assigned_department: grievance.assigned_department.clone(),
        resolution_notes: grievance.resolution_notes.clone(),
        resolved_at: grievance.resolved_at,
        view_count: grievance.view_count + 1, // Include the increment
        upvote_count: grievance.upvote_count,
        user_has_upvoted,
        created_at: grievance.created_at,
        updated_at: grievance.updated_at,
    };

    Ok(Json(ApiResponse {
        success: true,
        data: Some(response),
        message: None,
    }))
}

// ============================================================================
// UPDATE GRIEVANCE STATUS (Authority/Admin only)
// ============================================================================
pub async fn update_grievance_status(
    State(pool): State<PgPool>,
    session: Session,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateGrievanceStatusRequest>,
) -> Result<Json<ApiResponse<GrievanceResponse>>, AppError> {
    let user = get_session_user(&session, &pool).await?;

    // Only Authority and Admin can update status
    if !matches!(user.role, UserRole::Authority | UserRole::Admin) {
        return Err(AppError::Forbidden);
    }

    let grievance = sqlx::query_as::<_, Grievance>("SELECT * FROM grievances WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await?
        .ok_or(AppError::NotFound)?;

    // Update status
    sqlx::query("UPDATE grievances SET status = $1 WHERE id = $2")
        .bind(&payload.status)
        .bind(id)
        .execute(&pool)
        .await?;

    // Log status change with remarks
    sqlx::query(
        r#"
        INSERT INTO grievance_status_history 
        (grievance_id, old_status, new_status, remarks, updated_by, updated_by_role)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
    )
    .bind(id)
    .bind(&grievance.status)
    .bind(&payload.status)
    .bind(&payload.remarks)
    .bind(user.id)
    .bind(&user.role)
    .execute(&pool)
    .await?;

    // Get updated grievance
    let updated = sqlx::query_as::<_, Grievance>("SELECT * FROM grievances WHERE id = $1")
        .bind(id)
        .fetch_one(&pool)
        .await?;

    let response = GrievanceResponse {
        id: updated.id,
        submitter: None,
        is_anonymous: updated.is_anonymous,
        title: updated.title,
        description: updated.description,
        category: updated.category,
        priority: updated.priority,
        status: updated.status,
        location_type: updated.location_type,
        location_details: updated.location_details,
        photo_urls: updated.photo_urls.unwrap_or_default(),
        assigned_to: None,
        assigned_department: updated.assigned_department,
        resolution_notes: updated.resolution_notes,
        resolved_at: updated.resolved_at,
        view_count: updated.view_count,
        upvote_count: updated.upvote_count,
        user_has_upvoted: false,
        created_at: updated.created_at,
        updated_at: updated.updated_at,
    };

    Ok(Json(ApiResponse {
        success: true,
        data: Some(response),
        message: Some("Status updated successfully".to_string()),
    }))
}

// ============================================================================
// ASSIGN GRIEVANCE (Authority/Admin only)
// ============================================================================
pub async fn assign_grievance(
    State(pool): State<PgPool>,
    session: Session,
    Path(id): Path<Uuid>,
    Json(payload): Json<AssignGrievanceRequest>,
) -> Result<Json<ApiResponse<GrievanceResponse>>, AppError> {
    let user = get_session_user(&session, &pool).await?;

    if !matches!(user.role, UserRole::Authority | UserRole::Admin) {
        return Err(AppError::Forbidden);
    }

    // Verify assigned user exists and has appropriate role
    if let Some(assigned_id) = payload.assigned_to {
        let assigned_user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(assigned_id)
            .fetch_optional(&pool)
            .await?
            .ok_or(AppError::NotFound)?;

        if !matches!(
            assigned_user.role,
            UserRole::Authority | UserRole::Admin | UserRole::Faculty
        ) {
            return Err(AppError::BadRequest(
                "Can only assign to Authority, Admin, or Faculty".to_string(),
            ));
        }
    }

    sqlx::query(
        "UPDATE grievances SET assigned_to = $1, assigned_department = $2 WHERE id = $3",
    )
    .bind(payload.assigned_to)
    .bind(&payload.assigned_department)
    .bind(id)
    .execute(&pool)
    .await?;

    // Log the assignment
    sqlx::query("INSERT INTO audit_logs (user_id, action, metadata) VALUES ($1, $2, $3)")
        .bind(user.id)
        .bind("ASSIGN_GRIEVANCE")
        .bind(serde_json::json!({
            "grievance_id": id,
            "assigned_to": payload.assigned_to,
            "assigned_department": payload.assigned_department,
        }))
        .execute(&pool)
        .await?;

    let updated = sqlx::query_as::<_, Grievance>("SELECT * FROM grievances WHERE id = $1")
        .bind(id)
        .fetch_one(&pool)
        .await?;

    let response = GrievanceResponse {
        id: updated.id,
        submitter: None,
        is_anonymous: updated.is_anonymous,
        title: updated.title,
        description: updated.description,
        category: updated.category,
        priority: updated.priority,
        status: updated.status,
        location_type: updated.location_type,
        location_details: updated.location_details,
        photo_urls: updated.photo_urls.unwrap_or_default(),
        assigned_to: None,
        assigned_department: updated.assigned_department,
        resolution_notes: updated.resolution_notes,
        resolved_at: updated.resolved_at,
        view_count: updated.view_count,
        upvote_count: updated.upvote_count,
        user_has_upvoted: false,
        created_at: updated.created_at,
        updated_at: updated.updated_at,
    };

    Ok(Json(ApiResponse {
        success: true,
        data: Some(response),
        message: Some("Grievance assigned successfully".to_string()),
    }))
}

// ============================================================================
// RESOLVE GRIEVANCE (Authority/Admin only)
// ============================================================================
pub async fn resolve_grievance(
    State(pool): State<PgPool>,
    session: Session,
    Path(id): Path<Uuid>,
    Json(payload): Json<ResolveGrievanceRequest>,
) -> Result<Json<ApiResponse<GrievanceResponse>>, AppError> {
    let user = get_session_user(&session, &pool).await?;

    if !matches!(user.role, UserRole::Authority | UserRole::Admin) {
        return Err(AppError::Forbidden);
    }

    sqlx::query(
        r#"
        UPDATE grievances 
        SET status = 'resolved', 
            resolution_notes = $1, 
            resolved_at = NOW(), 
            resolved_by = $2
        WHERE id = $3
        "#,
    )
    .bind(&payload.resolution_notes)
    .bind(user.id)
    .bind(id)
    .execute(&pool)
    .await?;

    let updated = sqlx::query_as::<_, Grievance>("SELECT * FROM grievances WHERE id = $1")
        .bind(id)
        .fetch_one(&pool)
        .await?;

    let response = GrievanceResponse {
        id: updated.id,
        submitter: None,
        is_anonymous: updated.is_anonymous,
        title: updated.title,
        description: updated.description,
        category: updated.category,
        priority: updated.priority,
        status: updated.status,
        location_type: updated.location_type,
        location_details: updated.location_details,
        photo_urls: updated.photo_urls.unwrap_or_default(),
        assigned_to: None,
        assigned_department: updated.assigned_department,
        resolution_notes: updated.resolution_notes,
        resolved_at: updated.resolved_at,
        view_count: updated.view_count,
        upvote_count: updated.upvote_count,
        user_has_upvoted: false,
        created_at: updated.created_at,
        updated_at: updated.updated_at,
    };

    Ok(Json(ApiResponse {
        success: true,
        data: Some(response),
        message: Some("Grievance resolved successfully".to_string()),
    }))
}

// ============================================================================
// UPVOTE/REMOVE UPVOTE GRIEVANCE
// ============================================================================
pub async fn toggle_upvote(
    State(pool): State<PgPool>,
    session: Session,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    let user = get_session_user(&session, &pool).await?;

    // Check if grievance exists
    sqlx::query_as::<_, Grievance>("SELECT * FROM grievances WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await?
        .ok_or(AppError::NotFound)?;

    // Check if user has already upvoted
    let has_upvoted = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM grievance_upvotes WHERE grievance_id = $1 AND user_id = $2)",
    )
    .bind(id)
    .bind(user.id)
    .fetch_one(&pool)
    .await?;

    if has_upvoted {
        // Remove upvote
        sqlx::query("DELETE FROM grievance_upvotes WHERE grievance_id = $1 AND user_id = $2")
            .bind(id)
            .bind(user.id)
            .execute(&pool)
            .await?;

        sqlx::query("UPDATE grievances SET upvote_count = upvote_count - 1 WHERE id = $1")
            .bind(id)
            .execute(&pool)
            .await?;

        Ok(Json(ApiResponse {
            success: true,
            data: None,
            message: Some("Upvote removed".to_string()),
        }))
    } else {
        // Add upvote
        sqlx::query("INSERT INTO grievance_upvotes (grievance_id, user_id) VALUES ($1, $2)")
            .bind(id)
            .bind(user.id)
            .execute(&pool)
            .await?;

        sqlx::query("UPDATE grievances SET upvote_count = upvote_count + 1 WHERE id = $1")
            .bind(id)
            .execute(&pool)
            .await?;

        Ok(Json(ApiResponse {
            success: true,
            data: None,
            message: Some("Upvoted successfully".to_string()),
        }))
    }
}

// ============================================================================
// GET GRIEVANCE STATUS HISTORY
// ============================================================================
pub async fn get_grievance_history(
    State(pool): State<PgPool>,
    session: Session,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<GrievanceStatusHistoryResponse>>>, AppError> {
    let user = get_session_user(&session, &pool).await?;

    let grievance = sqlx::query_as::<_, Grievance>("SELECT * FROM grievances WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await?
        .ok_or(AppError::NotFound)?;

    if !can_view_grievance(&user, &grievance) {
        return Err(AppError::Forbidden);
    }

    let history = sqlx::query_as::<_, GrievanceStatusHistory>(
        "SELECT * FROM grievance_status_history WHERE grievance_id = $1 ORDER BY created_at DESC",
    )
    .bind(id)
    .fetch_all(&pool)
    .await?;

    let mut responses = Vec::new();
    for entry in history {
        let updated_by_user = if let Some(user_id) = entry.updated_by {
            sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
                .bind(user_id)
                .fetch_optional(&pool)
                .await?
                .map(UserResponse::from)
        } else {
            None
        };

        responses.push(GrievanceStatusHistoryResponse {
            id: entry.id,
            old_status: entry.old_status,
            new_status: entry.new_status,
            remarks: entry.remarks,
            updated_by: updated_by_user,
            created_at: entry.created_at,
        });
    }

    Ok(Json(ApiResponse {
        success: true,
        data: Some(responses),
        message: None,
    }))
}

// ============================================================================
// ADD COMMENT TO GRIEVANCE
// ============================================================================
pub async fn add_comment(
    State(pool): State<PgPool>,
    session: Session,
    Path(id): Path<Uuid>,
    Json(payload): Json<CreateCommentRequest>,
) -> Result<Json<ApiResponse<GrievanceCommentResponse>>, AppError> {
    let user = get_session_user(&session, &pool).await?;

    let grievance = sqlx::query_as::<_, Grievance>("SELECT * FROM grievances WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await?
        .ok_or(AppError::NotFound)?;

    if !can_view_grievance(&user, &grievance) {
        return Err(AppError::Forbidden);
    }

    // Only authorities and admins can add internal notes
    let is_internal = payload.is_internal
        && matches!(user.role, UserRole::Authority | UserRole::Admin);

    let comment = sqlx::query_as::<_, GrievanceComment>(
        r#"
        INSERT INTO grievance_comments (grievance_id, user_id, comment, is_internal)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
    )
    .bind(id)
    .bind(user.id)
    .bind(&payload.comment)
    .bind(is_internal)
    .fetch_one(&pool)
    .await?;

    let response = GrievanceCommentResponse {
        id: comment.id,
        user: UserResponse::from(user),
        comment: comment.comment,
        is_internal: comment.is_internal,
        created_at: comment.created_at,
    };

    Ok(Json(ApiResponse {
        success: true,
        data: Some(response),
        message: Some("Comment added successfully".to_string()),
    }))
}

// ============================================================================
// GET COMMENTS FOR GRIEVANCE
// ============================================================================
pub async fn get_comments(
    State(pool): State<PgPool>,
    session: Session,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<GrievanceCommentResponse>>>, AppError> {
    let user = get_session_user(&session, &pool).await?;

    let grievance = sqlx::query_as::<_, Grievance>("SELECT * FROM grievances WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await?
        .ok_or(AppError::NotFound)?;

    if !can_view_grievance(&user, &grievance) {
        return Err(AppError::Forbidden);
    }

    // Filter internal comments based on user role
    let comments = if matches!(user.role, UserRole::Authority | UserRole::Admin) {
        sqlx::query_as::<_, GrievanceComment>(
            "SELECT * FROM grievance_comments WHERE grievance_id = $1 ORDER BY created_at ASC",
        )
        .bind(id)
        .fetch_all(&pool)
        .await?
    } else {
        sqlx::query_as::<_, GrievanceComment>(
            "SELECT * FROM grievance_comments WHERE grievance_id = $1 AND is_internal = false ORDER BY created_at ASC",
        )
        .bind(id)
        .fetch_all(&pool)
        .await?
    };

    let mut responses = Vec::new();
    for comment in comments {
        let comment_user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(comment.user_id)
            .fetch_one(&pool)
            .await?;

        responses.push(GrievanceCommentResponse {
            id: comment.id,
            user: UserResponse::from(comment_user),
            comment: comment.comment,
            is_internal: comment.is_internal,
            created_at: comment.created_at,
        });
    }

    Ok(Json(ApiResponse {
        success: true,
        data: Some(responses),
        message: None,
    }))
}

// ============================================================================
// GET DEPARTMENTS
// ============================================================================
pub async fn get_departments(
    State(pool): State<PgPool>,
    session: Session,
) -> Result<Json<ApiResponse<Vec<Department>>>, AppError> {
    let _user = get_session_user(&session, &pool).await?;

    let departments = sqlx::query_as::<_, Department>("SELECT * FROM departments ORDER BY name")
        .fetch_all(&pool)
        .await?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(departments),
        message: None,
    }))
}

// ============================================================================
// DELETE GRIEVANCE (Student can delete their own, Admin can delete any)
// ============================================================================
pub async fn delete_grievance(
    State(pool): State<PgPool>,
    session: Session,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), AppError> {
    let user = get_session_user(&session, &pool).await?;

    let grievance = sqlx::query_as::<_, Grievance>("SELECT * FROM grievances WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await?
        .ok_or(AppError::NotFound)?;

    // Students can delete their own grievances, Admins can delete any
    let can_delete = match user.role {
        UserRole::Admin => true,
        UserRole::Student => grievance.submitted_by == Some(user.id),
        _ => false,
    };

    if !can_delete {
        return Err(AppError::Forbidden);
    }

    sqlx::query("DELETE FROM grievances WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse {
            success: true,
            data: None,
            message: Some("Grievance deleted successfully".to_string()),
        }),
    ))
}

// ============================================================================
// UPLOAD PHOTOS FOR GRIEVANCE
// ============================================================================
pub async fn upload_grievance_photos(
    State(pool): State<PgPool>,
    session: Session,
    Path(id): Path<Uuid>,
    mut multipart: Multipart,
) -> Result<Json<ApiResponse<Vec<String>>>, AppError> {
    let user = get_session_user(&session, &pool).await?;

    // Check if grievance exists and user has permission
    let grievance = sqlx::query_as::<_, Grievance>("SELECT * FROM grievances WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await?
        .ok_or(AppError::NotFound)?;

    if !can_modify_grievance(&user, &grievance) {
        return Err(AppError::Forbidden);
    }

    // Initialize Cloudinary service
    let cloudinary_config = CloudinaryConfig::from_env()
        .map_err(|e| AppError::InternalServerError(format!("Cloudinary config error: {}", e)))?;
    let cloudinary = CloudinaryService::new(cloudinary_config);

    let mut uploaded_urls = Vec::new();

    // Process each file in the multipart form
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::BadRequest(format!("Multipart error: {}", e)))?
    {
        let name = field.name().unwrap_or("").to_string();

        if name == "photos" {
            let data = field
                .bytes()
                .await
                .map_err(|e| AppError::BadRequest(format!("Failed to read file: {}", e)))?;

            // Convert bytes to base64
            let base64_data = base64::prelude::BASE64_STANDARD.encode(&data);

            // Upload to Cloudinary with grievance folder
            let public_id = format!("grievances/{}/{}", id, Uuid::new_v4());
            let url = cloudinary
                .upload_image(&base64_data, Some(public_id))
                .await
                .map_err(|e| {
                    AppError::InternalServerError(format!("Cloudinary upload failed: {}", e))
                })?;

            uploaded_urls.push(url);
        }
    }

    if uploaded_urls.is_empty() {
        return Err(AppError::BadRequest(
            "No photos provided for upload".to_string(),
        ));
    }

    // Update grievance with photo URLs
    let mut existing_urls = grievance.photo_urls.unwrap_or_default();
    existing_urls.extend(uploaded_urls.clone());

    sqlx::query("UPDATE grievances SET photo_urls = $1 WHERE id = $2")
        .bind(&existing_urls)
        .bind(id)
        .execute(&pool)
        .await?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(uploaded_urls),
        message: Some("Photos uploaded successfully".to_string()),
    }))
}

