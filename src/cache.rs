// Performance optimization module for Docker Registry
// Implements caching, connection pooling, and production optimizations

use std::sync::Arc;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use tokio::sync::{RwLock, Mutex};
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use redis::{Client as RedisClient, Connection, Commands};
use anyhow::{Result, Context};

/// Cache layer for Docker Registry operations
#[derive(Clone)]
pub struct RegistryCache {
    redis_client: Option<RedisClient>,
    memory_cache: Arc<RwLock<MemoryCache>>,
    config: CacheConfig,
}

/// In-memory cache for high-frequency data
#[derive(Default)]
struct MemoryCache {
    manifest_cache: HashMap<String, CacheEntry<Bytes>>,
    blob_metadata: HashMap<String, CacheEntry<BlobCacheMetadata>>,
    repository_cache: HashMap<String, CacheEntry<Vec<String>>>,
    tag_cache: HashMap<String, CacheEntry<Vec<String>>>,
}

/// Cache entry with TTL
#[derive(Clone)]
struct CacheEntry<T> {
    data: T,
    created_at: Instant,
    ttl: Duration,
}

impl<T> CacheEntry<T> {
    fn new(data: T, ttl: Duration) -> Self {
        Self {
            data,
            created_at: Instant::now(),
            ttl,
        }
    }
    
    fn is_expired(&self) -> bool {
        self.created_at.elapsed() > self.ttl
    }
}

/// Cache configuration
#[derive(Clone, Debug)]
pub struct CacheConfig {
    pub redis_url: Option<String>,
    pub manifest_ttl: Duration,
    pub blob_metadata_ttl: Duration,
    pub repository_ttl: Duration,
    pub tag_ttl: Duration,
    pub max_memory_entries: usize,
    pub enable_redis: bool,
    pub enable_memory: bool,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            redis_url: None,
            manifest_ttl: Duration::from_secs(300), // 5 minutes
            blob_metadata_ttl: Duration::from_secs(600), // 10 minutes
            repository_ttl: Duration::from_secs(60), // 1 minute
            tag_ttl: Duration::from_secs(120), // 2 minutes
            max_memory_entries: 10000,
            enable_redis: true,
            enable_memory: true,
        }
    }
}

/// Blob metadata for caching
#[derive(Clone, Serialize, Deserialize)]
pub struct BlobCacheMetadata {
    pub digest: String,
    pub size: u64,
    pub content_type: Option<String>,
    pub exists: bool,
}

impl RegistryCache {
    /// Create new registry cache
    pub async fn new(config: CacheConfig) -> Result<Self> {
        let redis_client = if config.enable_redis {
            if let Some(redis_url) = &config.redis_url {
                match RedisClient::open(redis_url.as_str()) {
                    Ok(client) => {
                        // Test connection
                        if let Ok(mut conn) = client.get_connection() {
                            let _: String = redis::cmd("PING").query(&mut conn).unwrap_or_default();
                            Some(client)
                        } else {
                            tracing::warn!("Redis connection failed, falling back to memory cache");
                            None
                        }
                    }
                    Err(e) => {
                        tracing::warn!("Redis client creation failed: {}, falling back to memory cache", e);
                        None
                    }
                }
            } else {
                None
            }
        } else {
            None
        };
        
        Ok(Self {
            redis_client,
            memory_cache: Arc::new(RwLock::new(MemoryCache::default())),
            config,
        })
    }
    
    /// Cache manifest data
    pub async fn cache_manifest(&self, key: &str, manifest: Bytes) -> Result<()> {
        // Memory cache
        if self.config.enable_memory {
            let mut cache = self.memory_cache.write().await;
            cache.manifest_cache.insert(
                key.to_string(),
                CacheEntry::new(manifest.clone(), self.config.manifest_ttl),
            );
            
            // Cleanup old entries if needed
            if cache.manifest_cache.len() > self.config.max_memory_entries {
                self.cleanup_memory_cache(&mut cache).await;
            }
        }
        
        // Redis cache
        if let Some(redis) = &self.redis_client {
            if let Ok(mut conn) = redis.get_connection() {
                let redis_key = format!("manifest:{}", key);
                let ttl_secs = self.config.manifest_ttl.as_secs();
                let _: Result<(), _> = conn.set_ex(&redis_key, manifest.as_ref(), ttl_secs);
            }
        }
        
        Ok(())
    }
    
