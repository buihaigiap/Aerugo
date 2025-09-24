use axum::http::{StatusCode, HeaderMap};
use axum_extra::headers::{authorization::Bearer, Authorization};
use axum_extra::TypedHeader;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::cache::RegistryCache;
use crate::models::api_key::ApiKey;
use chrono::{DateTime, Utc};
use sha2::{Sha256, Digest};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user id
    pub exp: usize,  // expiration time
}

pub fn verify_token(token: &str, secret: &[u8]) -> Result<Claims, StatusCode> {
    tracing::debug!("Verifying token: {}", token);

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret),
        &Validation::default()
    ).map_err(|e| {
        tracing::error!("Token verification error: {:?}", e);
        StatusCode::UNAUTHORIZED
    })?;

    tracing::debug!("Token verified successfully for user ID: {}", token_data.claims.sub);
    Ok(token_data.claims)
}

/// Verify token with cache support
pub async fn verify_token_cached(
    token: &str, 
    secret: &[u8], 
    cache: &Arc<RegistryCache>
) -> Result<Claims, StatusCode> {
    // First check cache
    if let Some(auth_entry) = cache.get_auth_token(token).await {
        tracing::debug!("Token found in cache for user ID: {}", auth_entry.user_id);
        return Ok(Claims {
            sub: auth_entry.user_id.to_string(),
            exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize, // Use current time + 24h
        });
    }

    // If not in cache, verify normally
    let claims = verify_token(token, secret)?;
    
    // Cache the verified token
    if let Ok(user_id) = claims.sub.parse::<i64>() {
        let auth_entry = crate::cache::AuthCacheEntry {
            user_id: user_id.to_string(),
            username: format!("user_{}", user_id), // TODO: Get actual username
            email: format!("user_{}@domain.com", user_id), // TODO: Get actual email
            is_admin: false, // TODO: Check actual admin status
        };
        
        if let Err(e) = cache.cache_auth_token(token, auth_entry).await {
            tracing::warn!("Failed to cache auth token: {}", e);
        }
    }

    Ok(claims)
}


/// Extract user ID with API key and JWT dual authentication support  
pub async fn extract_user_id_dual(
    auth: Option<TypedHeader<Authorization<Bearer>>>,
    headers: &HeaderMap,
    secret: &[u8],
    pool: &sqlx::PgPool,
    cache: Option<&Arc<RegistryCache>>,
) -> Result<i64, StatusCode> {
    // Try X-API-Key header first
    let api_key_header = headers
        .get("x-api-key")
        .and_then(|h| h.to_str().ok());
    
    extract_user_id_dual_auth(
        auth, 
        api_key_header, 
        secret, 
        pool, 
        cache
    ).await
}

pub async fn extract_user_id(
    auth: Option<TypedHeader<Authorization<Bearer>>>, 
    secret: &[u8],
) -> Result<i64, StatusCode> {
    let auth = auth.ok_or(StatusCode::UNAUTHORIZED)?;
    let claims = verify_token(auth.token(), secret)?;
    claims
        .sub
        .parse::<i64>()
        .map_err(|_| StatusCode::UNAUTHORIZED)
}

/// Extract user ID with cache support
pub async fn extract_user_id_cached(
    auth: Option<TypedHeader<Authorization<Bearer>>>, 
    secret: &[u8],
    cache: &Arc<RegistryCache>,
) -> Result<i64, StatusCode> {
    let auth = auth.ok_or(StatusCode::UNAUTHORIZED)?;
    let claims = verify_token_cached(auth.token(), secret, cache).await?;
    claims
        .sub
        .parse::<i64>()
        .map_err(|_| StatusCode::UNAUTHORIZED)
}

