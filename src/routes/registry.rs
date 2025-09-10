use axum::{
    routing::get,
    Router,
};

use crate::{
    handlers::registry,
    AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/repositories", get(registry::list_repositories))
        .route("/repositories/:repo_id", get(registry::get_repository))
        .route("/repositories/:repo_id/images", get(registry::list_images))
}