    /// Get cached manifest
    pub async fn get_manifest(&self, key: &str) -> Option<Bytes> {
        // Try memory cache first
        if self.config.enable_memory {
            let cache = self.memory_cache.read().await;
            if let Some(entry) = cache.manifest_cache.get(key) {
                if !entry.is_expired() {
                    return Some(entry.data.clone());
                }
            }
        }
        
        // Try Redis cache
        if let Some(redis) = &self.redis_client {
            if let Ok(mut conn) = redis.get_connection() {
                let redis_key = format!("manifest:{}", key);
                if let Ok(data) = conn.get::<_, Vec<u8>>(&redis_key) {
                    let bytes = Bytes::from(data);
                    
                    // Update memory cache
                    if self.config.enable_memory {
                        let mut cache = self.memory_cache.write().await;
                        cache.manifest_cache.insert(
                            key.to_string(),
                            CacheEntry::new(bytes.clone(), self.config.manifest_ttl),
                        );
                    }
                    
                    return Some(bytes);
                }
            }
        }
        
        None
    }
    
    /// Cache blob metadata
    pub async fn cache_blob_metadata(&self, digest: &str, metadata: BlobCacheMetadata) -> Result<()> {
        // Memory cache
        if self.config.enable_memory {
            let mut cache = self.memory_cache.write().await;
            cache.blob_metadata.insert(
                digest.to_string(),
                CacheEntry::new(metadata.clone(), self.config.blob_metadata_ttl),
            );
        }
        
        // Redis cache
        if let Some(redis) = &self.redis_client {
            if let Ok(mut conn) = redis.get_connection() {
                let redis_key = format!("blob_meta:{}", digest);
                let ttl_secs = self.config.blob_metadata_ttl.as_secs();
                if let Ok(json_data) = serde_json::to_string(&metadata) {
                    let _: Result<(), _> = conn.set_ex(&redis_key, json_data, ttl_secs);
                }
            }
        }
        
        Ok(())
    }
    
    /// Get cached blob metadata
    pub async fn get_blob_metadata(&self, digest: &str) -> Option<BlobCacheMetadata> {
        // Try memory cache first
        if self.config.enable_memory {
            let cache = self.memory_cache.read().await;
            if let Some(entry) = cache.blob_metadata.get(digest) {
                if !entry.is_expired() {
                    return Some(entry.data.clone());
                }
            }
        }
        
        // Try Redis cache
        if let Some(redis) = &self.redis_client {
            if let Ok(mut conn) = redis.get_connection() {
                let redis_key = format!("blob_meta:{}", digest);
                if let Ok(data) = conn.get::<_, String>(&redis_key) {
                    if let Ok(metadata) = serde_json::from_str::<BlobCacheMetadata>(&data) {
                        // Update memory cache
                        if self.config.enable_memory {
                            let mut cache = self.memory_cache.write().await;
                            cache.blob_metadata.insert(
                                digest.to_string(),
                                CacheEntry::new(metadata.clone(), self.config.blob_metadata_ttl),
                            );
                        }
                        
                        return Some(metadata);
                    }
                }
            }
        }
        
        None
    }
    
    /// Cache repository list
    pub async fn cache_repositories(&self, repositories: Vec<String>) -> Result<()> {
        let key = "repositories";
        
        // Memory cache
        if self.config.enable_memory {
            let mut cache = self.memory_cache.write().await;
            cache.repository_cache.insert(
                key.to_string(),
                CacheEntry::new(repositories.clone(), self.config.repository_ttl),
            );
        }
        
        // Redis cache
        if let Some(redis) = &self.redis_client {
            if let Ok(mut conn) = redis.get_connection() {
                let redis_key = format!("repos:{}", key);
                let ttl_secs = self.config.repository_ttl.as_secs();
                if let Ok(json_data) = serde_json::to_string(&repositories) {
                    let _: Result<(), _> = conn.set_ex(&redis_key, json_data, ttl_secs);
                }
            }
        }
        
        Ok(())
    }
    
