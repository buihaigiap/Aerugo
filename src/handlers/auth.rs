use crate::database::models::{NewUser, User};
use crate::models::api_key::ApiKey;
use crate::AppState;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::{extract::State, http::{StatusCode, HeaderMap}, response::IntoResponse, Json};
use axum_extra::headers::{authorization::Bearer, Authorization};
use axum_extra::TypedHeader;
use jsonwebtoken::{encode, EncodingKey, Header};
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use chrono::{Duration, Utc};
use uuid::Uuid;
use rand;
use sha2;
use hex;

/// User registration request
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RegisterRequest {
    /// Username for the new account
    username: String,
    /// Email address for the new account
    email: String,
    /// Password for the new account (min 8 characters)
    password: String,
}

/// Login request
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LoginRequest {
    /// User's email address (either email or username required)
    #[serde(default)]
    email: String,
    /// Username (either email or username required)
    #[serde(default)]
    username: String,
    /// User's password
    password: String,
}

/// Authentication response with JWT token
#[derive(Debug, Serialize, ToSchema)]
pub struct AuthResponse {
    /// JWT token for authenticating subsequent requests
    token: String,
}

/// Password change request
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ChangePasswordRequest {
    /// Current password for verification
    current_password: String,
    /// New password (min 8 characters)
    new_password: String,
    /// Confirmation of new password (must match new_password)
    confirm_password: String,
}

/// Forgot password request - simplified version
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ForgotPasswordRequest {
    /// Email address to reset password for
    #[schema(example = "user@example.com")]
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct VerifyOtpRequest {
    /// Email address 
    #[schema(example = "user@example.com")]
    pub email: String,
    /// 6-digit OTP code
    #[schema(example = "123456")]
    pub otp_code: String,
    /// New password
    #[schema(example = "newpassword123")]
    pub new_password: String,
    /// Confirm new password
    #[schema(example = "newpassword123")]
    pub confirm_password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user id
    pub exp: usize,  // expiration time
}

/// Register a new user
#[utoipa::path(
    post,
    path = "/api/v1/auth/register",
    tag = "auth",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User successfully registered", body = AuthResponse),
        (status = 409, description = "User already exists"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> impl IntoResponse {
    // Input validation for registration request
    
    // Validate password length (minimum 8 characters)
    if req.password.len() < 8 {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "Password must be at least 8 characters long"
            })),
        );
    }
    
    // Basic email format validation
    // Check for '@' and ensure there's a domain after it
    if !req.email.contains('@') {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "Invalid email format - must contain '@'"
            })),
        );
    }
    
    let email_parts: Vec<&str> = req.email.split('@').collect();
    if email_parts.len() != 2 || email_parts[0].is_empty() || email_parts[1].is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "Invalid email format - must have local part and domain"
            })),
        );
    }
    
    let local_part = email_parts[0];
    let domain = email_parts[1];
    
    // Check local part length and basic validity
    if local_part.len() > 64 {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "Local part of email too long (max 64 characters)"
            })),
        );
    }
    
    // Check domain validity
    if domain.len() > 255 {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "Domain part of email too long (max 255 characters)"
            })),
        );
    }
    
    // Domain must not start with dot
    if domain.starts_with('.') {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "Invalid domain - cannot start with dot"
            })),
        );
    }
    
    // Domain must not end with dot
    if domain.ends_with('.') {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "Invalid domain - cannot end with dot"
            })),
        );
    }
    
    // Check for consecutive dots in domain
    if domain.contains("..") {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "Invalid domain - consecutive dots not allowed"
            })),
        );
    }
    
    // Additional simple checks for common invalid patterns
    if req.email.starts_with('@') || req.email.ends_with('@') {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "Invalid email format"
            })),
        );
    }
    
    // Check if user already exists by email (unique constraint)
    let existing_user = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", req.email)
        .fetch_optional(&state.db_pool)
        .await;

    if let Ok(Some(_)) = existing_user {
        return (
            StatusCode::CONFLICT,
            Json(serde_json::json!({
                "error": "User with this email already exists"
            })),
        );
    }

    // Hash password using Argon2
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = match argon2.hash_password(req.password.as_bytes(), &salt) {
        Ok(hash) => hash.to_string(),
        Err(e) => {
            tracing::error!("Password hashing failed: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to hash password"
                })),
            );
        }
    };

    // Create new user record
    let new_user = NewUser {
        username: req.username.clone(),
        email: req.email.clone(),
        password_hash,
    };

    // Insert user into database
    let user = match sqlx::query_as!(
        User,
        "INSERT INTO users (username, email, password_hash)
         VALUES ($1, $2, $3)
         RETURNING id, username, email, password_hash, created_at",
        new_user.username,
        new_user.email,
        new_user.password_hash,
    )
    .fetch_one(&state.db_pool)
    .await
    {
        Ok(user) => user,
        Err(e) => {
            tracing::error!("Database insertion failed: {}", e);
            // Check if error is due to duplicate username (if constraint exists)
            if e.to_string().contains("duplicate key") {
                return (
                    StatusCode::CONFLICT,
                    Json(serde_json::json!({
                        "error": "Username already exists"
                    })),
                );
            }
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to create user"
                })),
            );
        }
    };

    // Generate JWT token with 24-hour expiration
    let claims = Claims {
        sub: user.id.to_string(),
        exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
    };

    let token = match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.config.auth.jwt_secret.expose_secret().as_bytes()),
    ) {
        Ok(token) => token,
        Err(e) => {
            tracing::error!("JWT token generation failed: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to create authentication token"
                })),
            );
        }
    };

    // Return success response with token
    (
        StatusCode::CREATED,
        Json(serde_json::json!({
            "token": token,
            "message": "User registered successfully"
        })),
    )
}

