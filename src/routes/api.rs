use axum::{
    Router,
    routing::{get, post},
};
use crate::AppState;
use crate::handlers;

pub fn api_router() -> Router<AppState> {
    Router::new()
        .route("/health", get(handlers::health::check))
        // Mount auth routes under /auth prefix
        .nest("/auth", super::auth::auth_router())
}
