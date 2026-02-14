mod academic;
mod admin;
mod auth;
mod cloudinary;
mod error;
mod grievances;
mod opportunity;
mod partitioned_cookies;
mod structs;
mod telemetry;

use admin::{
    get_all_users, get_audit_logs, get_system_stats, get_user_by_id, seed_dummy_users,
    update_own_role, update_user_role, update_user_status,
};
use auth::{get_current_user, google_callback, google_login_initiate, logout};
use axum::{
    Json, Router, middleware,
    routing::{delete, get, post, put},
};
use error::AppError;
use grievances::{
    add_comment, assign_grievance, create_grievance, delete_grievance, get_comments,
    get_departments, get_grievance_by_id, get_grievance_history, get_grievances, resolve_grievance,
    toggle_upvote, update_grievance_status, upload_grievance_photos,
};
use http::{HeaderName, Method};
use partitioned_cookies::add_partitioned_attribute;
use serde_json::json;
use sqlx::PgPool;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tower_sessions::{MemoryStore, SessionManagerLayer};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    telemetry::init_telemetry();

    let url = std::env::var("DATABASE_URL").unwrap();

    tracing::info!("Attempting to connect to database using URL: {:?}", url);
    let pool = PgPool::connect(&url).await.map_err(|e| {
        tracing::error!("Failed to connect to database: {:?}", e);
        e
    })?;
    tracing::info!("Successfully connected to database.");

    let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:4173".parse().unwrap(),
            std::env::var("FRONTEND_URL").unwrap().parse().unwrap(),
        ])
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([
            HeaderName::from_static("content-type"),
            HeaderName::from_static("authorization"),
            HeaderName::from_static("accept"),
        ])
        .allow_credentials(true);

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_name("aegis_session")
        .with_secure(false)
        .with_http_only(true)
        .with_same_site(tower_sessions::cookie::SameSite::Lax)
        .with_path("/");

    let app = Router::new()
        .route(
            "/",
            get(|| async { Json(json!({"status": "ok", "message": "Backend is running"})) }),
        )
        // Auth routes
        .route("/auth/google", get(google_login_initiate))
        .route("/auth/google/callback", get(google_callback))
        .route("/auth/logout", get(logout))
        .route("/auth/me", get(get_current_user))
        // Grievance routes
        .route("/api/grievances", post(create_grievance))
        .route("/api/grievances", get(get_grievances))
        .route("/api/grievances/{id}", get(get_grievance_by_id))
        .route("/api/grievances/{id}", delete(delete_grievance))
        .route("/api/grievances/{id}/status", put(update_grievance_status))
        .route("/api/grievances/{id}/assign", put(assign_grievance))
        .route("/api/grievances/{id}/resolve", put(resolve_grievance))
        .route("/api/grievances/{id}/upvote", post(toggle_upvote))
        .route("/api/grievances/{id}/photos", post(upload_grievance_photos))
        .route("/api/grievances/{id}/history", get(get_grievance_history))
        .route("/api/grievances/{id}/comments", post(add_comment))
        .route("/api/grievances/{id}/comments", get(get_comments))
        .route("/api/departments", get(get_departments))
        // Admin routes
        .route("/api/admin/users", get(get_all_users))
        .route("/api/admin/users/{id}", get(get_user_by_id))
        .route("/api/admin/users/{id}/role", put(update_user_role))
        .route("/api/admin/users/{id}/status", put(update_user_status))
        .route("/api/admin/audit-logs", get(get_audit_logs))
        .route("/api/admin/stats", get(get_system_stats))
        // Dev/Testing route - allows users to change their own role
        .route("/api/user/role", put(update_own_role))
        .route("/api/dev/seed", post(seed_dummy_users))
        // --- ACADEMIC ROUTES ---
        // Course Management
        .route(
            "/api/courses",
            post(academic::create_course).get(academic::get_courses),
        )
        .route("/api/courses/enroll", post(academic::enroll_course))
        .route(
            "/api/courses/my-enrollments",
            get(academic::get_my_enrollments),
        )
        .route("/api/courses/{id}", get(academic::get_course_details))
        // Attendance
        .route("/api/attendance/mark", post(academic::mark_attendance))
        .route("/api/attendance/{id}", get(academic::get_my_attendance))
        // Resources (Vault)
        .route(
            "/api/courses/{id}/resources",
            post(academic::create_resource).get(academic::get_course_resources),
        )
        // Calendar
        .route(
            "/api/events",
            post(academic::create_event).get(academic::get_my_calendar),
        )
        // --- OPPORTUNITY ROUTES (PILLAR 4) ---
        // 1. Opportunities (Faculty post, Students browse)
        .route(
            "/api/opportunities",
            post(opportunity::create_opportunity).get(opportunity::get_opportunities),
        )
        // 2. Applications (Apply & View My History)
        .route(
            "/api/opportunities/{id}/apply",
            post(opportunity::apply_opportunity),
        )
        .route(
            "/api/applications/my-applications",
            get(opportunity::get_my_applications),
        )
        // 3. Faculty Management (View Applicants & Update Status)
        .route(
            "/api/opportunities/{id}/applications",
            get(opportunity::get_opportunity_applications),
        )
        .route(
            "/api/applications/{id}/status",
            put(opportunity::update_application_status),
        )
        // 4. The Scholar's Ledger (Personal Tasks)
        .route(
            "/api/tasks",
            post(opportunity::create_task).get(opportunity::get_tasks),
        )
        .route(
            "/api/tasks/{id}",
            put(opportunity::update_task).delete(opportunity::delete_task),
        )
        .with_state(pool)
        .layer(session_layer)
        .layer(middleware::from_fn(add_partitioned_attribute))
        .layer(cors);

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse::<u16>()?;

    let address = SocketAddr::from(([0, 0, 0, 0], port));

    let listener = TcpListener::bind(&address).await?;
    tracing::debug!("listening on {}", listener.local_addr()?);

    axum::serve(listener, app).await?;

    Ok(())
}
