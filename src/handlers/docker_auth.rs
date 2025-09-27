// Docker Registry Authentication helper functions
use axum::{
    http::{HeaderMap, StatusCode, header::AUTHORIZATION},
    response::{IntoResponse, Response},
    Json,
};
use secrecy::ExposeSecret;
use base64::Engine;
use bcrypt;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use crate::{AppState, auth::verify_token};

/// Extract user ID from Authorization header
pub async fn extract_user_from_auth(
    headers: &HeaderMap, 
    state: &AppState,
    require_auth: bool
) -> Result<Option<String>, Response> {
    if let Some(auth_header) = headers.get(AUTHORIZATION) {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..]; // Remove "Bearer " prefix
                
                // Verify JWT token and extract user_id
                match verify_token(token, state.config.auth.jwt_secret.expose_secret().as_bytes()) {
                    Ok(claims) => {
                        match claims.sub.parse::<i64>() {
                            Ok(uid) => Ok(Some(uid.to_string())),
                            Err(_) => {
                                println!("❌ Invalid user ID in JWT token");
                                Err((
                                    StatusCode::UNAUTHORIZED,
                                    [("WWW-Authenticate", "Bearer")],
                                    Json(serde_json::json!({
                                        "errors": [{
                                            "code": "UNAUTHORIZED",
                                            "message": "Invalid user ID in token",
                                            "detail": {}
                                        }]
                                    }))
                                ).into_response())
                            }
                        }
                    }
                    Err(e) => {
                        println!("❌ JWT token verification failed: {:?}", e);
                        Err((
                            StatusCode::UNAUTHORIZED,
                            [("WWW-Authenticate", "Bearer")],
                            Json(serde_json::json!({
                                "errors": [{
                                    "code": "UNAUTHORIZED",
                                    "message": "Authentication required",
                                    "detail": {}
                                }]
                            }))
                        ).into_response())
                    }
                }
            } else if auth_str.starts_with("Basic ") {
                // Handle Basic authentication for docker login
                let encoded = &auth_str[6..]; // Remove "Basic " prefix
                match base64::prelude::BASE64_STANDARD.decode(encoded) {
                    Ok(decoded) => {
                        if let Ok(auth_string) = String::from_utf8(decoded) {
                            let parts: Vec<&str> = auth_string.splitn(2, ':').collect();
                            if parts.len() == 2 {
                                let username = parts[0];
                                let password = parts[1];
                                
                                // Verify credentials against database
                                match verify_docker_credentials(username, password, state).await {
                                    Ok(Some(user_id)) => Ok(Some(user_id)),
                                    Ok(None) => {
                                        println!("❌ Invalid docker credentials for user: {}", username);
                                        Err((
                                            StatusCode::UNAUTHORIZED,
                                            [("WWW-Authenticate", "Basic")],
                                            Json(serde_json::json!({
                                                "errors": [{
                                                    "code": "UNAUTHORIZED",
                                                    "message": "Invalid credentials",
                                                    "detail": {}
                                                }]
                                            }))
                                        ).into_response())
                                    }
                                    Err(_) => {
                                        println!("❌ Database error verifying credentials");
                                        Err((
                                            StatusCode::INTERNAL_SERVER_ERROR,
                                            Json(serde_json::json!({
                                                "errors": [{
                                                    "code": "UNKNOWN",
                                                    "message": "Internal server error",
                                                    "detail": {}
                                                }]
                                            }))
                                        ).into_response())
                                    }
                                }
                            } else {
                                println!("❌ Invalid Basic auth format");
                                Err((
                                    StatusCode::UNAUTHORIZED,
                                    [("WWW-Authenticate", "Basic")],
                                    Json(serde_json::json!({
                                        "errors": [{
                                            "code": "UNAUTHORIZED",
                                            "message": "Invalid authorization format",
                                            "detail": {}
                                        }]
                                    }))
                                ).into_response())
                            }
                        } else {
                            println!("❌ Invalid UTF-8 in Basic auth");
                            Err((
                                StatusCode::UNAUTHORIZED,
                                [("WWW-Authenticate", "Basic")],
                                Json(serde_json::json!({
                                    "errors": [{
                                        "code": "UNAUTHORIZED",
                                        "message": "Invalid authorization encoding",
                                        "detail": {}
                                    }]
                                }))
                            ).into_response())
                        }
                    }
                    Err(_) => {
                        println!("❌ Invalid base64 in Basic auth");
                        Err((
                            StatusCode::UNAUTHORIZED,
                            [("WWW-Authenticate", "Basic")],
                            Json(serde_json::json!({
                                "errors": [{
                                    "code": "UNAUTHORIZED",
                                    "message": "Invalid authorization encoding",
                                    "detail": {}
                                }]
                            }))
                        ).into_response())
                    }
                }
            } else {
                println!("❌ Invalid Authorization header format");
                Err((
                    StatusCode::UNAUTHORIZED,
                    [("WWW-Authenticate", "Basic")],
                    Json(serde_json::json!({
                        "errors": [{
                            "code": "UNAUTHORIZED",
                            "message": "Invalid authorization header",
                            "detail": {}
                        }]
                    }))
                ).into_response())
            }
        } else {
            println!("❌ Invalid Authorization header format");
            Err((
                StatusCode::UNAUTHORIZED,
                [("WWW-Authenticate", "Basic")],
                Json(serde_json::json!({
                    "errors": [{
                        "code": "UNAUTHORIZED",
                        "message": "Invalid authorization header",
                        "detail": {}
                    }]
                }))
            ).into_response())
        }
    } else {
        if require_auth {
            println!("⚠️ No Authorization header found");
            Err((
                StatusCode::UNAUTHORIZED,
                [("WWW-Authenticate", "Basic")],
                Json(serde_json::json!({
                    "errors": [{
                        "code": "UNAUTHORIZED",
                        "message": "Authentication required",
                        "detail": {}
                    }]
                }))
            ).into_response())
        } else {
            Ok(None)
        }
    }
}

