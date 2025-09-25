use axum::{
    routing::{get, post, delete, put},
    Router,
};

use crate::{
    handlers::repositories::{
        list_repositories,
        list_repositories_by_namespace,
        list_public_repositories,
        create_repository,
        update_repository,
        delete_repository,
        get_repository,
    },
    AppState,
};

pub fn repository_router() -> Router<AppState> {
    Router::new()
        .route("/:namespace", post(create_repository))
        .route("/repositories", get(list_repositories))  // List all repositories
        .route("/repositories/public", get(list_public_repositories))  // List public repositories without auth
        .route("/repositories/:namespace", get(list_repositories_by_namespace))  // List filtered by namespace
        .route("/:namespace/repositories/:repo_name", get(get_repository))  // Get repository details
        .route("/:namespace/:repo_name", put(update_repository))
        .route("/:namespace/:repo_name", delete(delete_repository))
}