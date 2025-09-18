#[cfg(test)]
mod tests {
    use aerugo::cache::{AuthCacheEntry, PermissionCacheEntry, UserSessionCache, RegistryCache, CacheConfig};
    use std::collections::HashMap;
    use std::time::Duration;

    #[test]
    fn test_auth_cache_structures() {
        // Test AuthCacheEntry structure
        let auth_entry = AuthCacheEntry {
            user_id: "123".to_string(),
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            is_admin: false,
        };

        assert_eq!(auth_entry.user_id, "123");
        assert_eq!(auth_entry.username, "testuser");
        assert_eq!(auth_entry.email, "test@example.com");
        assert!(!auth_entry.is_admin);
    }

    #[test]
    fn test_permission_cache_entry() {
        // Test PermissionCacheEntry structure
        let permission_entry = PermissionCacheEntry {
            can_read: true,
            can_write: false,
            can_admin: false,
            organization_id: Some("org123".to_string()),
        };

        assert!(permission_entry.can_read);
        assert!(!permission_entry.can_write);
        assert!(!permission_entry.can_admin);
        assert_eq!(permission_entry.organization_id, Some("org123".to_string()));
    }

    #[test]
    fn test_user_session_cache() {
        // Test UserSessionCache structure
        let mut session_data = HashMap::new();
        session_data.insert("last_ip".to_string(), "192.168.1.1".to_string());
        
        let session = UserSessionCache {
            user_id: "user123".to_string(),
            last_activity: 1234567890,
            session_data,
        };

        assert_eq!(session.user_id, "user123");
        assert_eq!(session.last_activity, 1234567890);
        assert_eq!(session.session_data.get("last_ip"), Some(&"192.168.1.1".to_string()));
    }

    #[test]
    fn test_cache_config_with_auth_fields() {
        // Test that CacheConfig includes authentication fields
        let config = CacheConfig {
            redis_url: None,
            manifest_ttl: Duration::from_secs(300),
            blob_metadata_ttl: Duration::from_secs(600),
            repository_ttl: Duration::from_secs(60),
            tag_ttl: Duration::from_secs(120),
            auth_token_ttl: Duration::from_secs(900),
            permission_ttl: Duration::from_secs(300),
            session_ttl: Duration::from_secs(1800),
            max_memory_entries: 10000,
            enable_redis: false,
            enable_memory: true,
        };

        assert_eq!(config.auth_token_ttl, Duration::from_secs(900));
        assert_eq!(config.permission_ttl, Duration::from_secs(300));
        assert_eq!(config.session_ttl, Duration::from_secs(1800));
    }

    #[tokio::test]
    async fn test_cache_config_default() {
        // Test default CacheConfig values
        let config = CacheConfig::default();
        
        assert_eq!(config.auth_token_ttl, Duration::from_secs(900)); // 15 minutes
        assert_eq!(config.permission_ttl, Duration::from_secs(300)); // 5 minutes
        assert_eq!(config.session_ttl, Duration::from_secs(1800)); // 30 minutes
        assert!(config.enable_redis);
        assert!(config.enable_memory);
    }
}
