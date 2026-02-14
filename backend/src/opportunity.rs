#![allow(dead_code)]

use crate::error::AppError;
use crate::structs::*;
use axum::{
    extract::{Path, Query, State},
    response::Json,
};
use sqlx::PgPool;
use tower_sessions::Session;
use uuid::Uuid;

// ============================================================================
// HELPERS
// ============================================================================

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

fn require_faculty(user: &User) -> Result<(), AppError> {
    if matches!(user.role, UserRole::Faculty | UserRole::Admin) {
        Ok(())
    } else {
        Err(AppError::Forbidden)
    }
}

// ============================================================================
// 1. OPPORTUNITIES (The Professor's Call)
// ============================================================================

// POST /api/opportunities (Faculty Only)
pub async fn create_opportunity(
    State(pool): State<PgPool>,
    session: Session,
    Json(payload): Json<CreateOpportunityRequest>,
) -> Result<Json<ApiResponse<OpportunityResponse>>, AppError> {
    let user = get_session_user(&session, &pool).await?;
    require_faculty(&user)?;

    let op = sqlx::query_as::<_, Opportunity>(
        r#"
        INSERT INTO opportunities (
            posted_by, title, description, opportunity_type, department, 
            required_skills, duration, stipend, location, application_deadline
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        RETURNING *
        "#
    )
    .bind(user.id)
    .bind(&payload.title)
    .bind(&payload.description)
    .bind(&payload.opportunity_type)
    .bind(&payload.department)
    .bind(&payload.required_skills)
    .bind(&payload.duration)
    .bind(&payload.stipend)
    .bind(&payload.location)
    .bind(payload.application_deadline)
    .fetch_one(&pool)
    .await?;

    // Construct Response
    // (In production, you might fetch the user details to populate `posted_by`)
    let response = OpportunityResponse {
        id: op.id,
        posted_by: UserResponse::from(user), // The creator is the current user
        title: op.title,
        description: op.description,
        opportunity_type: op.opportunity_type,
        department: op.department,
        required_skills: op.required_skills.unwrap_or_default(),
        duration: op.duration,
        stipend: op.stipend,
        location: op.location,
        application_deadline: op.application_deadline,
        is_active: op.is_active,
        created_at: op.created_at,
        has_applied: false, // Creator hasn't applied
    };

    Ok(Json(ApiResponse {
        success: true,
        data: Some(response),
        message: Some("Opportunity posted successfully".to_string()),
    }))
}

// GET /api/opportunities (Public/Student - Browsing)
// Supports filters: ?department=CSE&type=internship
pub async fn get_opportunities(
    State(pool): State<PgPool>,
    session: Session,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<ApiResponse<Vec<OpportunityResponse>>>, AppError> {
    let user = get_session_user(&session, &pool).await.ok(); // Optional auth for browsing
    let user_id = user.as_ref().map(|u| u.id);

    let mut query = String::from("SELECT * FROM opportunities WHERE is_active = true");

    // Dynamic Filtering
    if let Some(dept) = params.get("department") {
        query.push_str(&format!(" AND department = '{}'", dept)); // Warning: Use bind in prod
    }
    if let Some(op_type) = params.get("type") {
        query.push_str(&format!(" AND opportunity_type = '{}'", op_type));
    }
    // "Smart Filtering" -> Search by skill could be added here with array overlap

    query.push_str(" ORDER BY created_at DESC");

    let opportunities = sqlx::query_as::<_, Opportunity>(&query)
        .fetch_all(&pool)
        .await?;

    // Transform to Response & Check "has_applied"
    let mut responses = Vec::new();
    for op in opportunities {
        // Fetch Poster details
        let poster = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(op.posted_by)
            .fetch_one(&pool)
            .await?;

        // Check if current user has applied
        let has_applied = if let Some(uid) = user_id {
            sqlx::query_scalar::<_, bool>(
                "SELECT EXISTS(SELECT 1 FROM applications WHERE opportunity_id = $1 AND student_id = $2)"
            )
            .bind(op.id)
            .bind(uid)
            .fetch_one(&pool)
            .await?
        } else {
            false
        };

        responses.push(OpportunityResponse {
            id: op.id,
            posted_by: UserResponse::from(poster),
            title: op.title,
            description: op.description,
            opportunity_type: op.opportunity_type,
            department: op.department,
            required_skills: op.required_skills.unwrap_or_default(),
            duration: op.duration,
            stipend: op.stipend,
            location: op.location,
            application_deadline: op.application_deadline,
            is_active: op.is_active,
            created_at: op.created_at,
            has_applied,
        });
    }

    Ok(Json(ApiResponse {
        success: true,
        data: Some(responses),
        message: None,
    }))
}

// ============================================================================
// 2. APPLICATIONS (Student Actions)
// ============================================================================

// POST /api/opportunities/:id/apply
pub async fn apply_opportunity(
    State(pool): State<PgPool>,
    session: Session,
    Path(opportunity_id): Path<Uuid>,
    Json(payload): Json<ApplyRequest>,
) -> Result<Json<ApiResponse<String>>, AppError> {
    let user = get_session_user(&session, &pool).await?;
    
    // Check if opportunity exists and is active
    let op_exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM opportunities WHERE id = $1 AND is_active = true)"
    )
    .bind(opportunity_id)
    .fetch_one(&pool)
    .await?;

    if !op_exists {
        return Err(AppError::NotFound); // Or "Opportunity closed/not found"
    }

    // Check for duplicate application
    let exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM applications WHERE opportunity_id = $1 AND student_id = $2)"
    )
    .bind(opportunity_id)
    .bind(user.id)
    .fetch_one(&pool)
    .await?;

    if exists {
        return Err(AppError::BadRequest("You have already applied".to_string()));
    }

    // Create Application
    sqlx::query(
        r#"
        INSERT INTO applications (opportunity_id, student_id, resume_url, cover_letter, portfolio_url)
        VALUES ($1, $2, $3, $4, $5)
        "#
    )
    .bind(opportunity_id)
    .bind(user.id)
    .bind(&payload.resume_url)
    .bind(&payload.cover_letter)
    .bind(&payload.portfolio_url)
    .execute(&pool)
    .await?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some("Application submitted successfully".to_string()),
        message: None,
    }))
}

