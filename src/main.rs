use axum::Router;
use anyhow::Result;

mod routes;
mod handlers;
mod models;
mod config;
mod db;

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

    // Build our application with routes
    let app = Router::new()
        .nest("/api/v1", routes::api::api_router())
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(tower_http::cors::CorsLayer::permissive()) // Add CORS support
        .with_state(state);

    // Run it
    let addr: std::net::SocketAddr = settings.server.bind_address.parse()?;
    tracing::info!("listening on {}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await?, app)
        .await?;
    Ok(())
}
