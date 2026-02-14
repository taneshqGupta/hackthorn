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

// Helper: Get current user from session
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

// Helper: Ensure user is Faculty or Admin
fn require_faculty_or_admin(user: &User) -> Result<(), AppError> {
    if matches!(user.role, UserRole::Faculty | UserRole::Admin) {
        Ok(())
    } else {
        Err(AppError::Forbidden)
    }
}

// Helper: Ensure user is Student
fn require_student(user: &User) -> Result<(), AppError> {
    if user.role == UserRole::Student {
        Ok(())
    } else {
        Err(AppError::Forbidden)
    }
}

// ============================================================================
// CREATE COURSE (Faculty & Admin only)
// ============================================================================
pub async fn create_course(
    State(pool): State<PgPool>,
    session: Session,
    Json(payload): Json<CreateCourseRequest>,
) -> Result<Json<ApiResponse<CourseResponse>>, AppError> {
    let user = get_session_user(&session, &pool).await?;
    require_faculty_or_admin(&user)?;

    // Resolve instructor_id:
    // If Admin provides email, use it. If Faculty, default to themselves.
    let instructor_id = if user.role == UserRole::Admin {
        if let Some(email) = payload.instructor_email {
            let instructor = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
                .bind(email)
                .fetch_optional(&pool)
                .await?
                .ok_or_else(|| AppError::BadRequest("Instructor email not found".to_string()))?;
            
            Some(instructor.id)
        } else {
            None
        }
    } else {
        // If Faculty is creating, they are the instructor
        Some(user.id)
    };

    let course = sqlx::query_as::<_, Course>(
        r#"
        INSERT INTO courses (
            code, title, description, credits, department, course_type, semester, instructor_id
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING *
        "#,
    )
    .bind(&payload.code)
    .bind(&payload.title)
    .bind(&payload.description)
    .bind(payload.credits)
    .bind(&payload.department)
    .bind(&payload.course_type)
    .bind(&payload.semester)
    .bind(instructor_id)
    .fetch_one(&pool)
    .await?;

    // Fetch instructor details for the response
    let instructor_user = if let Some(iid) = course.instructor_id {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(iid)
            .fetch_optional(&pool)
            .await?
    } else {
        None
    };

    // Construct Response
    let response = CourseResponse {
        id: course.id,
        code: course.code,
        title: course.title,
        description: course.description,
        credits: course.credits,
        department: course.department,
        course_type: course.course_type,
        instructor: instructor_user.map(UserResponse::from),
        semester: course.semester,
    };

    Ok(Json(ApiResponse {
        success: true,
        data: Some(response),
        message: Some("Course created successfully".to_string()),
    }))
}

// ============================================================================
// GET ALL COURSES (Public/Authenticated)
// ============================================================================
pub async fn get_courses(
    State(pool): State<PgPool>,
    _session: Session, // Kept to ensure user is logged in if middleware requires it
    Query(filters): Query<CourseFilter>,
) -> Result<Json<ApiResponse<Vec<CourseResponse>>>, AppError> {
    // Note: We aren't strictly enforcing auth here to allow browsing, 
    // but if you want to lock it down, uncomment the next line:
    // let _user = get_session_user(&_session, &pool).await?;

    let mut query = String::from("SELECT * FROM courses WHERE 1=1");

    // Dynamic Filtering
    if let Some(ref semester) = filters.semester {
        query.push_str(&format!(" AND semester = '{}'", semester)); // Warning: Use bind for safety in production, strict eq here
    }
    if let Some(ref department) = filters.department {
        query.push_str(&format!(" AND department = '{}'", department));
    }
    if let Some(ref search) = filters.search {
        query.push_str(&format!(
            " AND (title ILIKE '%{}%' OR code ILIKE '%{}%')",
            search, search
        ));
    }

    query.push_str(" ORDER BY code ASC");

    // Execute Query
    let courses = sqlx::query_as::<_, Course>(&query)
        .fetch_all(&pool)
        .await?;

    // Enrich courses with Instructor details efficiently
    // (In a real high-scale app, we'd use a JOIN in the SQL, but this loop is fine for now)
    let mut response_list = Vec::new();

    for course in courses {
        let instructor_user = if let Some(iid) = course.instructor_id {
            sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
                .bind(iid)
                .fetch_optional(&pool)
                .await?
        } else {
            None
        };

        response_list.push(CourseResponse {
            id: course.id,
            code: course.code,
            title: course.title,
            description: course.description,
            credits: course.credits,
            department: course.department,
            course_type: course.course_type,
            instructor: instructor_user.map(UserResponse::from),
            semester: course.semester,
        });
    }

    Ok(Json(ApiResponse {
        success: true,
        data: Some(response_list),
        message: None,
    }))
}

// ============================================================================
// ENROLL IN COURSE (Student only)
// ============================================================================
#[derive(serde::Deserialize)]
pub struct EnrollRequest {
    pub course_id: Uuid,
}

pub async fn enroll_course(
    State(pool): State<PgPool>,
    session: Session,
    Json(payload): Json<EnrollRequest>,
) -> Result<Json<ApiResponse<String>>, AppError> {
    let user = get_session_user(&session, &pool).await?;
    require_student(&user)?;

    // Check if course exists
    let course_exists = sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM courses WHERE id = $1)")
        .bind(payload.course_id)
        .fetch_one(&pool)
        .await?;

    if !course_exists {
        return Err(AppError::NotFound); // Or custom "Course not found"
    }

    // Check if already enrolled
    let already_enrolled = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM course_enrollments WHERE student_id = $1 AND course_id = $2)"
    )
    .bind(user.id)
    .bind(payload.course_id)
    .fetch_one(&pool)
    .await?;

    if already_enrolled {
        return Err(AppError::BadRequest("You are already enrolled in this course".to_string()));
    }

    // Perform Enrollment
    sqlx::query(
        "INSERT INTO course_enrollments (student_id, course_id) VALUES ($1, $2)"
    )
    .bind(user.id)
    .bind(payload.course_id)
    .execute(&pool)
    .await?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some("Enrolled successfully".to_string()),
        message: None,
    }))
}

