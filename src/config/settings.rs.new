use serde::{Deserialize, Serialize};
use secrecy::{Secret, ExposeSecret};
use validator::Validate;
use std::net::SocketAddr;
use url::Url;
use anyhow::{Result, Context};

#[derive(Debug, Deserialize, Clone, Validate)]
pub struct Settings {
    #[validate]
    pub server: ServerSettings,
    #[validate]
    pub database: DatabaseSettings,
    #[validate]
    pub storage: StorageSettings,
    #[validate]
    pub cache: CacheSettings,
    #[validate]
    pub auth: AuthSettings,
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct ServerSettings {
    #[validate(custom = "validate_socket_addr")]
    pub bind_address: String,
    #[validate(range(min = 1024, max = 65535))]
    pub port: u16,
    pub api_prefix: String,
    pub log_level: String,
}

#[derive(Debug, Deserialize, Clone, Validate)]
pub struct DatabaseSettings {
    pub host: String,
    #[validate(range(min = 1024, max = 65535))]
    pub port: u16,
    pub username: String,
    pub password: Secret<String>,
    pub database_name: String,
    pub require_ssl: bool,
    pub min_connections: u32,
    pub max_connections: u32,
}

#[derive(Debug, Deserialize, Clone, Validate)]
pub struct StorageSettings {
    #[validate(custom = "validate_url")]
    pub endpoint: String,
    pub region: String,
    pub bucket: String,
    pub access_key_id: Secret<String>,
    pub secret_access_key: Secret<String>,
    pub use_path_style: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct CacheSettings {
    pub redis_url: String,
    pub pool_size: u32,
    pub ttl_seconds: u64,
}

#[derive(Debug, Deserialize, Clone, Validate)]
pub struct AuthSettings {
    pub jwt_secret: Secret<String>,
    #[validate(range(min = 300))] // Minimum 5 minutes
    pub jwt_expiration_seconds: u64,
    pub refresh_token_expiration_seconds: u64,
}

impl Settings {
    pub fn load() -> Result<Self> {
        // Load .env file if it exists
        dotenv::dotenv().ok();

        let env = std::env::var("APP_ENV").unwrap_or_else(|_| "development".into());

        let s = config::Config::builder()
            // Default config
            .add_source(config::File::with_name("config/default"))
            // Environment specific config
            .add_source(config::File::with_name(&format!("config/{}", env)).required(false))
            // Local overrides
            .add_source(config::File::with_name("config/local").required(false))
            // Add in settings from environment variables (with a prefix of APP and '__' as separator)
            // E.g. `APP_SERVER__PORT=5001` would set `Settings.server.port`
            .add_source(config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"))
            .build()
            .context("Failed to build configuration")?;

        // Deserialize and validate
        let settings: Settings = s.try_deserialize()
            .context("Failed to deserialize configuration")?;
        
        settings.validate_all()
            .context("Configuration validation failed")?;

        Ok(settings)
    }

    pub fn validate_all(&self) -> Result<(), validator::ValidationErrors> {
        self.validate()?;
        self.server.validate()?;
        self.database.validate()?;
        self.storage.validate()?;
        self.cache.validate()?;
        self.auth.validate()?;
        Ok(())
    }

    // Get base URL for server
    pub fn server_url(&self) -> String {
        format!("http://{}:{}", self.server.bind_address, self.server.port)
    }
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        let ssl_mode = if self.require_ssl { "require" } else { "prefer" };
        format!(
            "postgres://{}:{}@{}:{}/{}?sslmode={}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.database_name,
            ssl_mode
        )
    }
}

fn validate_socket_addr(addr: &str) -> Result<(), validator::ValidationError> {
    addr.parse::<SocketAddr>()
        .map(|_| ())
        .map_err(|_| validator::ValidationError::new("invalid_socket_address"))
}

fn validate_url(url: &str) -> Result<(), validator::ValidationError> {
    Url::parse(url)
        .map(|_| ())
        .map_err(|_| validator::ValidationError::new("invalid_url"))
}
