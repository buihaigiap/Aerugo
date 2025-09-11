use axum::{
    routing::{get, post, delete},
    Router,
};
use crate::AppState;
use crate::handlers::storage;

pub fn routes() -> Router<AppState> {
    Router::new()
        // Basic blob operations
        .route("/upload", post(storage::upload_blob))
        .route("/download/:digest", get(storage::download_blob))
        .route("/exists/:digest", get(storage::blob_exists))
        .route("/metadata/:digest", get(storage::blob_metadata))
        .route("/delete/:digest", delete(storage::delete_blob))
        
        // Streaming operations
        .route("/stream/upload", post(storage::upload_blob_streaming))
        .route("/stream/download/:digest", get(storage::download_blob_streaming))
        
        // Health check
        .route("/health", get(storage::health_check))
}