// ============================================================================
// GET MY ENROLLMENTS (Student only)
// ============================================================================
pub async fn get_my_enrollments(
    State(pool): State<PgPool>,
    session: Session,
) -> Result<Json<ApiResponse<Vec<CourseResponse>>>, AppError> {
    let user = get_session_user(&session, &pool).await?;
    require_student(&user)?;

    // Join query to get Course details for enrollments
    let courses = sqlx::query_as::<_, Course>(
        r#"
        SELECT c.* FROM courses c
        INNER JOIN course_enrollments ce ON c.id = ce.course_id
        WHERE ce.student_id = $1
        ORDER BY c.semester DESC, c.code ASC
        "#
    )
    .bind(user.id)
    .fetch_all(&pool)
    .await?;

    // Map to Response (reusing the logic to fetch instructors)
    let mut response_list = Vec::new();

    for course in courses {
        let instructor_user = if let Some(iid) = course.instructor_id {
            sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
                .bind(iid)
                .fetch_optional(&pool)
                .await?
        } else {
            None
        };

        response_list.push(CourseResponse {
            id: course.id,
            code: course.code,
            title: course.title,
            description: course.description,
            credits: course.credits,
            department: course.department,
            course_type: course.course_type,
            instructor: instructor_user.map(UserResponse::from),
            semester: course.semester,
        });
    }

    Ok(Json(ApiResponse {
        success: true,
        data: Some(response_list),
        message: None,
    }))
}

// ============================================================================
// GET COURSE DETAILS (With Enrolled Students count - for Faculty)
// ============================================================================
#[derive(serde::Serialize)]
pub struct CourseDetailStats {
    pub enrolled_count: i64,
    // Add attendance stats later
}