/// Verify docker credentials (username/password) against database
/// Also supports API key as password for enhanced security
async fn verify_docker_credentials(
    username: &str,
    password: &str,
    state: &AppState,
) -> Result<Option<String>, sqlx::Error> {
    // First try to authenticate as a user with regular password
    let user_result = sqlx::query!(
        "SELECT id, username, password_hash FROM users WHERE username = $1",
        username
    )
    .fetch_optional(&state.db_pool)
    .await?;

    if let Some(user) = user_result {
        // Try to verify password - support both bcrypt and argon2
        let password_valid = if user.password_hash.starts_with("$argon2") {
            // Argon2 hash
            match PasswordHash::new(&user.password_hash) {
                Ok(parsed_hash) => {
                    match Argon2::default().verify_password(password.as_bytes(), &parsed_hash) {
                        Ok(_) => true,
                        Err(_) => false,
                    }
                }
                Err(e) => {
                    println!("❌ Failed to parse Argon2 hash: {}", e);
                    false
                }
            }
        } else {
            // Assume bcrypt hash
            match bcrypt::verify(password, &user.password_hash) {
                Ok(valid) => valid,
                Err(e) => {
                    println!("❌ Password verification error: {}", e);
                    false
                }
            }
        };

        if password_valid {
            println!("✅ Docker login successful for user: {}", username);
            return Ok(Some(user.id.to_string()));
        } else {
            println!("❌ Invalid password for user: {}", username);
        }

        // If regular password failed, try API key authentication
        // Check if the password looks like an API key (ak_<32_hex_chars>)
        if password.starts_with("ak_") && password.len() == 35 {
            println!("🔑 Attempting API key authentication for user: {}", username);
            
            // Use existing API key verification from auth module
            match crate::auth::verify_api_key(password, &state.db_pool, state.cache.as_ref()).await {
                Ok(api_user_id) => {
                    // Verify that the API key belongs to the same user
                    if api_user_id == user.id {
                        println!("✅ Docker login successful with API key for user: {}", username);
                        return Ok(Some(user.id.to_string()));
                    } else {
                        println!("❌ API key belongs to different user (id: {}) than requested user: {}", api_user_id, username);
                    }
                }
                Err(e) => {
                    println!("❌ API key verification failed: {:?}", e);
                }
            }
        }
    }

    // TODO: Uncomment when migration is applied
    // // If user authentication failed, try organization-level credentials
    // // This allows organization admins to set registry-specific credentials
    // let org_result = sqlx::query!(
    //     "SELECT id, registry_username, registry_password_hash 
    //      FROM organizations 
    //      WHERE registry_username = $1 AND registry_password_hash IS NOT NULL",
    //     username
    // )
    // .fetch_optional(&state.db_pool)
    // .await?;

    // if let Some(org) = org_result {
    //     if let Some(hash) = org.registry_password_hash {
    //         match bcrypt::verify(password, &hash) {
    //             Ok(true) => {
    //                 println!("✅ Docker login successful for organization: {}", username);
    //                 // Return organization ID as string with prefix to distinguish from user IDs
    //                 return Ok(Some(format!("org_{}", org.id)));
    //             }
    //             Ok(false) => {
    //                 println!("❌ Invalid organization registry password for: {}", username);
    //             }
    //             Err(e) => {
    //                 println!("❌ Organization password verification error: {}", e);
    //             }
    //         }
    //     }
    // }

    Ok(None)
}

