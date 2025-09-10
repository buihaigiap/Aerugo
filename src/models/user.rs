use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// User information returned in API responses
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserResponse {
    /// Unique user ID
    pub id: i64,
    /// Username
    pub username: String,
    /// Email address
    pub email: String,
    /// When the user was created
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
}
