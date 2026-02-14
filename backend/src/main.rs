mod auth;
mod cloudinary;
mod error;
mod partitioned_cookies;
mod structs;
mod telemetry;

use auth::{get_current_user, google_callback, google_login_initiate, logout};
use axum::{
    middleware, routing::get, Json, Router,
};
use error::AppError;
use serde_json::json;
use http::{HeaderName, Method};
use partitioned_cookies::add_partitioned_attribute;
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
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_headers([
            HeaderName::from_static("content-type"),
            HeaderName::from_static("authorization"),
            HeaderName::from_static("accept"),
        ])
        .allow_credentials(true);

    let session_store = MemoryStore::default();
    
    // Only use Secure flag in production (HTTPS). For local development with HTTP, set to false.
    let is_production = std::env::var("RAILWAY_ENVIRONMENT").is_ok();
    tracing::info!("Session configuration: is_production={}, secure_cookies={}", is_production, is_production);
    
    let session_layer = SessionManagerLayer::new(session_store)
        .with_name("aegis_session")
        .with_secure(is_production)
        .with_http_only(true)
        .with_same_site(tower_sessions::cookie::SameSite::None)
        .with_path("/");

    let app = Router::new()
        .route("/", get(|| async { Json(json!({"status": "ok", "message": "Backend is running"})) }))
        .route("/auth/google", get(google_login_initiate))
        .route("/auth/google/callback", get(google_callback))
        .route("/auth/logout", get(logout))
        .route("/auth/me", get(get_current_user))
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
