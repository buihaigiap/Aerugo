use axum::{
    routing::{get, post, delete},
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
        // Docker API routes
        .route("/docker/build", post(registry::docker_build))
        .route("/docker/push", post(registry::docker_push))
        .route("/docker/pull", post(registry::docker_pull))
        .route("/docker/build-upload-s3", post(registry::docker_build_upload_s3))
        // S3 API routes
        .route("/s3/upload", post(registry::s3_upload))
        .route("/s3/download", post(registry::s3_download))
        .route("/s3/list", post(registry::s3_list))
        .route("/s3/delete", delete(registry::s3_delete))
}
