// Docker Registry V2 API routes
use axum::{
    routing::{get, post, put, patch, delete},
    Router,
};

use crate::{
    handlers::docker_registry_v2,
    AppState,
};

/// Creates the Docker Registry V2 API router
/// All routes are prefixed with /v2 and follow the Docker Registry V2 specification
pub fn docker_registry_v2_router() -> Router<AppState> {
    Router::new()
        
        // Docker Registry V2 version check - explicit route first
        .route("/", get(docker_registry_v2::version_check))
        
        // Repository catalog - specific routes before params
        .route("/_catalog", get(docker_registry_v2::get_catalog))
        
        // Use more specific patterns for Docker registry endpoints
        // These patterns should handle both simple names and namespaced names like org/repo
        
        // Tag listing endpoints - handles simple names and namespaced names
        .route("/:name/tags/list", get(docker_registry_v2::list_tags))
        .route("/:org/:name/tags/list", get(docker_registry_v2::list_tags_namespaced))
        
        // Manifest operations - simple names
        .route("/:name/manifests/:reference", 
            get(docker_registry_v2::get_manifest)
                .put(docker_registry_v2::put_manifest)
                .head(docker_registry_v2::head_manifest)
                .delete(docker_registry_v2::delete_manifest)
        )
        
        // Manifest operations - namespaced names (org/name)
        .route("/:org/:name/manifests/:reference",
            get(docker_registry_v2::get_manifest_namespaced)
                .put(docker_registry_v2::put_manifest_namespaced)
                .head(docker_registry_v2::head_manifest_namespaced)
                .delete(docker_registry_v2::delete_manifest_namespaced)
        )
        
        // Blob operations for simple names
        .route("/:name/blobs/:digest", 
            get(docker_registry_v2::get_blob)
                .head(docker_registry_v2::head_blob)
        )
        .route("/:org/:name/blobs/:digest", 
            get(docker_registry_v2::get_blob_namespaced)
                .head(docker_registry_v2::head_blob_namespaced)
        )
        
        // Blob upload operations for simple names
        .route("/:name/blobs/uploads/", post(docker_registry_v2::start_blob_upload))
        .route("/:org/:name/blobs/uploads/", post(docker_registry_v2::start_blob_upload_namespaced))
        
        // Blob upload by repository ID (authenticated with JWT)
        .route("/id/:repository_id/blobs/uploads/", post(docker_registry_v2::start_blob_upload_by_id))
        
        .route("/:name/blobs/uploads/:uuid",
            get(docker_registry_v2::get_upload_status)
                .patch(docker_registry_v2::upload_blob_chunk)
                .put(docker_registry_v2::complete_blob_upload)
                .delete(docker_registry_v2::cancel_blob_upload)
        )
        
        .route("/:org/:name/blobs/uploads/:uuid",
            get(docker_registry_v2::get_upload_status_namespaced)
                .patch(docker_registry_v2::upload_blob_chunk_namespaced)
                .put(docker_registry_v2::complete_blob_upload_namespaced)
                .delete(docker_registry_v2::cancel_blob_upload_namespaced)
        )
}