    /// Get cached repository list
    pub async fn get_repositories(&self) -> Option<Vec<String>> {
        let key = "repositories";
        
        // Try memory cache first
        if self.config.enable_memory {
            let cache = self.memory_cache.read().await;
            if let Some(entry) = cache.repository_cache.get(key) {
                if !entry.is_expired() {
                    return Some(entry.data.clone());
                }
            }
        }
        
        // Try Redis cache
        if let Some(redis) = &self.redis_client {
            if let Ok(mut conn) = redis.get_connection() {
                let redis_key = format!("repos:{}", key);
                if let Ok(data) = conn.get::<_, String>(&redis_key) {
                    if let Ok(repositories) = serde_json::from_str::<Vec<String>>(&data) {
                        // Update memory cache
                        if self.config.enable_memory {
                            let mut cache = self.memory_cache.write().await;
                            cache.repository_cache.insert(
                                key.to_string(),
                                CacheEntry::new(repositories.clone(), self.config.repository_ttl),
                            );
                        }
                        
                        return Some(repositories);
                    }
                }
            }
        }
        
        None
    }
    
    /// Cache tag list for repository
    pub async fn cache_tags(&self, repository: &str, tags: Vec<String>) -> Result<()> {
        // Memory cache
        if self.config.enable_memory {
            let mut cache = self.memory_cache.write().await;
            cache.tag_cache.insert(
                repository.to_string(),
                CacheEntry::new(tags.clone(), self.config.tag_ttl),
            );
        }
        
        // Redis cache
        if let Some(redis) = &self.redis_client {
            if let Ok(mut conn) = redis.get_connection() {
                let redis_key = format!("tags:{}", repository);
                let ttl_secs = self.config.tag_ttl.as_secs();
                if let Ok(json_data) = serde_json::to_string(&tags) {
                    let _: Result<(), _> = conn.set_ex(&redis_key, json_data, ttl_secs);
                }
            }
        }
        
        Ok(())
    }
    
    /// Get cached tag list
    pub async fn get_tags(&self, repository: &str) -> Option<Vec<String>> {
        // Try memory cache first
        if self.config.enable_memory {
            let cache = self.memory_cache.read().await;
            if let Some(entry) = cache.tag_cache.get(repository) {
                if !entry.is_expired() {
                    return Some(entry.data.clone());
                }
            }
        }
        
        // Try Redis cache
        if let Some(redis) = &self.redis_client {
            if let Ok(mut conn) = redis.get_connection() {
                let redis_key = format!("tags:{}", repository);
                if let Ok(data) = conn.get::<_, String>(&redis_key) {
                    if let Ok(tags) = serde_json::from_str::<Vec<String>>(&data) {
                        // Update memory cache
                        if self.config.enable_memory {
                            let mut cache = self.memory_cache.write().await;
                            cache.tag_cache.insert(
                                repository.to_string(),
                                CacheEntry::new(tags.clone(), self.config.tag_ttl),
                            );
                        }
                        
                        return Some(tags);
                    }
                }
            }
        }
        
        None
    }
    
