use crate::handlers;
use crate::AppState;
use axum::{
    routing::get,
    Router,
};

pub fn api_router() -> Router<AppState> {
    Router::new()
        // Mount auth routes under /auth prefix
        .nest("/auth", super::auth::auth_router())
        // Mount organization routes under /organizations prefix
        .nest("/organizations", super::organizations::organization_router())
        // Mount storage routes under /storage prefix
        .nest("/storage", super::storage::routes())
        // Mount repository management routes under /repos prefix
        .nest("/repos", super::repositories::repository_router())
}
