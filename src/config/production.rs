use serde::{Deserialize, Serialize};
// use std::time::Duration; // Removed unused import

/// Production Redis caching configuration for high-performance registry
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RedisConfig {
    /// Redis connection URL
    pub url: String,
    /// Maximum number of connections in pool  
    pub max_connections: u32,
    /// Connection timeout (seconds)
    pub connection_timeout: u64,
    /// Default TTL for cache entries (seconds)
    pub default_ttl: u64,
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            url: "redis://127.0.0.1:6379".to_string(),
            max_connections: 50,
            connection_timeout: 10,
            default_ttl: 3600, // 1 hour
        }
    }
}

/// Production caching layers configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CacheConfig {
    /// Redis configuration
    pub redis: RedisConfig,
    /// In-memory cache configuration
    pub memory: MemoryCacheConfig,
    /// Cache performance metrics
    pub metrics_enabled: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MemoryCacheConfig {
    /// Maximum entries in memory cache
    pub max_entries: u64,
    /// TTL cho manifest cache (seconds)
    pub manifest_ttl: u64,
    /// TTL cho blob metadata cache (seconds)
    pub blob_metadata_ttl: u64,
    /// TTL cho repository cache (seconds)
    pub repository_ttl: u64,
    /// TTL cho tag cache (seconds)
    pub tag_ttl: u64,
}

impl Default for MemoryCacheConfig {
    fn default() -> Self {
        Self {
            max_entries: 10000,
            manifest_ttl: 1800,      // 30 minutes
            blob_metadata_ttl: 3600, // 1 hour  
            repository_ttl: 7200,    // 2 hours
            tag_ttl: 900,            // 15 minutes
        }
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            redis: RedisConfig::default(),
            memory: MemoryCacheConfig::default(),
            metrics_enabled: true,
        }
    }
}

/// Production database connection pool settings
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DatabasePoolConfig {
    /// Maximum connections in pool
    pub max_connections: u32,
    /// Minimum idle connections
    pub min_connections: u32,
    /// Connection timeout (seconds)
    pub connect_timeout: u64,
    /// Maximum lifetime of connection (seconds)
    pub max_lifetime: u64,
    /// Idle timeout (seconds)
    pub idle_timeout: u64,
}

impl Default for DatabasePoolConfig {
    fn default() -> Self {
        Self {
            max_connections: 100,
            min_connections: 10,
            connect_timeout: 10,
            max_lifetime: 3600,
            idle_timeout: 600,
        }
    }
}

/// Production performance configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PerformanceConfig {
    /// Request timeout (seconds)
    pub request_timeout: u64,
    /// Maximum concurrent requests per client
    pub max_concurrent_requests: u32,
    /// Enable request/response compression
    pub compression_enabled: bool,
    /// Enable streaming for large blobs
    pub streaming_enabled: bool,
    /// Chunk size for streaming (bytes)
    pub streaming_chunk_size: usize,
    /// Enable metrics collection
    pub metrics_enabled: bool,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            request_timeout: 300,           // 5 minutes
            max_concurrent_requests: 1000,
            compression_enabled: true,
            streaming_enabled: true,
            streaming_chunk_size: 8192,     // 8KB chunks
            metrics_enabled: true,
        }
    }
}

/// Complete production configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProductionSettings {
    /// Cache configuration
    pub cache: CacheConfig,
    /// Database pool configuration  
    pub database_pool: DatabasePoolConfig,
    /// Performance configuration
    pub performance: PerformanceConfig,
    /// Health check interval (seconds)
    pub health_check_interval: u64,
}

impl Default for ProductionSettings {
    fn default() -> Self {
        Self {
            cache: CacheConfig::default(),
            database_pool: DatabasePoolConfig::default(),
            performance: PerformanceConfig::default(),
            health_check_interval: 30,
        }
    }
}

impl ProductionSettings {
    /// Load production settings from environment or file
    pub fn load() -> anyhow::Result<Self> {
        // Try loading from environment variables first
        if let Ok(config) = envy::from_env::<ProductionSettings>() {
            return Ok(config);
        }

        // Fallback to default configuration
        tracing::warn!("Using default production settings. Consider setting environment variables for production deployment.");
        Ok(ProductionSettings::default())
    }

    /// Create Redis connection URL with authentication if available
    pub fn redis_url_with_auth(&self, password: Option<&str>) -> String {
        match password {
            Some(pwd) => {
                let url = &self.cache.redis.url;
                if url.contains("://") {
                    let parts: Vec<&str> = url.splitn(2, "://").collect();
                    format!("{}://default:{}@{}", parts[0], pwd, parts[1].trim_start_matches("//"))
                } else {
                    format!("redis://default:{}@{}", pwd, url)
                }
            }
            None => self.cache.redis.url.clone(),
        }
    }

    /// Validate production configuration
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.cache.redis.max_connections == 0 {
            anyhow::bail!("Redis max_connections must be greater than 0");
        }

        if self.database_pool.max_connections == 0 {
            anyhow::bail!("Database max_connections must be greater than 0");
        }

        if self.performance.streaming_chunk_size == 0 {
            anyhow::bail!("Streaming chunk size must be greater than 0");
        }

        Ok(())
    }
}

/// Environment variable keys cho production config
pub mod env_keys {
    pub const REDIS_URL: &str = "REDIS_URL";
    pub const REDIS_PASSWORD: &str = "REDIS_PASSWORD";
    pub const REDIS_MAX_CONNECTIONS: &str = "REDIS_MAX_CONNECTIONS";
    
    pub const CACHE_METRICS_ENABLED: &str = "CACHE_METRICS_ENABLED";
    pub const MEMORY_CACHE_MAX_ENTRIES: &str = "MEMORY_CACHE_MAX_ENTRIES";
    
    pub const DB_POOL_MAX_CONNECTIONS: &str = "DATABASE_MAX_CONNECTIONS";
    pub const DB_POOL_MIN_CONNECTIONS: &str = "DATABASE_MIN_CONNECTIONS";
    
    pub const PERFORMANCE_REQUEST_TIMEOUT: &str = "REQUEST_TIMEOUT";
    pub const PERFORMANCE_MAX_CONCURRENT: &str = "MAX_CONCURRENT_REQUESTS";
    pub const PERFORMANCE_COMPRESSION: &str = "COMPRESSION_ENABLED";
    pub const PERFORMANCE_STREAMING: &str = "STREAMING_ENABLED";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_production_settings() {
        let settings = ProductionSettings::default();
        assert!(settings.validate().is_ok());
        assert_eq!(settings.cache.redis.max_connections, 50);
        assert_eq!(settings.database_pool.max_connections, 100);
        assert!(settings.performance.compression_enabled);
    }

    #[test]
    fn test_redis_url_with_auth() {
        let settings = ProductionSettings::default();
        let url_with_auth = settings.redis_url_with_auth(Some("password123"));
        assert!(url_with_auth.contains("password123"));
    }
}
