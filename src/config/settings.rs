use anyhow::{Context, Result};
use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use url::Url;
use validator::Validate;

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
    #[validate]
    pub email: EmailSettings,
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

impl ServerSettings {
    pub fn address(&self) -> String {
        // Use bind_address directly as it already contains host:port
        self.bind_address.clone()
    }
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

impl DatabaseSettings {
    pub fn url(&self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.database_name
        )
    }
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

impl StorageSettings {
    pub fn bucket_name(&self) -> &str {
        &self.bucket
    }
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

        // Create settings from environment variables directly
        // Log that we're using environment variables only
        eprintln!("Loading configuration from environment variables and .env file");
        eprintln!("LISTEN_ADDRESS: {:?}", std::env::var("LISTEN_ADDRESS"));
        eprintln!("DATABASE_URL: {:?}", std::env::var("DATABASE_URL").map(|_| "[HIDDEN]"));
        
        let settings = Settings {
            server: ServerSettings {
                bind_address: std::env::var("LISTEN_ADDRESS").unwrap_or_else(|_| "127.0.0.1:3000".to_string()),
                port: 3000, // Port is now parsed from LISTEN_ADDRESS
                api_prefix: std::env::var("API_PREFIX").unwrap_or_else(|_| "/api/v1".to_string()),
                log_level: std::env::var("LOG_LEVEL").unwrap_or_else(|_| "debug".to_string()),
            },
            database: {
                // If DATABASE_URL is set, parse it to extract components
                if let Ok(database_url) = std::env::var("DATABASE_URL") {
                    if let Ok(db_url) = url::Url::parse(&database_url) {
                        let host = db_url.host_str().unwrap_or("localhost").to_string();
                        let port = db_url.port().unwrap_or(5432);
                        let username = db_url.username().to_string();
                        let password = Secret::new(db_url.password().unwrap_or("").to_string());
                        let database_name = db_url.path().trim_start_matches('/').to_string();
                        
                        DatabaseSettings {
                            host,
                            port,
                            username,
                            password,
                            database_name,
                            require_ssl: std::env::var("DATABASE_REQUIRE_SSL")
                                .ok()
                                .and_then(|s| s.parse().ok())
                                .unwrap_or(false),
                            min_connections: std::env::var("DATABASE_MIN_CONNECTIONS")
                                .ok()
                                .and_then(|c| c.parse().ok())
                                .unwrap_or(5),
                            max_connections: std::env::var("DATABASE_MAX_CONNECTIONS")
                                .ok()
                                .and_then(|c| c.parse().ok())
                                .unwrap_or(20),
                        }
                    } else {
                        // Fallback to individual settings if URL can't be parsed
                        DatabaseSettings {
                            host: std::env::var("DATABASE_HOST").unwrap_or_else(|_| "localhost".to_string()),
                            port: std::env::var("DATABASE_PORT")
                                .ok()
                                .and_then(|p| p.parse().ok())
                                .unwrap_or(5432),
                            username: std::env::var("DATABASE_USERNAME").unwrap_or_else(|_| "aerugo".to_string()),
                            password: Secret::new(std::env::var("DATABASE_PASSWORD").unwrap_or_else(|_| "1".to_string())),
                            database_name: std::env::var("DATABASE_NAME").unwrap_or_else(|_| "aerugo_dev".to_string()),
                            require_ssl: std::env::var("DATABASE_REQUIRE_SSL")
                                .ok()
                                .and_then(|s| s.parse().ok())
                                .unwrap_or(false),
                            min_connections: std::env::var("DATABASE_MIN_CONNECTIONS")
                                .ok()
                                .and_then(|c| c.parse().ok())
                                .unwrap_or(5),
                            max_connections: std::env::var("DATABASE_MAX_CONNECTIONS")
                                .ok()
                                .and_then(|c| c.parse().ok())
                                .unwrap_or(20),
                        }
                    }
                } else {
                    // Use individual settings if DATABASE_URL is not set
                    DatabaseSettings {
                        host: std::env::var("DATABASE_HOST").unwrap_or_else(|_| "localhost".to_string()),
                        port: std::env::var("DATABASE_PORT")
                            .ok()
                            .and_then(|p| p.parse().ok())
                            .unwrap_or(5432),
                        username: std::env::var("DATABASE_USERNAME").unwrap_or_else(|_| "aerugo".to_string()),
                        password: Secret::new(std::env::var("DATABASE_PASSWORD").unwrap_or_else(|_| "1".to_string())),
                        database_name: std::env::var("DATABASE_NAME").unwrap_or_else(|_| "aerugo_dev".to_string()),
                        require_ssl: std::env::var("DATABASE_REQUIRE_SSL")
                            .ok()
                            .and_then(|s| s.parse().ok())
                            .unwrap_or(false),
                        min_connections: std::env::var("DATABASE_MIN_CONNECTIONS")
                            .ok()
                            .and_then(|c| c.parse().ok())
                            .unwrap_or(5),
                        max_connections: std::env::var("DATABASE_MAX_CONNECTIONS")
                            .ok()
                            .and_then(|c| c.parse().ok())
                            .unwrap_or(20),
                    }
                }
            },
            storage: StorageSettings {
                endpoint: std::env::var("STORAGE_ENDPOINT").unwrap_or_else(|_| "http://localhost:9000".to_string()),
                region: std::env::var("STORAGE_REGION").unwrap_or_else(|_| "us-east-1".to_string()),
                bucket: std::env::var("STORAGE_BUCKET").unwrap_or_else(|_| "aerugo".to_string()),
                access_key_id: Secret::new(std::env::var("STORAGE_ACCESS_KEY_ID").unwrap_or_else(|_| "minioadmin".to_string())),
                secret_access_key: Secret::new(std::env::var("STORAGE_SECRET_ACCESS_KEY").unwrap_or_else(|_| "minioadmin".to_string())),
                use_path_style: std::env::var("STORAGE_USE_PATH_STYLE")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(true),
            },
            cache: CacheSettings {
                redis_url: std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string()),
                pool_size: std::env::var("REDIS_POOL_SIZE")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(10),
                ttl_seconds: std::env::var("REDIS_TTL_SECONDS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(3600),
            },
            auth: AuthSettings {
                jwt_secret: Secret::new(std::env::var("JWT_SECRET").unwrap_or_else(|_| "your-super-secret-key".to_string())),
                jwt_expiration_seconds: std::env::var("JWT_EXPIRATION_SECONDS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(3600),
                refresh_token_expiration_seconds: std::env::var("REFRESH_TOKEN_EXPIRATION_SECONDS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(604800),
            },
            email: EmailSettings {
                smtp_host: std::env::var("SMTP_HOST").unwrap_or_else(|_| "localhost".to_string()),
                smtp_port: std::env::var("SMTP_PORT")
                    .ok()
                    .and_then(|p| p.parse().ok())
                    .unwrap_or(587),
                smtp_username: std::env::var("SMTP_USERNAME").unwrap_or_else(|_| "".to_string()),
                smtp_password: Secret::new(std::env::var("SMTP_PASSWORD").unwrap_or_else(|_| "".to_string())),
                from_email: std::env::var("FROM_EMAIL").unwrap_or_else(|_| "noreply@localhost".to_string()),
                from_name: std::env::var("FROM_NAME").unwrap_or_else(|_| "Aerugo ".to_string()),
                use_tls: std::env::var("SMTP_USE_TLS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(true),
                test_mode: std::env::var("EMAIL_TEST_MODE")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(cfg!(debug_assertions)), // Use test mode in development by default
                test_email_file: std::env::var("EMAIL_TEST_FILE").ok(),
            },
        };

        settings
            .validate_all()
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
        self.email.validate()?;
        Ok(())
    }

    // Get base URL for server
    pub fn server_url(&self) -> String {
        format!("http://{}:{}", self.server.bind_address, self.server.port)
    }
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        let ssl_mode = if self.require_ssl {
            "require"
        } else {
            "prefer"
        };
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

#[derive(Debug, Deserialize, Clone, Validate)]
pub struct EmailSettings {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: Secret<String>,
    pub from_email: String,
    pub from_name: String,
    pub use_tls: bool,
    // For testing environment
    pub test_mode: bool,
    pub test_email_file: Option<String>,
}
