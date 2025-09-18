use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// Blob upload tracking models (simplified)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BlobUpload {
    pub id: i32,
    pub uuid: String,
    pub repository_id: i64,
    pub user_id: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct NewBlobUpload {
    pub uuid: String,
    pub repository_id: i64,
    pub user_id: Option<String>,
}

// User models
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

// Organization models
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Organization {
    pub id: i64,
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub website_url: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OrganizationMember {
    pub id: i64,
    pub organization_id: i64,
    pub user_id: i64,
    pub role: String,
    pub joined_at: DateTime<Utc>,
    pub invited_at: Option<DateTime<Utc>>,
    pub invited_by: Option<i64>,
}

// Repository models
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Repository {
    pub id: i64,
    pub organization_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub is_public: bool, // Changed from visibility to match database schema
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<i64>,
}

// Permission models
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Permission {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RolePermission {
    pub id: i64,
    pub role: String,
    pub permission_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResourceType {
    Organization,
    Repository,
    Image,
}

impl std::fmt::Display for ResourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResourceType::Organization => write!(f, "Organization"),
            ResourceType::Repository => write!(f, "Repository"),
            ResourceType::Image => write!(f, "Image"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ResourcePermission {
    pub id: i64,
    pub resource_type: String,
    pub resource_id: i64,
    pub user_id: Option<i64>,
    pub organization_id: Option<i64>,
    pub permission_id: i64,
    pub granted_at: DateTime<Utc>,
    pub granted_by: i64,
}

// Manifest models for Docker registry
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Manifest {
    pub id: i64,
    pub repository_id: i64,
    pub digest: String,
    pub media_type: String,
    pub size: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct NewManifest {
    pub repository_id: i64,
    pub digest: String,
    pub media_type: String,
    pub size: i64,
}

// Tag models for Docker registry
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Tag {
    pub id: i64,
    pub repository_id: i64,
    pub name: String,
    pub manifest_id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct NewTag {
    pub repository_id: i64,
    pub name: String,
    pub manifest_id: i64,
}
