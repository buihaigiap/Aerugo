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
    println!("Initializing database connection...");
    let db_pool = db::create_pool(&settings).await?;
    println!("Database connection initialized successfully");

    // Shared application state
    let state = AppState {
        db_pool,
        config: settings.clone(),
    };
    println!("Application state created successfully");

    // Register API documentation
    println!("Registering API documentation...");
    let openapi = openapi::ApiDoc::openapi();
    println!("API documentation registered successfully");
    
    // Build our application with routes
    println!("Building application router...");
    let app = Router::new()
        .route("/health", axum::routing::get(handlers::health::check))
        .nest("/api/v1", routes::api::api_router())
        // Docker Registry V2 API
        .nest("/v2", routes::docker_registry_v2::docker_registry_v2_router())
        // Serve Swagger UI
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", openapi))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(tower_http::cors::CorsLayer::permissive()) // Add CORS support
        .with_state(state);
    println!("Application router built successfully");

    // Run it
    println!("Preparing to start server...");
    let listen_address = settings.server.bind_address.clone();
    println!("Listen address: {}", listen_address);
    let addr: std::net::SocketAddr = listen_address.parse()?;
    println!("Parsed address: {}", addr);
    
    println!("Creating TCP listener...");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("TCP listener created successfully");
    
    tracing::info!("listening on {}", addr);
    println!("Starting axum server...");
    axum::serve(listener, app).await?;
    Ok(())
}
