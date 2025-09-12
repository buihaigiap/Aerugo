use aerugo::{create_app, AppState};
use aerugo::config::Settings;
use aerugo::db;
use aerugo::storage::{Storage, s3::S3Storage};
use anyhow::Result;
use std::sync::Arc;
use secrecy::ExposeSecret;

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

    // Initialize S3 storage
    println!("Initializing S3 storage...");
    let s3_config = aerugo::storage::s3::S3Config {
        endpoint: settings.storage.endpoint.clone(),
        bucket: settings.storage.bucket_name().to_string(),
        region: settings.storage.region.clone(),
        auth_method: aerugo::storage::s3::S3AuthMethod::Static {
            access_key_id: settings.storage.access_key_id.expose_secret().clone(),
            secret_access_key: settings.storage.secret_access_key.expose_secret().clone(),
        },
        use_path_style: settings.storage.use_path_style,
        retry_attempts: Some(3),
        multipart_threshold: Some(64 * 1024 * 1024), // 64MB
        part_size: Some(8 * 1024 * 1024), // 8MB
    };
    
    let storage: Arc<dyn Storage> = Arc::new(
        S3Storage::new(&s3_config)
            .await
            .expect("Failed to initialize S3 storage")
    );
    println!("S3 storage initialized successfully");

    // Create shared application state
    let state = AppState {
        db_pool,
        config: settings.clone(),
        storage,
        cache: None, // TODO: Initialize Redis cache
    };
    println!("Application state created successfully");

    // Create application using lib.rs
    let app = create_app(state).await;
    println!("Application created successfully");

    // Run server
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
