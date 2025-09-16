use axum::{
    routing::{get, post, put, delete},
    Router,
};

use crate::{
    handlers::repositories::{
        list_repositories,
        create_repository,
        delete_repository,
        update_repository_permissions,
    },
    AppState,
};

pub fn repository_router() -> Router<AppState> {
    Router::new()
        .route("/:namespace", post(create_repository))
        .route("/repositories", get(list_repositories))  // List all repositories
        .route("/repositories/:namespace", get(list_repositories))  // List filtered by namespace
        .route("/:namespace/:repo_name", delete(delete_repository))
        .route("/:namespace/:repo_name/permissions", put(update_repository_permissions))
}