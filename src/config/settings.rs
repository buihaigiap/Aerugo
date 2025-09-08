use serde::{Deserialize, Serialize};
use secrecy::{Secret, ExposeSecret};
use validator::Validate;
use std::net::SocketAddr;
use url::Url;
use anyhow::Result;

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
    pub fn load() -> Result<Self, config::ConfigError> {
        // Load .env file if it exists
        dotenv::dotenv().ok();

        let config_builder = config::Config::builder()
            // Load default configuration
            .add_source(config::File::with_name("config/default"))
            // Add configuration from environment variables (i.e. `APP_SERVER__PORT=5001`)
            .add_source(config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"))
            // Optional local configuration file
            .add_source(config::File::with_name("config/local").required(false));

        // Build and convert
        let settings = config_builder.build()?.try_deserialize()?;

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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_settings() {
        let settings = Settings::load().expect("Failed to load settings");
        assert_eq!(settings.server.port, 3000);
        assert_eq!(settings.server.api_prefix, "/api/v1");
    }

    #[test]
    fn test_settings_validation() {
        let settings = Settings::load().expect("Failed to load settings");
        assert!(settings.validate_all().is_ok());
    }
}
