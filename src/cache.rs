// Performance optimization module for Docker Registry
// Implements caching, connection pooling, and production optimizations

use std::sync::Arc;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use tokio::sync::RwLock;
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use redis::{Client as RedisClient, Commands};
use anyhow::Result;

// Authentication cache structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthCacheEntry {
    pub user_id: String,
    pub username: String,
    pub email: String,
    pub is_admin: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionCacheEntry {
    pub can_read: bool,
    pub can_write: bool,
    pub can_admin: bool,
    pub organization_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSessionCache {
    pub user_id: String,
    pub last_activity: u64,
    pub session_data: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyCacheEntry {
    pub user_id: i64,
    pub expires_at: Option<chrono::NaiveDateTime>,
}

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
    // Authentication and permission caches
    auth_token_cache: HashMap<String, CacheEntry<AuthCacheEntry>>,
    api_key_cache: HashMap<String, CacheEntry<String>>, // Store serialized ApiKeyCacheEntry
    permission_cache: HashMap<String, CacheEntry<PermissionCacheEntry>>,
    user_session_cache: HashMap<String, CacheEntry<UserSessionCache>>,
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
    // Authentication cache TTLs
    pub auth_token_ttl: Duration,
    pub permission_ttl: Duration,
    pub session_ttl: Duration,
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
            // Authentication cache TTLs
            auth_token_ttl: Duration::from_secs(900), // 15 minutes
            permission_ttl: Duration::from_secs(300), // 5 minutes
            session_ttl: Duration::from_secs(1800), // 30 minutes
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
            
            // Remove oldest blob metadata entries
            while removed < target_removals && !cache.blob_metadata.is_empty() {
                if let Some(oldest_key) = cache.blob_metadata
                    .iter()
                    .min_by_key(|(_, entry)| entry.created_at)
                    .map(|(k, _)| k.clone()) {
                    cache.blob_metadata.remove(&oldest_key);
                    removed += 1;
                }
            }
            
            // Then remove oldest repository entries
            while removed < target_removals && !cache.repository_cache.is_empty() {
                if let Some(oldest_key) = cache.repository_cache
                    .iter()
                    .min_by_key(|(_, entry)| entry.created_at)
                    .map(|(k, _)| k.clone()) {
                    cache.repository_cache.remove(&oldest_key);
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
                auth_token_count: cache.auth_token_cache.len(),
                permission_count: cache.permission_cache.len(),
                session_count: cache.user_session_cache.len(),
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
    
    // ============ Authentication Caching Methods ============
    
    /// Cache authentication token
    pub async fn cache_auth_token(&self, token: &str, auth_entry: AuthCacheEntry) -> Result<()> {
        // Memory cache
        if self.config.enable_memory {
            let mut cache = self.memory_cache.write().await;
            cache.auth_token_cache.insert(
                token.to_string(),
                CacheEntry::new(auth_entry.clone(), self.config.auth_token_ttl),
            );
        }
        
        // Redis cache
        if let Some(redis) = &self.redis_client {
            if let Ok(mut conn) = redis.get_connection() {
                let redis_key = format!("auth:{}", token);
                let serialized = serde_json::to_string(&auth_entry)?;
                let _: Result<(), _> = conn.set_ex(&redis_key, serialized, self.config.auth_token_ttl.as_secs());
            }
        }
        
        Ok(())
    }
    
    /// Get cached authentication token
    pub async fn get_auth_token(&self, token: &str) -> Option<AuthCacheEntry> {
        // Try memory cache first
        if self.config.enable_memory {
            let cache = self.memory_cache.read().await;
            if let Some(entry) = cache.auth_token_cache.get(token) {
                if !entry.is_expired() {
                    return Some(entry.data.clone());
                }
            }
        }
        
        // Try Redis cache
        if let Some(redis) = &self.redis_client {
            if let Ok(mut conn) = redis.get_connection() {
                let redis_key = format!("auth:{}", token);
                if let Ok(data) = conn.get::<_, String>(&redis_key) {
                    if let Ok(auth_entry) = serde_json::from_str::<AuthCacheEntry>(&data) {
                        // Update memory cache
                        if self.config.enable_memory {
                            let mut cache = self.memory_cache.write().await;
                            cache.auth_token_cache.insert(
                                token.to_string(),
                                CacheEntry::new(auth_entry.clone(), self.config.auth_token_ttl),
                            );
                        }
                        
                        return Some(auth_entry);
                    }
                }
            }
        }
        
        None
    }
    
    /// Cache user permissions for repository
    pub async fn cache_permissions(&self, user_id: &str, repo_name: &str, permissions: PermissionCacheEntry) -> Result<()> {
        let cache_key = format!("{}:{}", user_id, repo_name);
        
        // Memory cache
        if self.config.enable_memory {
            let mut cache = self.memory_cache.write().await;
            cache.permission_cache.insert(
                cache_key.clone(),
                CacheEntry::new(permissions.clone(), self.config.permission_ttl),
            );
        }
        
        // Redis cache
        if let Some(redis) = &self.redis_client {
            if let Ok(mut conn) = redis.get_connection() {
                let redis_key = format!("perms:{}", cache_key);
                let serialized = serde_json::to_string(&permissions)?;
                let _: Result<(), _> = conn.set_ex(&redis_key, serialized, self.config.permission_ttl.as_secs());
            }
        }
        
        Ok(())
    }
    
    /// Get cached permissions
    pub async fn get_permissions(&self, user_id: &str, repo_name: &str) -> Option<PermissionCacheEntry> {
        let cache_key = format!("{}:{}", user_id, repo_name);
        
        // Try memory cache first
        if self.config.enable_memory {
            let cache = self.memory_cache.read().await;
            if let Some(entry) = cache.permission_cache.get(&cache_key) {
                if !entry.is_expired() {
                    return Some(entry.data.clone());
                }
            }
        }
        
        // Try Redis cache
        if let Some(redis) = &self.redis_client {
            if let Ok(mut conn) = redis.get_connection() {
                let redis_key = format!("perms:{}", cache_key);
                if let Ok(data) = conn.get::<_, String>(&redis_key) {
                    if let Ok(permissions) = serde_json::from_str::<PermissionCacheEntry>(&data) {
                        // Update memory cache
                        if self.config.enable_memory {
                            let mut cache = self.memory_cache.write().await;
                            cache.permission_cache.insert(
                                cache_key,
                                CacheEntry::new(permissions.clone(), self.config.permission_ttl),
                            );
                        }
                        
                        return Some(permissions);
                    }
                }
            }
        }
        
        None
    }
    
    /// Cache user session data
    pub async fn cache_session(&self, session_id: &str, session_data: UserSessionCache) -> Result<()> {
        // Memory cache
        if self.config.enable_memory {
            let mut cache = self.memory_cache.write().await;
            cache.user_session_cache.insert(
                session_id.to_string(),
                CacheEntry::new(session_data.clone(), self.config.session_ttl),
            );
        }
        
        // Redis cache
        if let Some(redis) = &self.redis_client {
            if let Ok(mut conn) = redis.get_connection() {
                let redis_key = format!("session:{}", session_id);
                let serialized = serde_json::to_string(&session_data)?;
                let _: Result<(), _> = conn.set_ex(&redis_key, serialized, self.config.session_ttl.as_secs());
            }
        }
        
        Ok(())
    }
    
    /// Get cached session data
    pub async fn get_session(&self, session_id: &str) -> Option<UserSessionCache> {
        // Try memory cache first
        if self.config.enable_memory {
            let cache = self.memory_cache.read().await;
            if let Some(entry) = cache.user_session_cache.get(session_id) {
                if !entry.is_expired() {
                    return Some(entry.data.clone());
                }
            }
        }
        
        // Try Redis cache
        if let Some(redis) = &self.redis_client {
            if let Ok(mut conn) = redis.get_connection() {
                let redis_key = format!("session:{}", session_id);
                if let Ok(data) = conn.get::<_, String>(&redis_key) {
                    if let Ok(session_data) = serde_json::from_str::<UserSessionCache>(&data) {
                        // Update memory cache
                        if self.config.enable_memory {
                            let mut cache = self.memory_cache.write().await;
                            cache.user_session_cache.insert(
                                session_id.to_string(),
                                CacheEntry::new(session_data.clone(), self.config.session_ttl),
                            );
                        }
                        
                        return Some(session_data);
                    }
                }
            }
        }
        
        None
    }
    
    /// Invalidate authentication cache entries
    pub async fn invalidate_auth_token(&self, token: &str) -> Result<()> {
        // Remove from memory cache
        if self.config.enable_memory {
            let mut cache = self.memory_cache.write().await;
            cache.auth_token_cache.remove(token);
        }
        
        // Remove from Redis cache
        if let Some(redis) = &self.redis_client {
            if let Ok(mut conn) = redis.get_connection() {
                let redis_key = format!("auth:{}", token);
                let _: Result<(), _> = conn.del(&redis_key);
            }
        }
        
        Ok(())
    }
    
    /// Invalidate all permissions for a user
    pub async fn invalidate_user_permissions(&self, user_id: &str) -> Result<()> {
        // Remove from memory cache
        if self.config.enable_memory {
            let mut cache = self.memory_cache.write().await;
            let keys_to_remove: Vec<String> = cache.permission_cache.keys()
                .filter(|key| key.starts_with(&format!("{}:", user_id)))
                .cloned()
                .collect();
            for key in keys_to_remove {
                cache.permission_cache.remove(&key);
            }
        }
        
        // Remove from Redis cache
        if let Some(redis) = &self.redis_client {
            if let Ok(mut conn) = redis.get_connection() {
                let pattern = format!("perms:{}:*", user_id);
                let keys: Vec<String> = conn.keys(&pattern).unwrap_or_default();
                if !keys.is_empty() {
                    let _: Result<(), _> = conn.del(&keys);
                }
            }
        }
        
        Ok(())
    }
    
    // ============ End Authentication Caching Methods ============

    /// Invalidate manifest cache entry
    pub async fn invalidate_manifest(&self, cache_key: &str) -> Result<()> {
        // Remove from memory cache
        if self.config.enable_memory {
            let mut cache = self.memory_cache.write().await;
            cache.manifest_cache.remove(cache_key);
        }
        
        // Remove from Redis cache
        if let Some(redis) = &self.redis_client {
            if let Ok(mut conn) = redis.get_connection() {
                let redis_key = format!("manifest:{}", cache_key.strip_prefix("manifest:").unwrap_or(cache_key));
                let _: Result<(), _> = conn.del(&redis_key);
            }
        }
        
        Ok(())
    }
    
    /// Invalidate tags cache for a repository
    pub async fn invalidate_tags(&self, repository: &str) -> Result<()> {
        // Remove from memory cache
        if self.config.enable_memory {
            let mut cache = self.memory_cache.write().await;
            cache.tag_cache.remove(repository);
        }
        
        // Remove from Redis cache
        if let Some(redis) = &self.redis_client {
            if let Ok(mut conn) = redis.get_connection() {
                let redis_key = format!("tags:{}", repository);
                let _: Result<(), _> = conn.del(&redis_key);
            }
        }
        
        Ok(())
    }
    
    /// Invalidate repository cache
    pub async fn invalidate_repositories(&self) -> Result<()> {
        // Remove from memory cache
        if self.config.enable_memory {
            let mut cache = self.memory_cache.write().await;
            cache.repository_cache.clear();
        }
        
        // Remove from Redis cache
        if let Some(redis) = &self.redis_client {
            if let Ok(mut conn) = redis.get_connection() {
                let keys: Vec<String> = conn.keys("repos:*").unwrap_or_default();
                if !keys.is_empty() {
                    let _: Result<(), _> = conn.del(&keys);
                }
            }
        }
        
        Ok(())
    }

    /// Cache OTP code for password reset
    pub async fn cache_otp_code(&self, email: &str, otp_code: &str, ttl: Duration) -> Result<()> {
        if self.config.enable_memory {
            let mut cache = self.memory_cache.write().await;
            let cache_key = format!("otp:reset:{}", email);
            cache.user_session_cache.insert(
                cache_key,
                CacheEntry::new(UserSessionCache {
                    user_id: email.to_string(), // Using email as user_id for OTP
                    last_activity: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
                    session_data: {
                        let mut map = HashMap::new();
                        map.insert("otp_code".to_string(), otp_code.to_string());
                        map
                    },
                }, ttl),
            );
        }

        if self.config.enable_redis && self.redis_client.is_some() {
            if let Some(redis) = &self.redis_client {
                if let Ok(mut conn) = redis.get_connection() {
                    let redis_key = format!("otp:reset:{}", email);
                    let _: Result<(), _> = conn.set_ex(&redis_key, otp_code, ttl.as_secs() as u64);
                }
            }
        }

        Ok(())
    }

    /// Get cached OTP code
    pub async fn get_otp_code(&self, email: &str) -> Option<String> {
        let cache_key = format!("otp:reset:{}", email);
        
        if self.config.enable_memory {
            let cache = self.memory_cache.read().await;
            if let Some(entry) = cache.user_session_cache.get(&cache_key) {
                if !entry.is_expired() {
                    if let Some(otp_code) = entry.data.session_data.get("otp_code") {
                        return Some(otp_code.clone());
                    }
                }
            }
        }

        if self.config.enable_redis && self.redis_client.is_some() {
            if let Some(redis) = &self.redis_client {
                if let Ok(mut conn) = redis.get_connection() {
                    if let Ok(otp_code) = conn.get::<_, String>(&cache_key) {
                        return Some(otp_code);
                    }
                }
            }
        }

        None
    }

    /// Remove OTP code (after use)
    pub async fn remove_otp_code(&self, email: &str) -> Result<()> {
        let cache_key = format!("otp:reset:{}", email);
        
        if self.config.enable_memory {
            let mut cache = self.memory_cache.write().await;
            cache.user_session_cache.remove(&cache_key);
        }

        if self.config.enable_redis && self.redis_client.is_some() {
            if let Some(redis) = &self.redis_client {
                if let Ok(mut conn) = redis.get_connection() {
                    let _: Result<(), _> = conn.del(&cache_key);
                }
            }
        }

        Ok(())
    }
    
    /// Cache API key information  
    pub async fn cache_api_key_info(&self, key_hash: &str, api_key_entry: ApiKeyCacheEntry) -> Result<()> {
        let cache_key = format!("api_key:{}", key_hash);
        
        // Memory cache - store serialized string for API keys
        if self.config.enable_memory {
            let mut cache = self.memory_cache.write().await;
            let serialized = serde_json::to_string(&api_key_entry)?;
            cache.api_key_cache.insert(
                cache_key.clone(),
                CacheEntry::new(serialized, self.config.auth_token_ttl),
            );
        }
        
        // Redis cache
        if let Some(redis) = &self.redis_client {
            if let Ok(mut conn) = redis.get_connection() {
                let serialized = serde_json::to_string(&api_key_entry)?;
                let _: Result<(), _> = conn.set_ex(&cache_key, serialized, self.config.auth_token_ttl.as_secs());
            }
        }
        
        Ok(())
    }
    
    /// Get cached API key information
    pub async fn get_api_key_info(&self, key_hash: &str) -> Option<ApiKeyCacheEntry> {
        let cache_key = format!("api_key:{}", key_hash);
        
        // Try memory cache first
        if self.config.enable_memory {
            let cache = self.memory_cache.read().await;
            if let Some(entry) = cache.api_key_cache.get(&cache_key) {
                if !entry.is_expired() {
                    // entry.data is already a String for API key cache
                    if let Ok(api_key_entry) = serde_json::from_str::<ApiKeyCacheEntry>(&entry.data) {
                        return Some(api_key_entry);
                    }
                }
            }
        }
        
        // Try Redis cache
        if let Some(redis) = &self.redis_client {
            if let Ok(mut conn) = redis.get_connection() {
                if let Ok(data) = conn.get::<_, String>(&cache_key) {
                    if let Ok(api_key_entry) = serde_json::from_str::<ApiKeyCacheEntry>(&data) {
                        // Update memory cache
                        if self.config.enable_memory {
                            let mut cache = self.memory_cache.write().await;
                            cache.api_key_cache.insert(
                                cache_key,
                                CacheEntry::new(data, self.config.auth_token_ttl),
                            );
                        }
                        
                        return Some(api_key_entry);
                    }
                }
            }
        }
        
        None
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
    pub auth_token_count: usize,
    pub permission_count: usize,
    pub session_count: usize,
}
