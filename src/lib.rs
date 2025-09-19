use sqlx::PgPool;
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use axum::{Router, response::Html, http::{StatusCode, Uri}};
use axum::routing::get;
use tower_http::services::{ServeDir, ServeFile};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod auth;
pub mod cache;
pub mod config;
pub mod database;
pub mod db;
pub mod handlers;
pub mod models;
pub mod openapi;
pub mod routes;
pub mod storage;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub config: config::Settings,
    pub storage: Arc<dyn storage::Storage>,
    pub cache: Option<Arc<cache::RegistryCache>>,
    pub manifest_cache: Arc<RwLock<HashMap<String, String>>>, // digest -> content
}

// Handler for serving index.html (SPA entry point)
async fn serve_spa() -> Result<Html<String>, StatusCode> {
    match tokio::fs::read_to_string("dist/static/index.html").await {
        Ok(content) => Ok(Html(content)),
        Err(_) => Ok(Html(r#"
<!DOCTYPE html>
<html>
<head>
    <title>Aerugo Registry</title>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <style>
        body { font-family: system-ui; padding: 2rem; text-align: center; }
        code { background: #f5f5f5; padding: 0.2rem 0.4rem; border-radius: 3px; }
    </style>
</head>
<body>
    <div id="root">
        <h1>🚀 Aerugo Container Registry</h1>
        <p>Frontend not built yet. Please run:</p>
        <code>./build-frontend.sh</code>
        <br><br>
        <p><a href="/docs">📖 API Documentation</a></p>
    </div>
</body>
</html>
        "#.to_string()))
    }
}

// Fallback handler for SPA routes
async fn spa_fallback(uri: Uri) -> Result<Html<String>, StatusCode> {
    let path = uri.path();
    
    // Don't handle API routes
    if path.starts_with("/api") || path.starts_with("/v2") || path.starts_with("/docs") {
        return Err(StatusCode::NOT_FOUND);
    }
    
    // For all other routes, serve the SPA
    serve_spa().await
}

/// Create the main Axum application router
pub async fn create_app(state: AppState) -> Router {
    // Register API documentation
    let openapi = openapi::ApiDoc::openapi();
    
    // API routes with state
    let api_router = Router::new()
        .nest("/api/v1", routes::api::api_router())
        // Docker Registry V2 API routes - direct routes to avoid nesting conflicts
        .merge(routes::docker_registry_v2::docker_registry_v2_router())
        // Health and monitoring endpoints  
        .merge(routes::health::health_router())
        // Serve Swagger UI
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", openapi))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(tower_http::cors::CorsLayer::permissive())
        .with_state(state);

    // Static files and SPA
    let static_router = Router::new()
        .nest_service("/assets", ServeDir::new("dist/static/assets"))
        .route_service("/favicon.ico", ServeFile::new("dist/static/favicon.ico"))
        .route("/", get(serve_spa))
        .fallback(spa_fallback);

    // Combine everything
    Router::new()
        .merge(api_router)
        .merge(static_router)
}