use axum::{
    routing::get,
    Router,
};
use anyhow::Result;

mod routes;
mod handlers;
mod models;
mod config;

use crate::config::settings::Settings;

#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration
    let settings = Settings::load().expect("Failed to load configuration");
    settings.validate_all().expect("Invalid configuration");

    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Build our application with a route
    let app = Router::new()
        .route("/health", get(handlers::health::check))
        .layer(tower_http::trace::TraceLayer::new_for_http());

    // Run it
    let addr: std::net::SocketAddr = settings.server.bind_address.parse()?;
    tracing::info!("listening on {}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await?, app)
        .await?;
    Ok(())
}
