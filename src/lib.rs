use sqlx::PgPool;
use std::sync::Arc;
use axum::Router;
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
}

/// Create the main Axum application router
pub async fn create_app(state: AppState) -> Router {
    // Register API documentation
    let openapi = openapi::ApiDoc::openapi();
    
    Router::new()
        .route("/health", axum::routing::get(handlers::health::check))
        .nest("/api/v1", routes::api::api_router())
        // Docker Registry V2 API
        .nest("/v2", routes::docker_registry_v2::docker_registry_v2_router())
        // Serve Swagger UI
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", openapi))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(tower_http::cors::CorsLayer::permissive())
        .with_state(state)
}