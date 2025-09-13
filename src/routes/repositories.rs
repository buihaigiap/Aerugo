use axum::{
    routing::{get, post, put, delete},
    Router,
};

use crate::{
    handlers::repositories,
    AppState,
};

pub fn repository_router() -> Router<AppState> {
    Router::new()
        .route("/:namespace", post(repositories::create_repository))
        .route("/:namespace/repositories", get(repositories::list_repositories))  // List all repositories
        .route("/:namespace/repositories/:repo_name", get(repositories::get_repository))  // Get specific repository
        .route("/:namespace/:repo_name", delete(repositories::delete_repository))
        .route("/:namespace/:repo_name/permissions", put(repositories::set_repository_permissions))
}