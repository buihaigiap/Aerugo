use aerugo::config::settings::Settings;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set required environment variables for testing
    env::set_var("LISTEN_ADDRESS", "127.0.0.1:8080");
    env::set_var("LOG_LEVEL", "debug");
    env::set_var("DATABASE_URL", "postgresql://test:test@localhost:5433/test");
    env::set_var("S3_ENDPOINT", "http://localhost:9001");
    env::set_var("S3_BUCKET", "test-bucket");
    env::set_var("S3_ACCESS_KEY", "test-access");
    env::set_var("S3_SECRET_KEY", "test-secret");
    env::set_var("S3_REGION", "us-east-1");
    env::set_var("REDIS_URL", "redis://localhost:6380");
    env::set_var("JWT_SECRET", "test-jwt-secret");

    println!("Loading configuration from environment variables...");
    
    let settings = Settings::load()?;
    
    println!("✅ Configuration loaded successfully!");
    println!();
    println!("Server Configuration:");
    println!("  Bind Address: {}", settings.server.bind_address);
    println!("  Port: {}", settings.server.port);
    println!("  API Prefix: {}", settings.server.api_prefix);
    println!();
    println!("Database Configuration:");
    println!("  Host: {}", settings.database.host);
    println!("  Port: {}", settings.database.port);
    println!("  Username: {}", settings.database.username);
    println!("  Database: {}", settings.database.database_name);
    println!("  SSL Required: {}", settings.database.require_ssl);
    println!("  Min Connections: {}", settings.database.min_connections);
    println!("  Max Connections: {}", settings.database.max_connections);
    println!();
    println!("Storage Configuration:");
    println!("  Endpoint: {}", settings.storage.endpoint);
    println!("  Region: {}", settings.storage.region);
    println!("  Bucket: {}", settings.storage.bucket);
    println!("  Use Path Style: {}", settings.storage.use_path_style);
    println!();
    println!("Cache Configuration:");
    println!("  Redis URL: {}", settings.cache.redis_url);
    println!("  Pool Size: {}", settings.cache.pool_size);
    println!("  TTL Seconds: {}", settings.cache.ttl_seconds);
    println!();
    println!("Auth Configuration:");
    println!("  JWT Expiration: {} seconds", settings.auth.jwt_expiration_seconds);
    println!("  Refresh Token Expiration: {} seconds", settings.auth.refresh_token_expiration_seconds);
    
    println!();
    println!("🎉 All configuration loaded from environment variables successfully!");
    println!("No default configuration files were used.");
    
    Ok(())
}