/// Check if user has permission to access a repository
pub async fn check_repository_permission(
    user_id: &str,
    namespace: &str,
    repository: &str,
    operation: &str, // "pull", "push", "delete"
    state: &AppState,
) -> Result<bool, sqlx::Error> {
    println!("🔒 Checking {} permission for user {} on {}/{}", operation, user_id, namespace, repository);

    // If user_id starts with "org_", it's an organization-level access
    if user_id.starts_with("org_") {
        let org_id: i64 = user_id[4..].parse().unwrap_or(0);
        
        // Check if the organization matches the namespace
        let org_result = sqlx::query!(
            "SELECT name FROM organizations WHERE id = $1 AND name = $2",
            org_id, namespace
        )
        .fetch_optional(&state.db_pool)
        .await?;

        return Ok(org_result.is_some());
    }

    // Regular user permission check
    let user_id_int: i64 = user_id.parse().unwrap_or(0);

    // Check if user is a member of the organization and has required permissions
    let permission_result = sqlx::query!(
        "SELECT om.role, r.id 
         FROM organization_members om
         JOIN organizations o ON om.organization_id = o.id
         JOIN repositories r ON r.organization_id = o.id
         WHERE om.user_id = $1 AND o.name = $2 AND r.name = $3",
        user_id_int, namespace, repository
    )
    .fetch_optional(&state.db_pool)
    .await?;

    if let Some(member) = permission_result {
        match operation {
            "pull" => {
                // All organization members can pull
                Ok(true)
            }
            "push" => {
                // Only owners, admins and maintainers can push
                Ok(member.role == "owner" || member.role == "admin" || member.role == "maintainer")
            }
            "delete" => {
                // Only admins can delete
                Ok(member.role == "admin")
            }
            _ => Ok(false)
        }
    } else {
        // User is not a member of the organization
        // Check if repository is public or if user is the creator
        let repo_result = sqlx::query!(
            "SELECT r.id, r.is_public, r.created_by
             FROM repositories r
             JOIN organizations o ON r.organization_id = o.id
             WHERE o.name = $1 AND r.name = $2",
            namespace, repository
        )
        .fetch_optional(&state.db_pool)
        .await?;

        if let Some(repo) = repo_result {
            match operation {
                "pull" => {
                    // Allow pull if repository is public OR user is the creator OR user has org access
                    if repo.is_public || repo.created_by == Some(user_id_int) {
                        return Ok(true);
                    }
                    
                    // Check organization membership for private repos
                    let org_member = sqlx::query!(
                        "SELECT role FROM organization_members om 
                         JOIN organizations o ON om.organization_id = o.id 
                         WHERE om.user_id = $1 AND o.name = $2",
                        user_id_int, namespace
                    )
                    .fetch_optional(&state.db_pool)
                    .await?;
                    
                    Ok(org_member.is_some()) // Any org member can pull
                }
                "push" => {
                    // Allow push if user is the creator
                    if repo.created_by == Some(user_id_int) {
                        return Ok(true);
                    }
                    
                    // Check organization membership - only owner/maintainer can push
                    let org_member = sqlx::query!(
                        "SELECT role FROM organization_members om 
                         JOIN organizations o ON om.organization_id = o.id 
                         WHERE om.user_id = $1 AND o.name = $2",
                        user_id_int, namespace
                    )
                    .fetch_optional(&state.db_pool)
                    .await?;
                    
                    if let Some(member) = org_member {
                        Ok(member.role == "owner" || member.role == "admin" || member.role == "maintainer")
                    } else {
                        Ok(false)
                    }
                }
                "delete" => {
                    // Allow delete if user is the creator
                    if repo.created_by == Some(user_id_int) {
                        return Ok(true);
                    }
                    
                    // Check organization membership - only owner can delete
                    let org_member = sqlx::query!(
                        "SELECT role FROM organization_members om 
                         JOIN organizations o ON om.organization_id = o.id 
                         WHERE om.user_id = $1 AND o.name = $2",
                        user_id_int, namespace
                    )
                    .fetch_optional(&state.db_pool)
                    .await?;
                    
                    if let Some(member) = org_member {
                        Ok(member.role == "owner")
                    } else {
                        Ok(false)
                    }
                }
                _ => Ok(false)
            }
        } else {
            // Repository doesn't exist
            Ok(false)
        }
    }
}