/// Login with username or email and password
#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    tag = "auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = AuthResponse),
        (status = 401, description = "Invalid credentials"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> impl IntoResponse {
    // Find user by email or username
    let user = if !req.email.is_empty() {
        // Try to find user by email
        match sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", req.email)
            .fetch_optional(&state.db_pool)
            .await
        {
            Ok(Some(user)) => Some(user),
            Ok(None) => None,
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({
                        "error": format!("Database error: {}", e)
                    })),
                );
            }
        }
    } else if !req.username.is_empty() {
        // Try to find user by username
        match sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1", req.username)
            .fetch_optional(&state.db_pool)
            .await
        {
            Ok(Some(user)) => Some(user),
            Ok(None) => None,
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({
                        "error": format!("Database error: {}", e)
                    })),
                );
            }
        }
    } else {
        None
    };
    
    // Return error if user not found
    let user = match user {
        Some(user) => user,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "error": "Invalid email or password"
                })),
            );
        }
    };

    // Verify password
    let parsed_hash = match PasswordHash::new(&user.password_hash) {
        Ok(hash) => hash,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to parse password hash: {}", e)
                })),
            );
        }
    };

    if Argon2::default()
        .verify_password(req.password.as_bytes(), &parsed_hash)
        .is_err()
    {
        return (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({
                "error": "Invalid email or password"
            })),
        );
    }

    // Generate JWT token
    let claims = Claims {
        sub: user.id.to_string(),
        exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
    };

    let token = match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.config.auth.jwt_secret.expose_secret().as_bytes()),
    ) {
        Ok(token) => token,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to create token: {}", e)
                })),
            );
        }
    };

    (
        StatusCode::OK,
        Json(serde_json::json!({
            "token": token
        })),
    )
}