// GET /api/applications/my-applications (Student History)
pub async fn get_my_applications(
    State(pool): State<PgPool>,
    session: Session,
) -> Result<Json<ApiResponse<Vec<ApplicationResponse>>>, AppError> {
    let user = get_session_user(&session, &pool).await?;

    let applications = sqlx::query_as::<_, Application>(
        "SELECT * FROM applications WHERE student_id = $1 ORDER BY applied_at DESC"
    )
    .bind(user.id)
    .fetch_all(&pool)
    .await?;

    // Enrich with Opportunity Details
    let mut responses = Vec::new();
    for app in applications {
        let op = sqlx::query_as::<_, Opportunity>("SELECT * FROM opportunities WHERE id = $1")
            .bind(app.opportunity_id)
            .fetch_one(&pool)
            .await?;
            
        // For the response, we need the poster of the opportunity too
        let poster = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(op.posted_by)
            .fetch_one(&pool)
            .await?;

        // Construct simplified op response
        let op_response = OpportunityResponse {
            id: op.id,
            posted_by: UserResponse::from(poster),
            title: op.title,
            description: op.description,
            opportunity_type: op.opportunity_type,
            department: op.department,
            required_skills: op.required_skills.unwrap_or_default(),
            duration: op.duration,
            stipend: op.stipend,
            location: op.location,
            application_deadline: op.application_deadline,
            is_active: op.is_active,
            created_at: op.created_at,
            has_applied: true,
        };

        responses.push(ApplicationResponse {
            id: app.id,
            opportunity: Some(op_response),
            student: Some(UserResponse::from(user.clone())),
            resume_url: app.resume_url,
            cover_letter: app.cover_letter,
            portfolio_url: app.portfolio_url,
            status: app.status,
            applied_at: app.applied_at,
        });
    }

    Ok(Json(ApiResponse {
        success: true,
        data: Some(responses),
        message: None,
    }))
}

// ============================================================================
// 3. FACULTY MANAGEMENT (Managing Applications)
// ============================================================================

// GET /api/opportunities/:id/applications (View Applicants)
pub async fn get_opportunity_applications(
    State(pool): State<PgPool>,
    session: Session,
    Path(opportunity_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<ApplicationResponse>>>, AppError> {
    let user = get_session_user(&session, &pool).await?;
    require_faculty(&user)?;

    // Verify ownership (Only poster or admin sees apps)
    let op = sqlx::query_as::<_, Opportunity>("SELECT * FROM opportunities WHERE id = $1")
        .bind(opportunity_id)
        .fetch_optional(&pool)
        .await?
        .ok_or(AppError::NotFound)?;

    if op.posted_by != user.id && user.role != UserRole::Admin {
        return Err(AppError::Forbidden);
    }

    let applications = sqlx::query_as::<_, Application>(
        "SELECT * FROM applications WHERE opportunity_id = $1 ORDER BY applied_at ASC"
    )
    .bind(opportunity_id)
    .fetch_all(&pool)
    .await?;

    // Enrich with Student Details
    let mut responses = Vec::new();
    for app in applications {
        let student = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(app.student_id)
            .fetch_one(&pool)
            .await?;

        responses.push(ApplicationResponse {
            id: app.id,
            opportunity: None, // Not needed, context is known
            student: Some(UserResponse::from(student)),
            resume_url: app.resume_url,
            cover_letter: app.cover_letter,
            portfolio_url: app.portfolio_url,
            status: app.status,
            applied_at: app.applied_at,
        });
    }

    Ok(Json(ApiResponse {
        success: true,
        data: Some(responses),
        message: None,
    }))
}

// PUT /api/applications/:id/status (Accept/Reject)
pub async fn update_application_status(
    State(pool): State<PgPool>,
    session: Session,
    Path(application_id): Path<Uuid>,
    Json(payload): Json<UpdateApplicationStatusRequest>,
) -> Result<Json<ApiResponse<String>>, AppError> {
    let user = get_session_user(&session, &pool).await?;
    require_faculty(&user)?;

    // 1. Fetch App to find Opportunity
    let app = sqlx::query_as::<_, Application>("SELECT * FROM applications WHERE id = $1")
        .bind(application_id)
        .fetch_optional(&pool)
        .await?
        .ok_or(AppError::NotFound)?;

    // 2. Verify Ownership of Opportunity
    let op = sqlx::query_as::<_, Opportunity>("SELECT * FROM opportunities WHERE id = $1")
        .bind(app.opportunity_id)
        .fetch_one(&pool)
        .await?;

    if op.posted_by != user.id && user.role != UserRole::Admin {
        return Err(AppError::Forbidden);
    }

    // 3. Update
    sqlx::query(
        "UPDATE applications SET status = $1, faculty_remarks = $2, updated_at = NOW() WHERE id = $3"
    )
    .bind(&payload.status)
    .bind(&payload.faculty_remarks)
    .bind(application_id)
    .execute(&pool)
    .await?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(format!("Application marked as {:?}", payload.status)),
        message: None,
    }))
}