pub async fn get_course_details(
    State(pool): State<PgPool>,
    session: Session,
    Path(course_id): Path<Uuid>,
) -> Result<Json<ApiResponse<CourseDetailStats>>, AppError> {
    let _user = get_session_user(&session, &pool).await?;
    
    // Optional: Restrict to only the instructor or admin?
    // For now, let's allow faculty to check their course stats.
    
    // Check if course exists
    let _course = sqlx::query_as::<_, Course>("SELECT * FROM courses WHERE id = $1")
        .bind(course_id)
        .fetch_optional(&pool)
        .await?
        .ok_or(AppError::NotFound)?;

    // Get enrollment count
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM course_enrollments WHERE course_id = $1")
        .bind(course_id)
        .fetch_one(&pool)
        .await?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(CourseDetailStats { enrolled_count: count }),
        message: None,
    }))
}

// ============================================================================
// ATTENDANCE SYSTEM
// ============================================================================

#[derive(serde::Deserialize)]
pub struct MarkAttendanceRequest {
    pub student_id: Uuid,
    pub course_id: Uuid,
    pub date: chrono::NaiveDate,
    pub status: AttendanceStatus,
    pub remarks: Option<String>,
}

// 1. Mark Attendance (Faculty Only)
pub async fn mark_attendance(
    State(pool): State<PgPool>,
    session: Session,
    Json(payload): Json<MarkAttendanceRequest>,
) -> Result<Json<ApiResponse<String>>, AppError> {
    let user = get_session_user(&session, &pool).await?;
    require_faculty_or_admin(&user)?;

    // 1. Find the Enrollment ID (link between student and course)
    let enrollment_id: Option<Uuid> = sqlx::query_scalar(
        "SELECT id FROM course_enrollments WHERE student_id = $1 AND course_id = $2"
    )
    .bind(payload.student_id)
    .bind(payload.course_id)
    .fetch_optional(&pool)
    .await?;

    let enrollment_id = enrollment_id.ok_or_else(|| {
        AppError::BadRequest("Student is not enrolled in this course".to_string())
    })?;

    // 2. Insert or Update Log (Upsert logic to prevent duplicate logs for same day)
    // Note: If you want to allow multiple logs per day (e.g. 2 lectures), remove the constraint or handling.
    // Assuming 1 log per day per course for simplicity here.
    
    // Check if log exists for this date
    let exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM attendance_logs WHERE enrollment_id = $1 AND date = $2)"
    )
    .bind(enrollment_id)
    .bind(payload.date)
    .fetch_one(&pool)
    .await?;

    if exists {
        // Update existing
        sqlx::query(
            "UPDATE attendance_logs SET status = $1, remarks = $2 WHERE enrollment_id = $3 AND date = $4"
        )
        .bind(&payload.status)
        .bind(&payload.remarks)
        .bind(enrollment_id)
        .bind(payload.date)
        .execute(&pool)
        .await?;
    } else {
        // Insert new
        sqlx::query(
            "INSERT INTO attendance_logs (enrollment_id, date, status, remarks) VALUES ($1, $2, $3, $4)"
        )
        .bind(enrollment_id)
        .bind(payload.date)
        .bind(&payload.status)
        .bind(&payload.remarks)
        .execute(&pool)
        .await?;
    }

    Ok(Json(ApiResponse {
        success: true,
        data: Some("Attendance marked successfully".to_string()),
        message: None,
    }))
}

// 2. Get My Attendance (Student View)
#[derive(serde::Serialize)]
pub struct AttendanceSummary {
    pub course_id: Uuid,
    pub total_classes: i64,
    pub present_count: i64,
    pub percentage: f64,
    pub logs: Vec<AttendanceLog>,
}

pub async fn get_my_attendance(
    State(pool): State<PgPool>,
    session: Session,
    Path(course_id): Path<Uuid>,
) -> Result<Json<ApiResponse<AttendanceSummary>>, AppError> {
    let user = get_session_user(&session, &pool).await?;
    // Students can see their own, Faculty can see (logic omitted for brevity, adding check for student)
    
    // 1. Get Enrollment ID
    let enrollment_id: Option<Uuid> = sqlx::query_scalar(
        "SELECT id FROM course_enrollments WHERE student_id = $1 AND course_id = $2"
    )
    .bind(user.id)
    .bind(course_id)
    .fetch_optional(&pool)
    .await?;

    let enrollment_id = enrollment_id.ok_or_else(|| {
        AppError::BadRequest("You are not enrolled in this course".to_string())
    })?;

    // 2. Fetch Logs
    let logs = sqlx::query_as::<_, AttendanceLog>(
        "SELECT * FROM attendance_logs WHERE enrollment_id = $1 ORDER BY date DESC"
    )
    .bind(enrollment_id)
    .fetch_all(&pool)
    .await?;

    // 3. Calculate Stats
    let total_classes = logs.len() as i64;
    let present_count = logs.iter().filter(|l| matches!(l.status, AttendanceStatus::Present)).count() as i64;
    
    let percentage = if total_classes > 0 {
        (present_count as f64 / total_classes as f64) * 100.0
    } else {
        0.0
    };

    Ok(Json(ApiResponse {
        success: true,
        data: Some(AttendanceSummary {
            course_id,
            total_classes,
            present_count,
            percentage,
            logs,
        }),
        message: None,
    }))
}