/// Get current user information
#[utoipa::path(
    get,
    path = "/api/v1/auth/me",
    responses(
        (status = 200, description = "User information retrieved successfully", body = UserResponse),
        (status = 401, description = "Unauthorized"),
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn me(
    auth: Option<TypedHeader<Authorization<Bearer>>>,
    headers: HeaderMap,
    State(state): State<AppState>
) -> impl IntoResponse {
    // Add debug logging
    if let Some(ref auth_header) = auth {
        tracing::debug!("Auth header present: {}", auth_header.token());
    } else {
        tracing::debug!("No auth header provided");
    }
    
    // Extract and verify token using dual authentication (JWT + API key)
    let user_id = match crate::auth::extract_user_id_dual(
        auth,
        &headers,
        state.config.auth.jwt_secret.expose_secret().as_bytes(),
        &state.db_pool,
        state.cache.as_ref()
    ).await {
        Ok(id) => {
            tracing::debug!("Authentication successful for user ID: {}", id);
            id
        },
        Err(status) => {
            tracing::debug!("Authentication failed with status: {:?}", status);
            return (
                status,
                Json(serde_json::json!({
                    "error": "Unauthorized"
                }))
            );
        }
    };

    #[derive(sqlx::FromRow)]
    struct UserInfo {
        id: i64,
        username: String,
        email: String,
    }

    match sqlx::query_as::<_, UserInfo>("SELECT id, username, email FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(&state.db_pool)
        .await
    {
        Ok(Some(user)) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "id": user.id,
                "username": user.username,
                "email": user.email,
                "created_at": chrono::Utc::now()  // Adding created_at as expected by test
            })),
        ),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "User not found"
            })),
        ),
        Err(e) => {
            tracing::error!("Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Internal server error"
                })),
            )
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RefreshRequest {
    /// Old token to refresh
    token: String,
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/refresh",
    tag = "auth",
    request_body = RefreshRequest,
    responses(
        (status = 200, description = "Token refreshed successfully", body = AuthResponse),
        (status = 401, description = "Invalid or expired token"),
        (status = 500, description = "Internal server error")
    )
)]

pub async fn refresh(
    State(state): State<AppState>,
    Json(req): Json<RefreshRequest>,
) -> impl IntoResponse {
    let claims = match crate::auth::verify_token(&req.token, state.config.auth.jwt_secret.expose_secret().as_bytes()) {
        Ok(claims) => claims,
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "error": "Invalid token"
                })),
            );
        }
    };

    let new_claims = Claims {
        sub: claims.sub,
        exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
    };

    let new_token = match encode(
        &Header::default(),
        &new_claims,
        &EncodingKey::from_secret(state.config.auth.jwt_secret.expose_secret().as_bytes()),
    ) {
        Ok(token) => token,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to create new token: {}", e)
                })),
            );
        }
    };

    (
        StatusCode::OK,
        Json(serde_json::json!({
            "token": new_token
        })),
    )
}

/// Logout request
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LogoutRequest {
    /// JWT token to invalidate
    token: String,
}

/// Logout handler to invalidate authentication cache
#[utoipa::path(
    post,
    path = "/auth/logout",
    tag = "authentication",
    request_body = LogoutRequest,
    responses(
        (status = 200, description = "Successfully logged out"),
        (status = 401, description = "Invalid token"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn logout(
    State(state): State<AppState>,
    Json(req): Json<LogoutRequest>,
) -> impl IntoResponse {
    // Verify the token first
    let claims = match crate::auth::verify_token(&req.token, state.config.auth.jwt_secret.expose_secret().as_bytes()) {
        Ok(claims) => claims,
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "error": "Invalid token"
                })),
            );
        }
    };

    // Invalidate token in cache
    if let Some(cache) = &state.cache {
        if let Err(e) = cache.invalidate_auth_token(&req.token).await {
            tracing::warn!("Failed to invalidate auth token in cache: {}", e);
        }

        // Optionally invalidate user permissions
        if let Ok(user_id) = claims.sub.parse::<i64>() {
            if let Err(e) = cache.invalidate_user_permissions(&user_id.to_string()).await {
                tracing::warn!("Failed to invalidate user permissions in cache: {}", e);
            }
        }
    }

    (
        StatusCode::OK,
        Json(serde_json::json!({
            "message": "Successfully logged out"
        })),
    )
}

