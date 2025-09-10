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
        // Mount organization routes under /orgs prefix
        .nest("/orgs", super::organizations::organization_router())
        // Mount registry routes under /registry prefix
        .nest("/registry", super::registry::routes())
}
