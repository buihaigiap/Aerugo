use aerugo::{create_app, AppState};
use aerugo::config::Settings;
use aerugo::storage::{Storage, s3::S3Storage};
use aerugo::cache::{RegistryCache, CacheConfig};
use anyhow::{Result, Context};
use std::sync::Arc;
use std::time::Duration;
use std::process::{Command, Stdio};
use secrecy::ExposeSecret;

#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration
    let settings = Settings::load().expect("Failed to load configuration");
    settings.validate_all().expect("Invalid configuration");

    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Start frontend development server in debug mode
    // Disabled to serve static files via backend instead
    // #[cfg(debug_assertions)]
    // start_frontend_dev_server();

    println!("🚀 Starting Aerugo Container Registry");
    if cfg!(debug_assertions) {
        println!("🔧 Development Mode");
        println!("🔗 Full Application: http://localhost:8080");
        println!("🔗 API Docs: http://localhost:8080/docs");
    } else {
        println!("🏭 Production Mode");  
        println!("🔗 Full Application: http://localhost:8080");
        println!("🔗 API Docs: http://localhost:8080/docs");
    }
    println!();

    // Initialize database connection and run migrations
    println!("Initializing database connection and running migrations...");
    let db_pool = aerugo::db::create_pool(&settings)
        .await
        .context("Failed to create database pool and run migrations")?;
    
    println!("Database connection and migrations completed successfully");

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

    // Initialize email service
    println!("Initializing email service...");
    let email_service = match aerugo::email::EmailService::new(settings.email.clone()) {
        Ok(service) => {
            if settings.email.test_mode {
                println!("Email service initialized in TEST MODE");
            } else {
                println!("Email service initialized with SMTP: {}:{}", 
                    settings.email.smtp_host, settings.email.smtp_port);
            }
            Arc::new(service)
        },
        Err(e) => {
            return Err(anyhow::anyhow!("Failed to initialize email service: {}", e));
        }
    };

    // Create shared application state
    let state = AppState {
        db_pool: db_pool.clone(),
        config: settings.clone(),
        storage,
        cache,
        manifest_cache: Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new())),
        email_service,
    };
    println!("Application state created successfully");

    // Start background task to cleanup expired API keys
    let cleanup_db_pool = db_pool.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(3600)); // Run every hour
        loop {
            interval.tick().await;
            if let Err(e) = aerugo::handlers::auth::cleanup_expired_api_keys(&cleanup_db_pool).await {
                tracing::error!("Failed to cleanup expired API keys: {}", e);
            }
        }
    });
    println!("Background API key cleanup task started");

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

#[cfg(debug_assertions)]
fn start_frontend_dev_server() {
    use std::path::Path;
    
    let fe_dir = "app/Fe-AI-Decenter";
    
    if !Path::new(fe_dir).exists() {
        println!("⚠️  Frontend directory not found: {}", fe_dir);
        return;
    }

    println!("📦 Starting frontend development server...");
    
    // Start frontend dev server in background
    std::thread::spawn(move || {
        // First, ensure dependencies are installed
        let npm_install = Command::new("npm")
            .current_dir(fe_dir)
            .args(&["install"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();

        if npm_install.is_err() || !npm_install.unwrap().success() {
            eprintln!("⚠️  Failed to install frontend dependencies");
            return;
        }

        // Start dev server
        let _child = Command::new("npm")
            .current_dir(fe_dir)
            .args(&["run", "dev"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("Failed to start frontend dev server");

        // Keep thread alive
        loop {
            std::thread::sleep(std::time::Duration::from_secs(10));
        }
    });
    
    // Give frontend server time to start
    std::thread::sleep(std::time::Duration::from_millis(2000));
}