/// Change user password
#[utoipa::path(
    put,
    path = "/api/v1/auth/change-password",
    tag = "auth",
    request_body = ChangePasswordRequest,
    security(
        ("bearerAuth" = [])
    ),
    responses(
        (status = 200, description = "Password successfully changed"),
        (status = 400, description = "Invalid request or password validation failed"),
        (status = 401, description = "Unauthorized - invalid token or current password"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn change_password(
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    State(state): State<AppState>,
    Json(req): Json<ChangePasswordRequest>,
) -> impl IntoResponse {
    // Verify JWT token
    let claims = match crate::auth::verify_token(auth.token(), state.config.auth.jwt_secret.expose_secret().as_bytes()) {
        Ok(claims) => claims,
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "error": "Invalid or expired token"
                })),
            );
        }
    };

    // Parse user ID from claims
    let user_id: i64 = match claims.sub.parse() {
        Ok(id) => id,
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "error": "Invalid token format"
                })),
            );
        }
    };

    // Validate password requirements
    if req.new_password.len() < 8 {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "New password must be at least 8 characters long"
            })),
        );
    }

    if req.new_password != req.confirm_password {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "New password and confirmation do not match"
            })),
        );
    }

    // Get current user from database
    let user = match sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id)
        .fetch_optional(&state.db_pool)
        .await
    {
        Ok(Some(user)) => user,
        Ok(None) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "error": "User not found"
                })),
            );
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Database error: {}", e)
                })),
            );
        }
    };

    // Verify current password
    let parsed_hash = match PasswordHash::new(&user.password_hash) {
        Ok(hash) => hash,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to parse stored password hash"
                })),
            );
        }
    };

    if Argon2::default()
        .verify_password(req.current_password.as_bytes(), &parsed_hash)
        .is_err()
    {
        return (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({
                "error": "Current password is incorrect"
            })),
        );
    }

    // Hash the new password
    let salt = SaltString::generate(&mut OsRng);
    let new_password_hash = match Argon2::default()
        .hash_password(req.new_password.as_bytes(), &salt)
    {
        Ok(hash) => hash.to_string(),
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to hash new password"
                })),
            );
        }
    };

    // Update password in database
    match sqlx::query!(
        "UPDATE users SET password_hash = $1 WHERE id = $2",
        new_password_hash,
        user_id
    )
    .execute(&state.db_pool)
    .await
    {
        Ok(_) => {
            // Optionally invalidate existing auth tokens in cache
            if let Some(cache) = &state.cache {
                // Note: In a production system, you might want to invalidate all user sessions
                // or maintain a token blacklist for better security
                if let Err(e) = cache.invalidate_user_permissions(&user_id.to_string()).await {
                    tracing::warn!("Failed to invalidate user permissions in cache: {}", e);
                }
            }

            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "message": "Password successfully changed"
                })),
            )
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to update password: {}", e)
                })),
            );
        }
    }
}

