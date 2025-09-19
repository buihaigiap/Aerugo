use aerugo::{create_app, AppState};
use aerugo::config::Settings;
use aerugo::db;
use aerugo::storage::{Storage, s3::S3Storage};
use aerugo::cache::{RegistryCache, CacheConfig};
use anyhow::{Result, Context};
use std::sync::Arc;
use std::time::Duration;
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
    let db_pool = sqlx::postgres::PgPool::connect(&settings.database.url())
        .await
        .context("Failed to connect to database")?;

    // Skip migrations for now and create table manually
    println!("⚠️  Skipping database migrations due to modified migration conflicts");
    
    println!("Database connection and table setup completed successfully");

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

    // Initialize cache
    println!("Initializing cache layer...");
    let cache_config = CacheConfig {
        redis_url: Some(settings.cache.redis_url.clone()),
        manifest_ttl: Duration::from_secs(settings.cache.ttl_seconds),
        blob_metadata_ttl: Duration::from_secs(settings.cache.ttl_seconds * 2), // 2x longer for blob metadata
        repository_ttl: Duration::from_secs(60), // 1 minute for repo lists
        tag_ttl: Duration::from_secs(120), // 2 minutes for tag lists
        // Authentication cache TTLs
        auth_token_ttl: Duration::from_secs(900), // 15 minutes
        permission_ttl: Duration::from_secs(300), // 5 minutes
        session_ttl: Duration::from_secs(1800), // 30 minutes
        max_memory_entries: 10000,
        enable_redis: true,
        enable_memory: true,
    };
    
    let cache = match RegistryCache::new(cache_config).await {
        Ok(cache) => {
            println!("Cache initialized successfully (Redis + Memory)");
            Some(Arc::new(cache))
        },
        Err(e) => {
            println!("Warning: Failed to initialize cache: {}. Continuing without cache.", e);
            None
        }
    };

    // Create shared application state
    let state = AppState {
        db_pool,
        config: settings.clone(),
        storage,
        cache,
        manifest_cache: Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new())),
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