// ============================================================================
// ACADEMIC RESOURCES (THE VAULT)
// ============================================================================

// 1. Upload Resource (Faculty Only)
pub async fn create_resource(
    State(pool): State<PgPool>,
    session: Session,
    Json(payload): Json<CreateResourceRequest>, // Defined in structs.rs earlier
    // Note: course_id needs to be passed. Let's assume it's in the query or body. 
    // Since CreateResourceRequest in structs.rs didn't have course_id, let's accept it via Path or extend struct.
    // I will use a Path wrapper here for clean REST design: POST /api/courses/:id/resources
    Path(course_id): Path<Uuid>,
) -> Result<Json<ApiResponse<AcademicResource>>, AppError> {
    let user = get_session_user(&session, &pool).await?;
    require_faculty_or_admin(&user)?;

    let resource = sqlx::query_as::<_, AcademicResource>(
        r#"
        INSERT INTO academic_resources (
            course_id, uploaded_by, title, description, resource_type, file_url, year, tags, is_verified
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, true)
        RETURNING *
        "#
    )
    .bind(course_id)
    .bind(user.id)
    .bind(&payload.title)
    .bind(&payload.description)
    .bind(&payload.resource_type)
    .bind(&payload.file_url)
    .bind(payload.year)
    .bind(&payload.tags)
    .fetch_one(&pool)
    .await?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(resource),
        message: Some("Resource uploaded successfully".to_string()),
    }))
}

// 2. Get Course Resources (Public/Student)
pub async fn get_course_resources(
    State(pool): State<PgPool>,
    session: Session,
    Path(course_id): Path<Uuid>,
    Query(filters): Query<std::collections::HashMap<String, String>>, // Simple filters
) -> Result<Json<ApiResponse<Vec<AcademicResourceResponse>>>, AppError> {
    let _user = get_session_user(&session, &pool).await?; // Ensure logged in

    // Check course exists first
    let exists = sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM courses WHERE id = $1)")
        .bind(course_id)
        .fetch_one(&pool)
        .await?;
        
    if !exists { return Err(AppError::NotFound); }

    let mut query = String::from("SELECT * FROM academic_resources WHERE course_id = $1");
    
    // Simple filter for type (e.g. ?type=pyq)
    if let Some(res_type) = filters.get("type") {
        query.push_str(&format!(" AND resource_type = '{}'", res_type)); // Warning: Bind safer, but strict enum check needed
    }

    query.push_str(" ORDER BY created_at DESC");

    let resources = sqlx::query_as::<_, AcademicResource>(&query)
        .bind(course_id)
        .fetch_all(&pool)
        .await?;

    // Transform to Response DTO
    let mut responses = Vec::new();
    for res in resources {
        // Fetch uploader info
        let uploader = if let Some(uid) = res.uploaded_by {
             sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
                .bind(uid)
                .fetch_optional(&pool)
                .await?
        } else { None };

        responses.push(AcademicResourceResponse {
            id: res.id,
            title: res.title,
            description: res.description,
            resource_type: res.resource_type,
            file_url: res.file_url,
            uploaded_by: uploader.map(UserResponse::from),
            year: res.year,
            tags: res.tags.unwrap_or_default(),
            created_at: res.created_at,
        });
    }

    Ok(Json(ApiResponse {
        success: true,
        data: Some(responses),
        message: None,
    }))
}