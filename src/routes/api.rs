use crate::handlers;
use crate::routes::organizations;
use crate::AppState;
use axum::{
    routing::{get, post},
    Router,
};

pub fn api_router() -> Router<AppState> {
    Router::new()
        .route("/health", get(handlers::health::check))
        // Mount auth routes under /auth prefix
        .nest("/auth", super::auth::auth_router())
        // Mount organization routes under /orgs prefix
        .nest("/orgs", super::organizations::organization_router())
}