    /// Invalidate cache entries
    pub async fn invalidate(&self, pattern: &str) -> Result<()> {
        // Clear memory cache entries matching pattern
        if self.config.enable_memory {
            let mut cache = self.memory_cache.write().await;
            
            match pattern {
                "manifests" => cache.manifest_cache.clear(),
                "repositories" => cache.repository_cache.clear(),
                key if key.starts_with("tags:") => {
                    let repo = key.strip_prefix("tags:").unwrap_or("");
                    cache.tag_cache.remove(repo);
                }
                _ => {
                    // Remove specific key
                    cache.manifest_cache.remove(pattern);
                    cache.blob_metadata.remove(pattern);
                    cache.repository_cache.remove(pattern);
                    cache.tag_cache.remove(pattern);
                }
            }
        }
        
        // Clear Redis cache entries
        if let Some(redis) = &self.redis_client {
            if let Ok(mut conn) = redis.get_connection() {
                match pattern {
                    "manifests" => {
                        let keys: Vec<String> = conn.keys("manifest:*").unwrap_or_default();
                        if !keys.is_empty() {
                            let _: Result<(), _> = conn.del(&keys);
                        }
                    }
                    "repositories" => {
                        let keys: Vec<String> = conn.keys("repos:*").unwrap_or_default();
                        if !keys.is_empty() {
                            let _: Result<(), _> = conn.del(&keys);
                        }
                    }
                    key if key.starts_with("tags:") => {
                        let _: Result<(), _> = conn.del(format!("tags:{}", key.strip_prefix("tags:").unwrap_or("")));
                    }
                    _ => {
                        // Try to remove specific keys
                        let possible_keys = vec![
                            format!("manifest:{}", pattern),
                            format!("blob_meta:{}", pattern),
                            format!("repos:{}", pattern),
                            format!("tags:{}", pattern),
                        ];
                        for key in possible_keys {
                            let _: Result<(), _> = conn.del(&key);
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Cleanup expired memory cache entries
    async fn cleanup_memory_cache(&self, cache: &mut MemoryCache) {
        // Remove expired manifests
        cache.manifest_cache.retain(|_, entry| !entry.is_expired());
        
        // Remove expired blob metadata
        cache.blob_metadata.retain(|_, entry| !entry.is_expired());
        
        // Remove expired repositories
        cache.repository_cache.retain(|_, entry| !entry.is_expired());
        
        // Remove expired tags
        cache.tag_cache.retain(|_, entry| !entry.is_expired());
        
        // If still over limit, remove oldest entries
        let total_entries = cache.manifest_cache.len() + 
                           cache.blob_metadata.len() + 
                           cache.repository_cache.len() + 
                           cache.tag_cache.len();
        
        if total_entries > self.config.max_memory_entries {
            let target_removals = total_entries - self.config.max_memory_entries;
            let mut removed = 0;
            
            // Remove oldest manifest entries first
            while removed < target_removals && !cache.manifest_cache.is_empty() {
                if let Some(oldest_key) = cache.manifest_cache
                    .iter()
                    .min_by_key(|(_, entry)| entry.created_at)
                    .map(|(k, _)| k.clone()) {
                    cache.manifest_cache.remove(&oldest_key);
                    removed += 1;
                }
            }
        }
    }
    
    /// Get cache statistics
    pub async fn get_stats(&self) -> CacheStats {
        let memory_stats = if self.config.enable_memory {
            let cache = self.memory_cache.read().await;
            MemoryCacheStats {
                manifest_count: cache.manifest_cache.len(),
                blob_metadata_count: cache.blob_metadata.len(),
                repository_count: cache.repository_cache.len(),
                tag_count: cache.tag_cache.len(),
            }
        } else {
            MemoryCacheStats::default()
        };
        
        CacheStats {
            memory_cache: memory_stats,
            redis_connected: self.redis_client.is_some(),
        }
    }
    
    /// Health check for cache system
    pub async fn health_check(&self) -> anyhow::Result<()> {
        // Test Redis connection if available
        if let Some(redis) = &self.redis_client {
            if let Ok(mut conn) = redis.get_connection() {
                let _: String = redis::cmd("PING")
                    .query(&mut conn)
                    .map_err(|e| anyhow::anyhow!("Redis health check failed: {}", e))?;
            }
        }
        
        Ok(())
    }
    
    /// Cleanup expired entries
    pub async fn cleanup_expired(&self) -> anyhow::Result<()> {
        if self.config.enable_memory {
            let mut cache = self.memory_cache.write().await;
            self.cleanup_memory_cache(&mut cache).await;
        }
        Ok(())
    }
    
    /// Get statistics for monitoring
    pub async fn get_statistics(&self) -> CacheStats {
        self.get_stats().await
    }
    
    /// Clear all cache entries
    pub async fn clear(&self) -> anyhow::Result<()> {
        // Clear memory cache
        if self.config.enable_memory {
            let mut cache = self.memory_cache.write().await;
            cache.manifest_cache.clear();
            cache.blob_metadata.clear();
            cache.repository_cache.clear();
            cache.tag_cache.clear();
        }
        
        // Clear Redis cache
        if let Some(redis) = &self.redis_client {
            if let Ok(mut conn) = redis.get_connection() {
                let _: Result<(), _> = redis::cmd("FLUSHDB").query(&mut conn);
            }
        }
        
        Ok(())
    }
}

/// Cache statistics
#[derive(Debug, Serialize)]
pub struct CacheStats {
    pub memory_cache: MemoryCacheStats,
    pub redis_connected: bool,
}

#[derive(Debug, Serialize, Default)]
pub struct MemoryCacheStats {
    pub manifest_count: usize,
    pub blob_metadata_count: usize,
    pub repository_count: usize,
    pub tag_count: usize,
}