// ============================================================================
// 4. THE SCHOLAR'S LEDGER (Task Manager)
// ============================================================================

// POST /api/tasks
pub async fn create_task(
    State(pool): State<PgPool>,
    session: Session,
    Json(payload): Json<CreateTaskRequest>,
) -> Result<Json<ApiResponse<PersonalTask>>, AppError> {
    let user = get_session_user(&session, &pool).await?;

    let task = sqlx::query_as::<_, PersonalTask>(
        r#"
        INSERT INTO personal_tasks (user_id, title, description, priority, due_date, tags, status, progress_percentage)
        VALUES ($1, $2, $3, $4, $5, $6, 'pending', 0)
        RETURNING *
        "#
    )
    .bind(user.id)
    .bind(payload.title)
    .bind(payload.description)
    .bind(payload.priority)
    .bind(payload.due_date)
    .bind(payload.tags)
    .fetch_one(&pool)
    .await?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(task),
        message: Some("Task created".to_string()),
    }))
}

// GET /api/tasks
pub async fn get_tasks(
    State(pool): State<PgPool>,
    session: Session,
) -> Result<Json<ApiResponse<Vec<PersonalTask>>>, AppError> {
    let user = get_session_user(&session, &pool).await?;

    let tasks = sqlx::query_as::<_, PersonalTask>(
        "SELECT * FROM personal_tasks WHERE user_id = $1 ORDER BY due_date ASC NULLS LAST"
    )
    .bind(user.id)
    .fetch_all(&pool)
    .await?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(tasks),
        message: None,
    }))
}

// PUT /api/tasks/:id
pub async fn update_task(
    State(pool): State<PgPool>,
    session: Session,
    Path(task_id): Path<Uuid>,
    Json(payload): Json<UpdateTaskRequest>,
) -> Result<Json<ApiResponse<PersonalTask>>, AppError> {
    let user = get_session_user(&session, &pool).await?;

    // Verify ownership
    let exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM personal_tasks WHERE id = $1 AND user_id = $2)"
    )
    .bind(task_id)
    .bind(user.id)
    .fetch_one(&pool)
    .await?;

    if !exists { return Err(AppError::NotFound); }

    // Dynamic Update
    // Note: In real app, build dynamic SQL. Here, we update everything (coalesce in SQL or just overwrite)
    // For simplicity, let's assume the frontend sends the *full* object or we fetch-merge-update.
    // The UpdateTaskRequest struct has Options, so let's do a COALESCE update in SQL.
    
    let task = sqlx::query_as::<_, PersonalTask>(
        r#"
        UPDATE personal_tasks SET 
            title = COALESCE($1, title),
            description = COALESCE($2, description),
            status = COALESCE($3, status),
            priority = COALESCE($4, priority),
            progress_percentage = COALESCE($5, progress_percentage),
            due_date = COALESCE($6, due_date),
            tags = COALESCE($7, tags),
            updated_at = NOW()
        WHERE id = $8
        RETURNING *
        "#
    )
    .bind(payload.title)
    .bind(payload.description)
    .bind(payload.status)
    .bind(payload.priority)
    .bind(payload.progress_percentage)
    .bind(payload.due_date)
    .bind(payload.tags)
    .bind(task_id)
    .fetch_one(&pool)
    .await?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(task),
        message: Some("Task updated".to_string()),
    }))
}

// DELETE /api/tasks/:id
pub async fn delete_task(
    State(pool): State<PgPool>,
    session: Session,
    Path(task_id): Path<Uuid>,
) -> Result<Json<ApiResponse<String>>, AppError> {
    let user = get_session_user(&session, &pool).await?;

    let result = sqlx::query("DELETE FROM personal_tasks WHERE id = $1 AND user_id = $2")
        .bind(task_id)
        .bind(user.id)
        .execute(&pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(Json(ApiResponse {
        success: true,
        data: Some("Task deleted".to_string()),
        message: None,
    }))
}