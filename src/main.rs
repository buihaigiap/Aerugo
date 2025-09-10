use anyhow::Result;
use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod auth;
mod config;
mod database;
mod db;
mod handlers;
mod models;
mod openapi;
mod routes;
// mod storage; // Temporarily disabled due to AWS SDK compatibility issues

use crate::config::settings::Settings;

#[derive(Clone)]
pub struct AppState {
    db_pool: sqlx::PgPool,
    config: Settings,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration
    let settings = Settings::load().expect("Failed to load configuration");
    settings.validate_all().expect("Invalid configuration");

    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Initialize database connection
    let db_pool = db::create_pool(&settings).await?;

    // Create app state
    let state = AppState {
        db_pool,
        config: settings.clone(),
    };

    // Register API documentation
    let openapi = openapi::ApiDoc::openapi();
    
    // Build our application with routes
    let app = Router::new()
        .route("/health", axum::routing::get(handlers::health::check))
        .nest("/api/v1", routes::api::api_router())
        // Serve Swagger UI
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", openapi))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(tower_http::cors::CorsLayer::permissive()) // Add CORS support
        .with_state(state);

    // Run it
    let listen_address = settings.server.bind_address.clone();
    let addr: std::net::SocketAddr = listen_address.parse()?;
    tracing::info!("listening on {}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;
    Ok(())
}
