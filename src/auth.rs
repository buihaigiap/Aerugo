use axum::http::StatusCode;
use axum_extra::headers::{authorization::Bearer, Authorization};
use axum_extra::TypedHeader;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::cache::RegistryCache;
use chrono::{DateTime, Utc};

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


