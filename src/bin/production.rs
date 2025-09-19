use aerugo::config::{Settings, ProductionSettings};
use aerugo::cache::{RegistryCache, CacheConfig};
use aerugo::storage::{Storage, s3::S3Storage};
use aerugo::{create_app, AppState};
use anyhow::Context;
use sqlx::postgres::PgPoolOptions;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{info, warn};
use secrecy::ExposeSecret;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing cho production logging
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "aerugo=info,tower_http=debug".into()),
        )
        .finish(); // Remove .json() as it may not be available
    tracing::subscriber::set_global_default(subscriber)
        .context("Failed to set tracing subscriber")?;

    info!("🚀 Starting Aerugo Docker Registry with production optimizations");

    // Load configuration
    let settings = Settings::load().context("Failed to load application settings")?;
    let production_config = ProductionSettings::load()
        .context("Failed to load production settings")?;

    // Validate production configuration
    production_config
        .validate()
        .context("Invalid production configuration")?;

    info!(
        "📊 Production config loaded - Redis pool: {}, DB pool: {}, Cache enabled: {}",
        production_config.cache.redis.max_connections,
        production_config.database_pool.max_connections,
        production_config.cache.metrics_enabled
    );

    // Create database connection pool with production settings
    let connection_url = settings.database.url();
    info!("🔗 Connecting to database: {}", connection_url.replace(settings.database.password.expose_secret(), "[HIDDEN]"));
    
    let database_pool = PgPoolOptions::new()
        .max_connections(production_config.database_pool.max_connections)
        .min_connections(production_config.database_pool.min_connections)
        .acquire_timeout(Duration::from_secs(production_config.database_pool.connect_timeout))
        .max_lifetime(Duration::from_secs(production_config.database_pool.max_lifetime))
        .idle_timeout(Duration::from_secs(production_config.database_pool.idle_timeout))
        .connect(&connection_url)
        .await
        .context("Failed to create database pool")?;

        info!("✅ Database pool established with {} max connections", 
          production_config.database_pool.max_connections);

    // Run database migrations
    // sqlx::migrate!("./migrations")
    //     .run(&database_pool)
    //     .await
    //     .context("Failed to run database migrations")?;

    info!("✅ Database migrations skipped for now");

    // Initialize Redis cache with production config
    let redis_url = production_config.redis_url_with_auth(
        std::env::var("REDIS_PASSWORD").ok().as_deref()
    );

    let cache_config = aerugo::cache::CacheConfig {
        redis_url: Some(redis_url),
        manifest_ttl: Duration::from_secs(production_config.cache.memory.manifest_ttl),
        blob_metadata_ttl: Duration::from_secs(production_config.cache.memory.blob_metadata_ttl),
        repository_ttl: Duration::from_secs(production_config.cache.memory.repository_ttl),
        tag_ttl: Duration::from_secs(production_config.cache.memory.tag_ttl),
        // Authentication cache TTLs
        auth_token_ttl: Duration::from_secs(900), // 15 minutes
        permission_ttl: Duration::from_secs(300), // 5 minutes
        session_ttl: Duration::from_secs(1800), // 30 minutes
        max_memory_entries: production_config.cache.memory.max_entries as usize,
        enable_redis: true,
        enable_memory: true,
    };

    let cache = RegistryCache::new(cache_config)
        .await
        .context("Failed to initialize registry cache")?;

    info!("✅ Registry cache initialized with Redis + in-memory layers");

    // Initialize S3 storage with production settings
    let aws_config = aws_config::load_from_env().await;
    let s3_client = aws_sdk_s3::Client::new(&aws_config);
    
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
            .context("Failed to initialize S3 storage")?
    );

    info!("✅ S3 storage initialized - bucket: {}", settings.storage.bucket_name());

    // Create application state with production optimizations
    let app_state = AppState {
        db_pool: database_pool,
        config: settings.clone(),
        cache: Some(Arc::new(cache)),
        storage,
        manifest_cache: Arc::new(RwLock::new(HashMap::new())),
    };

    // Create Axum application with optimized routes
    let app = create_app(app_state.clone()).await;

    // Configure server with production settings
    let listener = tokio::net::TcpListener::bind(&settings.server.address())
        .await
        .context("Failed to bind to server address")?;

    info!("🌐 Server starting on {} with production optimizations", settings.server.address());

    // Start background tasks
    start_background_tasks(app_state.clone(), &production_config).await?;

    // Start metrics server if enabled
    if production_config.performance.metrics_enabled {
        start_metrics_server(&settings).await?;
    }

    // Run server with graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("Server error")?;

    info!("👋 Aerugo Docker Registry shutdown completed");
    Ok(())
}

/// Start background tasks cho production monitoring
async fn start_background_tasks(
    app_state: AppState,
    config: &ProductionSettings,
) -> anyhow::Result<()> {
    // Cache cleanup task
    let cache_cleanup = app_state.cache.clone();
    let cleanup_interval = Duration::from_secs(config.health_check_interval);
    
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(cleanup_interval);
        loop {
            interval.tick().await;
            if let Some(cache) = &cache_cleanup {
                if let Err(e) = cache.cleanup_expired().await {
                    warn!("Cache cleanup error: {}", e);
                }
            }
        }
    });

    // Health check task
    let health_state = app_state.clone();
    let health_interval = Duration::from_secs(config.health_check_interval);
    
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(health_interval);
        loop {
            interval.tick().await;
            
            // Check database health
            if let Err(e) = sqlx::query("SELECT 1").execute(&health_state.db_pool).await {
                warn!("Database health check failed: {}", e);
            }
            
            // Check cache health  
            if let Some(cache) = &health_state.cache {
                if let Err(e) = cache.health_check().await {
                    warn!("Cache health check failed: {}", e);
                }
            }
        }
    });

    info!("✅ Background tasks started - cache cleanup & health monitoring");
    Ok(())
}

/// Start metrics server cho production monitoring
async fn start_metrics_server(settings: &Settings) -> anyhow::Result<()> {
    // TODO: Fix metrics dependencies
    warn!("Metrics server disabled - need to fix prometheus dependencies");
    Ok(())
    
    /* 
    use axum::{routing::get, Router};
    use metrics_prometheus::PrometheusHandle;
    
    let recorder = metrics_prometheus::PrometheusBuilder::new()
        .build_recorder();
    let handle = recorder.handle();
    
    if let Err(e) = metrics::set_global_recorder(Box::new(recorder)) {
        warn!("Failed to set metrics recorder: {}", e);
        return Ok(());
    }

    let metrics_app = Router::new()
        .route("/metrics", get(move || async move { handle.render() }));

    // Start metrics server on different port
    let metrics_addr = format!("{}:9090", 
        settings.server.address().split(':').next().unwrap_or("127.0.0.1"));
    
    let metrics_listener = tokio::net::TcpListener::bind(&metrics_addr).await?;
    
    tokio::spawn(async move {
        if let Err(e) = axum::serve(metrics_listener, metrics_app).await {
            warn!("Metrics server error: {}", e);
        }
    });

    info!("📊 Metrics server started on {}", metrics_addr);
    Ok(())
    */
}

/// Graceful shutdown signal handler
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("🛑 Received Ctrl+C, starting graceful shutdown");
        },
        _ = terminate => {
            info!("🛑 Received terminate signal, starting graceful shutdown");
        },
    }
}
