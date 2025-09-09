use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// User models
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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
    pub visibility: String, // public/private
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Image metadata models
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ImageMetadata {
    pub id: i64,
    pub repository_id: i64,
    pub digest: String,
    pub manifest: serde_json::Value,
    pub config: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub pushed_at: DateTime<Utc>,
    pub pulled_at: Option<DateTime<Utc>>,
    pub size_bytes: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ImageTag {
    pub id: i64,
    pub repository_id: i64,
    pub metadata_id: i64,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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
