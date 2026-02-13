mod auth;
mod cloudinary;
mod error;
mod partitioned_cookies;
mod posts;
mod structs;
mod telemetry;
use auth::{
    check_auth, get_my_profile, get_my_user_id, get_user_profile, login, logout, register,
    update_profile_picture,
};
use axum::{
    Router, middleware,
    routing::{delete, get, post},
};
use error::AppError;
use http::{HeaderName, Method};
use partitioned_cookies::add_partitioned_attribute;
use posts::{
    create_post, delete_post, list_community_offers, list_community_posts, list_community_requests,
    list_my_posts, list_offers, list_requests, update_post,
};
use sqlx::PgPool;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tower_sessions::{MemoryStore, SessionManagerLayer};

use crate::posts::list_user_posts;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    telemetry::init_telemetry();

    let url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "http://0.0.0.0:8000".to_string());

    tracing::info!("Attempting to connect to database using URL: {:?}", url);
    let pool = PgPool::connect(&url).await.map_err(|e| {
        tracing::error!("Failed to connect to database: {:?}", e);
        e
    })?;
    tracing::info!("Successfully connected to database.");

    let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:3000".parse().unwrap(),
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
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(true)
        .with_same_site(tower_sessions::cookie::SameSite::None);

    let app = Router::new()
        .route("/", get(list_my_posts))
        .route("/posts", get(list_my_posts))
        .route("/posts/offers", get(list_offers))
        .route("/posts/requests", get(list_requests))
        .route("/foreignposts/{userid}", get(list_user_posts))
        .route("/community", get(list_community_posts))
        .route("/community/offers", get(list_community_offers))
        .route("/community/requests", get(list_community_requests))
        .route("/posts/create", post(create_post))
        .route("/posts/delete/{id}", delete(delete_post))
        .route("/posts/update", post(update_post))
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .route("/auth/logout", post(logout))
        .route("/auth/check", get(check_auth))
        .route("/auth/myprofile", get(get_my_profile))
        .route("/auth/my_userid", get(get_my_user_id))
        .route("/auth/myprofile/picture", post(update_profile_picture))
        .route("/auth/userprofile/{user_id}", get(get_user_profile))
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
