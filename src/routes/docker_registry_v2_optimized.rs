// Optimized Docker Registry V2 routes with performance enhancements
use axum::{
    routing::{get, post, put, patch, delete, head},
    Router, middleware,
};

use crate::{
    handlers::docker_registry_v2_optimized,
    AppState,
};

/// Creates the optimized Docker Registry V2 API router
/// All routes include caching, performance optimizations, and production features
pub fn docker_registry_v2_optimized_router() -> Router<AppState> {
    Router::new()
        // Base API version endpoint
        .route("/", get(docker_registry_v2_optimized::base_api))
        
        // Repository catalog with caching
        .route("/_catalog", get(docker_registry_v2_optimized::get_catalog_optimized))
        
        // Tag listing endpoints with caching - handles simple names and namespaced names
        .route("/:name/tags/list", get(docker_registry_v2_optimized::list_tags_optimized))
        .route("/:org/:name/tags/list", get(docker_registry_v2_optimized::list_tags_namespaced_optimized))
        
        // Manifest operations for simple names with caching
        .route("/:name/manifests/:reference", 
            get(docker_registry_v2_optimized::get_manifest_optimized)
                .put(docker_registry_v2_optimized::put_manifest_optimized)
                .delete(docker_registry_v2_optimized::delete_manifest_optimized)
        )
        
        // Manifest operations for namespaced names with caching
        .route("/:org/:name/manifests/:reference",
            get(docker_registry_v2_optimized::get_manifest_namespaced_optimized)
                .put(docker_registry_v2_optimized::put_manifest_namespaced_optimized)
                .delete(docker_registry_v2_optimized::delete_manifest_namespaced_optimized)
        )
        
        // HEAD requests for manifests with caching
        .route("/:name/manifests/:reference", head(docker_registry_v2_optimized::head_manifest_optimized))
        .route("/:org/:name/manifests/:reference", head(docker_registry_v2_optimized::head_manifest_namespaced_optimized))
        
        // Blob operations for simple names with streaming and caching
        .route("/:name/blobs/:digest", get(docker_registry_v2_optimized::get_blob_optimized))
        .route("/:org/:name/blobs/:digest", get(docker_registry_v2_optimized::get_blob_namespaced_optimized))
        
        // HEAD requests for blobs with caching
        .route("/:name/blobs/:digest", head(docker_registry_v2_optimized::head_blob_optimized))
        .route("/:org/:name/blobs/:digest", head(docker_registry_v2_optimized::head_blob_namespaced_optimized))
        
        // Blob upload operations for simple names with optimized chunking
        .route("/:name/blobs/uploads/", post(docker_registry_v2_optimized::start_blob_upload_optimized))
        .route("/:org/:name/blobs/uploads/", post(docker_registry_v2_optimized::start_blob_upload_namespaced_optimized))
        
        .route("/:name/blobs/uploads/:uuid",
            get(docker_registry_v2_optimized::get_upload_status_optimized)
                .patch(docker_registry_v2_optimized::upload_blob_chunk_optimized)
                .put(docker_registry_v2_optimized::complete_blob_upload_optimized)
                .delete(docker_registry_v2_optimized::cancel_blob_upload_optimized)
        )
        
        .route("/:org/:name/blobs/uploads/:uuid",
            get(docker_registry_v2_optimized::get_upload_status_namespaced_optimized)
                .patch(docker_registry_v2_optimized::upload_blob_chunk_namespaced_optimized)
                .put(docker_registry_v2_optimized::complete_blob_upload_namespaced_optimized)
                .delete(docker_registry_v2_optimized::cancel_blob_upload_namespaced_optimized)
        )
        
        // Performance monitoring endpoint
        .route("/_aerugo/metrics", get(docker_registry_v2_optimized::get_performance_metrics))
        .route("/_aerugo/cache/stats", get(docker_registry_v2_optimized::get_cache_stats))
        .route("/_aerugo/cache/clear", post(docker_registry_v2_optimized::clear_cache))
}