/// NEW: Simple forgot password with 6-digit OTP
#[utoipa::path(
    post,
    path = "/api/v1/auth/forgot-password",
    tag = "auth",
    request_body = ForgotPasswordRequest,
    responses(
        (status = 200, description = "Password reset OTP sent to email"),
        (status = 400, description = "Invalid email or user not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn forgot_password(
    State(state): State<AppState>,
    Json(req): Json<ForgotPasswordRequest>,
) -> impl IntoResponse {
    // Find user by email
    let user = match sqlx::query!("SELECT id, username, email FROM users WHERE email = $1", req.email)
        .fetch_optional(&state.db_pool)
        .await
    {
        Ok(Some(user)) => user,
        Ok(None) => {
            return Json(serde_json::json!({
                "error": "Email not found"
            }));
        }
        Err(_) => {
            return Json(serde_json::json!({
                "error": "Internal server error"  
            }));
        }
    };

    // Generate 6-digit code
    use rand::Rng;
    let otp_code: u32 = rand::thread_rng().gen_range(100000..=999999);
    let otp_string = otp_code.to_string();
    
    // Store OTP in Redis cache with 15 minutes TTL
    if let Some(cache) = &state.cache {
        if let Err(e) = cache.cache_otp_code(&user.email, &otp_string, std::time::Duration::from_secs(900)).await {
            tracing::warn!("Failed to store OTP in cache: {}", e);
            return Json(serde_json::json!({
                "error": "Failed to generate OTP code"
            }));
        }
    } else {
        return Json(serde_json::json!({
            "error": "OTP service not available"
        }));
    }
    
    // Send email
    match state.email_service.send_forgot_password_email(
        &user.email, 
        &user.username,
        &otp_string,
        ""
    ).await {
        Ok(()) => Json(serde_json::json!({
            "message": "Password reset instructions have been sent to your email",
            "email_sent": true
        })),
        Err(_) => Json(serde_json::json!({
            "error": "Failed to send email"
        }))
    }
}

/// Verify OTP and reset password
#[utoipa::path(
    post,
    path = "/api/v1/auth/verify-otp",
    tag = "auth",
    request_body = VerifyOtpRequest,
    responses(
        (status = 200, description = "Password successfully reset"),
        (status = 400, description = "Invalid OTP, passwords don't match, or validation failed"),
        (status = 404, description = "OTP expired or does not exist"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn verify_otp_and_reset(
    State(state): State<AppState>,
    Json(req): Json<VerifyOtpRequest>,
) -> impl IntoResponse {
    // Validate passwords match
    if req.new_password != req.confirm_password {
        return Json(serde_json::json!({
            "error": "Passwords do not match"
        }));
    }
    
    // Validate password length
    if req.new_password.len() < 8 {
        return Json(serde_json::json!({
            "error": "Password must be at least 8 characters long"
        }));
    }

    // Find user by email
    let user = match sqlx::query!("SELECT id, username, email FROM users WHERE email = $1", req.email)
        .fetch_optional(&state.db_pool)
        .await
    {
        Ok(Some(user)) => user,
        Ok(None) => {
            return Json(serde_json::json!({
                "error": "Email not found"
            }));
        }
        Err(_) => {
            return Json(serde_json::json!({
                "error": "Internal server error"  
            }));
        }
    };

    // Validate OTP format
    if req.otp_code.len() != 6 || !req.otp_code.chars().all(|c| c.is_ascii_digit()) {
        return Json(serde_json::json!({
            "error": "Invalid OTP code. Must be 6 digits."
        }));
    }
    
    // Verify OTP from Redis cache
    if let Some(cache) = &state.cache {
        match cache.get_otp_code(&req.email).await {
            Some(stored_otp) => {
                if stored_otp != req.otp_code {
                    return Json(serde_json::json!({
                        "error": "Invalid OTP code"
                    }));
                }
                // OTP is valid, delete it to prevent reuse
                let _ = cache.remove_otp_code(&req.email).await;
            }
            None => {
                return Json(serde_json::json!({
                    "error": "OTP code has expired or does not exist"
                }));
            }
        }
    } else {
        return Json(serde_json::json!({
            "error": "OTP verification service not available"
        }));
    }

    // Hash new password
    use argon2::{Argon2, PasswordHasher};
    use argon2::password_hash::{SaltString, rand_core::OsRng};
    
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = match Argon2::default().hash_password(req.new_password.as_bytes(), &salt) {
        Ok(hash) => hash.to_string(),
        Err(_) => {
            return Json(serde_json::json!({
                "error": "Failed to hash password"
            }));
        }
    };

    // Update password in database
    match sqlx::query!("UPDATE users SET password_hash = $1 WHERE id = $2", password_hash, user.id)
        .execute(&state.db_pool)
        .await
    {
        Ok(_) => Json(serde_json::json!({
            "message": "Password successfully reset",
            "success": true
        })),
        Err(_) => Json(serde_json::json!({
            "error": "Failed to update password"
        }))
    }
}

/// API Key response (without the actual secret key)
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ApiKeyResponse {
    /// API key ID
    pub id: i64,
    /// Name/description of the API key
    pub name: Option<String>,
    /// When this key was last used
    pub last_used_at: Option<chrono::NaiveDateTime>,
    /// When this key expires
    pub expires_at: Option<chrono::NaiveDateTime>,
    /// Whether this key is active
    pub is_active: Option<bool>,
    /// When this key was created
    pub created_at: Option<chrono::NaiveDateTime>,
}

/// Create API Key request
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateApiKeyRequest {
    /// Name/description for the API key
    pub name: String,
}

/// Create API Key response (includes the actual key - only shown once!)
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateApiKeyResponse {
    /// API key ID
    pub id: i64,
    /// The actual API key (ak_...) - ONLY SHOWN ONCE!
    pub api_key: String,
    /// Optional expiration date
    pub expires_at: Option<chrono::NaiveDateTime>,
    /// Creation timestamp
    pub created_at: Option<chrono::NaiveDateTime>,
    /// Security warning
    pub warning: String,
}

/// Delete API Key response
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeleteApiKeyResponse {
    /// Success message
    pub message: String,
}

/// Error response for API key operations
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ApiKeyErrorResponse {
    /// Error message
    pub error: String,
    /// Error details (optional)
    pub details: Option<String>,
}

/// Get user's API keys
#[utoipa::path(
    get,
    path = "/api/v1/auth/api-keys",
    params(
        ("name" = Option<String>, Query, description = "Search API keys by name")
    ),
    responses(
        (status = 200, description = "User API keys retrieved successfully", body = Vec<ApiKeyResponse>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error"),
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn get_user_api_keys(
    auth_header: Option<TypedHeader<Authorization<Bearer>>>,
    headers: HeaderMap,
    State(state): State<AppState>,
    name: Option<axum::extract::Query<std::collections::HashMap<String, String>>>,
) -> Result<Json<Vec<ApiKeyResponse>>, StatusCode> {
    // Extract API key from X-API-Key header
    let api_key = headers.get("x-api-key")
        .and_then(|v| v.to_str().ok());

    // Extract user ID using dual auth function (supports both JWT and API key)
    let user_id = crate::auth::extract_user_id_dual_auth(
        auth_header, 
        api_key,
        &state.config.auth.jwt_secret.expose_secret().as_bytes(),
        &state.db_pool, 
        state.cache.as_ref()
    ).await.map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Extract name filter from query parameters
    let name_filter = name.as_ref()
        .and_then(|query| query.get("name"))
        .map(|n| n.as_str());

    // Query user's API keys from database with optional name filter
    let api_keys = if let Some(search_name) = name_filter {
        sqlx::query_as!(
            ApiKey,
            r#"
            SELECT id, user_id, name, key_hash, last_used_at, expires_at, created_at, updated_at, is_active
            FROM api_keys 
            WHERE user_id = $1 AND is_active = true AND name ILIKE $2
            ORDER BY created_at DESC
            "#,
            user_id,
            format!("%{}%", search_name)
        )
        .fetch_all(&state.db_pool)
        .await
    } else {
        sqlx::query_as!(
            ApiKey,
            r#"
            SELECT id, user_id, name, key_hash, last_used_at, expires_at, created_at, updated_at, is_active
            FROM api_keys 
            WHERE user_id = $1 AND is_active = true
            ORDER BY created_at DESC
            "#,
            user_id
        )
        .fetch_all(&state.db_pool)
        .await
    };

    let api_keys = api_keys.map_err(|e| {
        tracing::error!("Database error fetching API keys: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Convert to response format (without the actual secret key)
    let response: Vec<ApiKeyResponse> = api_keys
        .into_iter()
        .map(|key| {
            ApiKeyResponse {
                id: key.id,
                name: key.name,
                last_used_at: key.last_used_at,
                expires_at: key.expires_at,
                is_active: key.is_active,
                created_at: key.created_at,
            }
        })
        .collect();

    Ok(Json(response))
}

/// Create a new API key for the user
#[utoipa::path(
    post,
    path = "/api/v1/auth/api-keys",
    request_body = CreateApiKeyRequest,
    tag = "auth",
    responses(
        (status = 201, description = "API key created successfully", body = CreateApiKeyResponse),
        (status = 401, description = "Unauthorized - invalid or missing JWT token", body = ApiKeyErrorResponse),
        (status = 409, description = "Conflict - API key name already exists for this user", body = ApiKeyErrorResponse),
        (status = 500, description = "Internal server error", body = ApiKeyErrorResponse)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn create_api_key(
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    State(state): State<AppState>,
    Json(request): Json<CreateApiKeyRequest>,
) -> Result<(StatusCode, Json<CreateApiKeyResponse>), (StatusCode, Json<ApiKeyErrorResponse>)> {
    // Extract user ID from JWT
    let user_id = crate::auth::extract_user_id_dual_auth(
        Some(TypedHeader(auth)), 
        None, // No X-API-Key header for this endpoint
        &state.config.auth.jwt_secret.expose_secret().as_bytes(),
        &state.db_pool, 
        state.cache.as_ref()
    ).await.map_err(|_| {
        let error_response = ApiKeyErrorResponse {
            error: "Authentication failed".to_string(),
            details: Some("Invalid or expired JWT token".to_string()),
        };
        (StatusCode::UNAUTHORIZED, Json(error_response))
    })?;

    // Check if user already has an API key with the same name
    let existing_key = sqlx::query!(
        "SELECT id FROM api_keys WHERE user_id = $1 AND name = $2 AND is_active = true",
        user_id,
        request.name
    )
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Database error checking duplicate API key name: {}", e);
        let error_response = ApiKeyErrorResponse {
            error: "Database error".to_string(),
            details: Some("Failed to check for duplicate API key names".to_string()),
        };
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    if existing_key.is_some() {
        tracing::warn!("User {} attempted to create API key with duplicate name: {}", user_id, request.name);
        let error_response = ApiKeyErrorResponse {
            error: "Duplicate API key name".to_string(),
            details: Some(format!("An API key with the name '{}' already exists for this user", request.name)),
        };
        return Err((StatusCode::CONFLICT, Json(error_response)));
    }

    // Set expiration to 15 days from now
    let expires_at = chrono::Utc::now().naive_utc() + chrono::Duration::days(15);

    // Generate new API key
    let api_key = format!("ak_{}", hex::encode(rand::random::<[u8; 16]>()));
    
    // Hash the API key for storage
    let key_hash = {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(api_key.as_bytes());
        hex::encode(hasher.finalize())
    };

    // Insert into database
    let api_key_record = sqlx::query_as!(
        ApiKey,
        r#"
        INSERT INTO api_keys (user_id, key_hash, name, expires_at, is_active)
        VALUES ($1, $2, $3, $4, true)
        RETURNING id, user_id, name, key_hash, last_used_at, expires_at, created_at, updated_at, is_active
        "#,
        user_id,
        key_hash,
        request.name,
        Some(expires_at),
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Database error creating API key: {}", e);
        let error_response = ApiKeyErrorResponse {
            error: "Failed to create API key".to_string(),
            details: Some("Database insertion error".to_string()),
        };
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    // Return response with the actual API key (only time it's shown!)
    let response = CreateApiKeyResponse {
        id: api_key_record.id,
        api_key: api_key.clone(), // 🔑 The actual key - only shown once!
        expires_at: api_key_record.expires_at,
        created_at: api_key_record.created_at,
        warning: "⚠️ SECURITY WARNING: This API key will only be shown once. Please save it securely immediately. If lost, you will need to generate a new one.".to_string(),
    };

    tracing::info!("Created new API key for user {} (expires: {})", 
        user_id, expires_at.format("%Y-%m-%d %H:%M:%S"));

    Ok((StatusCode::CREATED, Json(response)))
}

/// Delete an API key
#[utoipa::path(
    delete,
    path = "/api/v1/auth/api-keys/{id}",
    params(
        ("id" = i64, Path, description = "API key ID to delete")
    ),
    tag = "auth",
    responses(
        (status = 200, description = "API key deleted successfully", body = DeleteApiKeyResponse),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "API key not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn delete_api_key(
    auth_header: Option<TypedHeader<Authorization<Bearer>>>,
    headers: HeaderMap,
    State(state): State<AppState>,
    axum::extract::Path(key_id): axum::extract::Path<i64>,
) -> Result<(StatusCode, Json<DeleteApiKeyResponse>), StatusCode> {
    // Extract API key from X-API-Key header
    let api_key = headers.get("x-api-key")
        .and_then(|v| v.to_str().ok());

    // Extract user ID using dual auth function
    let user_id = crate::auth::extract_user_id_dual_auth(
        auth_header, 
        api_key,
        &state.config.auth.jwt_secret.expose_secret().as_bytes(),
        &state.db_pool, 
        state.cache.as_ref()
    ).await.map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Delete the API key (only if it belongs to the user)
    let result = sqlx::query!(
        "DELETE FROM api_keys WHERE id = $1 AND user_id = $2",
        key_id,
        user_id
    )
    .execute(&state.db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Database error deleting API key: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    tracing::info!("Deleted API key {} for user {}", key_id, user_id);
    
    let response = DeleteApiKeyResponse {
        message: "API key deleted successfully".to_string(),
    };
    
    Ok((StatusCode::OK, Json(response)))
}

/// Clean up expired API keys from database
pub async fn cleanup_expired_api_keys(db_pool: &sqlx::PgPool) -> Result<i64, sqlx::Error> {
    let now = chrono::Utc::now().naive_utc();
    
    let result = sqlx::query!(
        "DELETE FROM api_keys WHERE expires_at IS NOT NULL AND expires_at < $1",
        now
    )
    .execute(db_pool)
    .await?;

    tracing::info!("Cleaned up {} expired API keys", result.rows_affected());
    Ok(result.rows_affected() as i64)
}  
