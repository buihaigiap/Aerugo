use axum::{
    response::IntoResponse,
    http::StatusCode,
    Json,
    extract::State,
};
use serde::{Deserialize, Serialize};
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use jsonwebtoken::{encode, Header, EncodingKey};
use secrecy::ExposeSecret;
use crate::models::user::{User, NewUser};
use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    username: String,
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user id
    pub exp: usize, // expiration time
}

pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> impl IntoResponse {
    // Check if user already exists
    let existing_user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE email = $1",
        req.email
    )
    .fetch_optional(&state.db_pool)
    .await;

    if let Ok(Some(_)) = existing_user {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "User with this email already exists"
            }))
        );
    }

    // Hash password using Argon2
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = match argon2.hash_password(req.password.as_bytes(), &salt) {
        Ok(hash) => hash.to_string(),
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to hash password: {}", e)
                }))
            );
        }
    };

    // Create new user
    let new_user = NewUser {
        username: req.username,
        email: req.email,
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
    .await {
        Ok(user) => user,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to create user: {}", e)
                }))
            );
        }
    };

    // Generate JWT token
    let claims = Claims {
        sub: user.id.to_string(),
        exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
    };

    let token = match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.config.auth.jwt_secret.expose_secret().as_bytes())
    ) {
        Ok(token) => token,
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": format!("Failed to create token: {}", e)
            })));
        }
    };

    (StatusCode::CREATED, Json(serde_json::json!({
        "token": token
    })))
}

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> impl IntoResponse {
    // Find user by email
    let user = match sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE email = $1",
        req.email
    )
    .fetch_optional(&state.db_pool)
    .await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({
                "error": "Invalid email or password"
            })));
        }
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": format!("Database error: {}", e)
            })));
        }
    };

    // Verify password
    let parsed_hash = match PasswordHash::new(&user.password_hash) {
        Ok(hash) => hash,
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": format!("Failed to parse password hash: {}", e)
            })));
        }
    };

    if Argon2::default()
        .verify_password(req.password.as_bytes(), &parsed_hash)
        .is_err()
    {
        return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({
            "error": "Invalid email or password"
        })));
    }

    // Generate JWT token
    let claims = Claims {
        sub: user.id.to_string(),
        exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
    };

    let token = match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.config.auth.jwt_secret.expose_secret().as_bytes())
    ) {
        Ok(token) => token,
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": format!("Failed to create token: {}", e)
            })));
        }
    };

    (StatusCode::OK, Json(serde_json::json!({
        "token": token
    })))
}

pub async fn me(State(state): State<AppState>) -> impl IntoResponse {
    // This will be implemented later with JWT middleware
    (StatusCode::NOT_IMPLEMENTED, Json(serde_json::json!({
        "error": "Not implemented"
    })))
}
