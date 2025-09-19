use axum::{
    routing::{get, post, delete},
    Router,
};

use crate::{
    handlers::repositories::{
        list_repositories,
        list_repositories_by_namespace,
        create_repository,
        delete_repository,
        get_repository,
    },
    AppState,
};

pub fn repository_router() -> Router<AppState> {
    Router::new()
        .route("/:namespace", post(create_repository))
        .route("/repositories", get(list_repositories))  // List all repositories
        .route("/repositories/:namespace", get(list_repositories_by_namespace))  // List filtered by namespace
        .route("/:namespace/repositories/:repo_name", get(get_repository))  // Get repository details
        .route("/:namespace/:repo_name", delete(delete_repository))
}