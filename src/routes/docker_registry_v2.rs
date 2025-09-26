// Docker Registry V2 API routes
use axum::{
    routing::{get, post, put, patch, delete, any},
    Router, response::Redirect,
};

use crate::{
    handlers::docker_registry_v2,
    AppState,
};

// Helper function to handle redirect from /v2 to /v2/
async fn redirect_to_v2() -> Redirect {
    Redirect::permanent("/v2/")
}

/// Creates the Docker Registry V2 API router
/// All routes are prefixed with /v2 and follow the Docker Registry V2 specification
pub fn docker_registry_v2_router() -> Router<AppState> {
    Router::new()
        
        // Docker Registry V2 version check - both /v2 and /v2/
        .route("/v2", get(docker_registry_v2::version_check))
        .route("/v2/", get(docker_registry_v2::version_check))
        
        // Repository catalog
        .route("/v2/_catalog", get(docker_registry_v2::get_catalog))
        
        // Use more specific patterns for Docker registry endpoints
        // These patterns should handle both simple names and namespaced names like org/repo
        
        // Tag listing endpoints - handles simple names and namespaced names
        .route("/v2/:name/tags/list", get(docker_registry_v2::list_tags))
        .route("/v2/:org/:name/tags/list", get(docker_registry_v2::list_tags_namespaced))
        
        // Manifest operations - simple names
        .route("/v2/:name/manifests/:reference", 
            get(docker_registry_v2::get_manifest)
                .put(docker_registry_v2::put_manifest)
                .head(docker_registry_v2::head_manifest)
                .delete(docker_registry_v2::delete_manifest)
        )
        
        // Manifest operations - namespaced names (org/name)
        .route("/v2/:org/:name/manifests/:reference",
            get(docker_registry_v2::get_manifest_namespaced)
                .put(docker_registry_v2::put_manifest_namespaced)
                .head(docker_registry_v2::head_manifest_namespaced)
                .delete(docker_registry_v2::delete_manifest_namespaced)
        )
        
        // Blob operations for simple names
        .route("/v2/:name/blobs/:digest", 
            get(docker_registry_v2::get_blob)
                .head(docker_registry_v2::head_blob)
        )
        .route("/v2/:org/:name/blobs/:digest", 
            get(docker_registry_v2::get_blob_namespaced)
                .head(docker_registry_v2::head_blob_namespaced)
        )
        
        // List all blobs in repository (custom API - not Docker Registry V2 standard)
        .route("/v2/:name/blobs/", get(docker_registry_v2::list_blobs))
        .route("/v2/:org/:name/blobs/", get(docker_registry_v2::list_blobs_namespaced))
        
        // Blob upload operations for simple names
        .route("/v2/:name/blobs/uploads/", post(docker_registry_v2::start_blob_upload))
        .route("/v2/:org/:name/blobs/uploads/", post(docker_registry_v2::start_blob_upload_namespaced))
        
        // Blob upload by repository ID (authenticated with JWT)
        .route("/v2/id/:repository_id/blobs/uploads/", post(docker_registry_v2::start_blob_upload_by_id))
        
        .route("/v2/:name/blobs/uploads/:uuid",
            get(docker_registry_v2::get_upload_status)
                .patch(docker_registry_v2::upload_blob_chunk)
                .put(docker_registry_v2::complete_blob_upload)
                .delete(docker_registry_v2::cancel_blob_upload)
        )
        
        .route("/v2/:org/:name/blobs/uploads/:uuid",
            get(docker_registry_v2::get_upload_status_namespaced)
                .patch(docker_registry_v2::upload_blob_chunk_namespaced)
                .put(docker_registry_v2::complete_blob_upload_namespaced)
                .delete(docker_registry_v2::cancel_blob_upload_namespaced)
        )
}