/// Check user permissions with cache support
pub async fn check_permission_cached(
    user_id: i64,
    repository: &str,
    action: &str, // "pull", "push", "admin"
    cache: &Arc<RegistryCache>,
    pool: &sqlx::PgPool,
) -> Result<bool, StatusCode> {
    // Try cache first
    if let Some(permissions) = cache.get_permissions(&user_id.to_string(), repository).await {
        let has_permission = match action {
            "pull" => permissions.can_read,
            "push" => permissions.can_write,
            "admin" => permissions.can_admin,
            _ => false,
        };
        
        tracing::debug!("Permission check from cache: user={}, repo={}, action={}, allowed={}", 
                       user_id, repository, action, has_permission);
        return Ok(has_permission);
    }

    // If not in cache, check database
    // First get repository info
    let repo_check = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS (
            SELECT 1 FROM repositories r
            JOIN organizations o ON r.organization_id = o.id
            JOIN organization_members om ON r.organization_id = om.organization_id
            WHERE CONCAT(o.name, '/', r.name) = $1 AND om.user_id = $2
        )"
    )
    .bind(repository)
    .bind(user_id)
    .fetch_one(pool)
    .await
    .map_err(|e| {
        tracing::error!("Database permission check failed: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let is_member = repo_check;
    
    // For simplicity, members can pull/push, owners can admin
    let can_pull = is_member;
    let can_push = is_member;
    let can_admin = is_member; // TODO: Add proper role checking

    // Cache the result
    let permission_entry = crate::cache::PermissionCacheEntry {
        can_read: can_pull,
        can_write: can_push,
        can_admin,
        organization_id: Some("org1".to_string()), // TODO: Get actual org ID
    };
    
    if let Err(e) = cache.cache_permissions(&user_id.to_string(), repository, permission_entry).await {
        tracing::warn!("Failed to cache permissions: {}", e);
    }

    let has_permission = match action {
        "pull" => can_pull,
        "push" => can_push, 
        "admin" => can_admin,
        _ => false,
    };

    tracing::debug!("Permission check from database: user={}, repo={}, action={}, allowed={}", 
                   user_id, repository, action, has_permission);
    
    Ok(has_permission)
}

/// Generate a new API key with format ak_<32_hex_chars>
pub fn generate_api_key() -> String {
    let random_part: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();
    format!("ak_{}", random_part)
}

/// Hash an API key using SHA-256
pub fn hash_api_key(api_key: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(api_key.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Verify API key and return user ID (full permissions like JWT)
pub async fn verify_api_key(
    api_key: &str,
    pool: &sqlx::PgPool,
    cache: Option<&Arc<RegistryCache>>,
) -> Result<i64, StatusCode> {
    let key_hash = hash_api_key(api_key);
    
    // Check cache first if available
    if let Some(cache) = cache {
        if let Some(cached_info) = cache.get_api_key_info(&key_hash).await {
            // Update last_used_at in background (fire and forget)
            let pool_clone = pool.clone();
            let key_hash_clone = key_hash.clone();
            tokio::spawn(async move {
                let _ = sqlx::query!(
                    "UPDATE api_keys SET last_used_at = CURRENT_TIMESTAMP WHERE key_hash = $1",
                    key_hash_clone
                )
                .execute(&pool_clone)
                .await;
            });
            
            return Ok(cached_info.user_id);
        }
    }
    
    // Query database
    let api_key_record = sqlx::query_as!(
        ApiKey,
        r#"
        SELECT id, user_id, name, key_hash, last_used_at, expires_at, created_at, updated_at, is_active
        FROM api_keys 
        WHERE key_hash = $1 AND is_active = true
        "#,
        key_hash
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        tracing::error!("Database error verifying API key: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    let api_key_record = api_key_record.ok_or(StatusCode::UNAUTHORIZED)?;
    
    // Check if expired
    if let Some(expires_at) = api_key_record.expires_at {
        let expires_utc = expires_at.and_utc(); // Convert to UTC
        if expires_utc < Utc::now() {
            tracing::warn!("API key expired (id: {})", api_key_record.id);
            return Err(StatusCode::UNAUTHORIZED);
        }
    }
    
    // Update last_used_at
    let _ = sqlx::query!(
        "UPDATE api_keys SET last_used_at = CURRENT_TIMESTAMP WHERE id = $1",
        api_key_record.id
    )
    .execute(pool)
    .await;
    
    // Cache the result if cache is available
    if let Some(cache) = cache {
        let cache_info = crate::cache::ApiKeyCacheEntry {
            user_id: api_key_record.user_id,
            expires_at: api_key_record.expires_at,
        };
        let _ = cache.cache_api_key_info(&key_hash, cache_info).await;
    }
    
    Ok(api_key_record.user_id)
}

/// Extract user ID from either JWT token or API key (simplified, full access)
pub async fn extract_user_id_dual_auth(
    auth: Option<TypedHeader<Authorization<Bearer>>>,
    api_key_header: Option<&str>, // X-API-Key header value
    secret: &[u8],
    pool: &sqlx::PgPool,
    cache: Option<&Arc<RegistryCache>>,
) -> Result<i64, StatusCode> {
    // Try API key first if provided via X-API-Key header
    if let Some(api_key) = api_key_header {
        if api_key.starts_with("ak_") {
            tracing::debug!("Attempting API key authentication");
            return verify_api_key(api_key, pool, cache).await;
        }
    }
    
    // Try Bearer token (could be JWT or API key)
    if let Some(auth) = auth {
        let token = auth.token();
        
        // Check if it's an API key (starts with ak_)
        if token.starts_with("ak_") {
            tracing::debug!("Attempting API key authentication via Bearer");
            return verify_api_key(token, pool, cache).await;
        }
        
        // Otherwise treat as JWT
        tracing::debug!("Attempting JWT authentication");
        if let Some(cache) = cache {
            let claims = verify_token_cached(token, secret, cache).await?;
            let user_id = claims.sub.parse::<i64>().map_err(|_| StatusCode::UNAUTHORIZED)?;
            return Ok(user_id);
        } else {
            let claims = verify_token(token, secret)?;
            let user_id = claims.sub.parse::<i64>().map_err(|_| StatusCode::UNAUTHORIZED)?;
            return Ok(user_id);
        }
    }
    
    Err(StatusCode::UNAUTHORIZED)
}


