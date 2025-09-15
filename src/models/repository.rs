use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow, ToSchema)]
pub struct Repository {
    /// Unique repository ID
    pub id: i64,
    /// Organization ID that owns this repository
    pub organization_id: i64,
    /// Repository name (URL-friendly)
    pub name: String,
    /// Optional description
    pub description: Option<String>,
    /// Repository visibility (true = public, false = private)
    pub is_public: bool,
    /// User ID who created this repository
    pub created_by: Option<i64>,
    /// When the repository was created
    pub created_at: DateTime<Utc>,
    /// When the repository was last updated
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow, ToSchema)]
pub struct RepositoryPermission {
    /// Unique permission ID
    pub id: i64,
    /// Repository ID
    pub repository_id: i64,
    /// User ID (null if organization permission)
    pub user_id: Option<i64>,
    /// Organization ID (null if user permission)
    pub organization_id: Option<i64>,
    /// Permission level (read, write, admin)
    pub permission: String,
    /// User who granted this permission
    pub granted_by: i64,
    /// When the permission was granted
    pub created_at: DateTime<Utc>,
    /// When the permission was last updated
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateRepositoryRequest {
    /// Repository name (URL-friendly)
    #[validate(length(min = 3, max = 50))]
    pub name: String,
    /// Optional description
    #[validate(length(max = 500))]
    pub description: Option<String>,
    /// Repository visibility (true = public, false = private)
    pub is_public: bool,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SetRepositoryPermissionsRequest {
    /// User ID to grant permissions to (exclusive with organization_id)
    pub user_id: Option<i64>,
    /// Organization ID to grant permissions to (exclusive with user_id)
    pub organization_id: Option<i64>,
    /// Permission level to grant (read, write, admin)
    #[validate(length(min = 4, max = 5))]
    pub permission: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RepositoryDetailsResponse {
    /// Repository information
    pub repository: Repository,
    /// List of tags in the repository
    pub tags: Vec<String>,
    /// User's permissions for this repository
    pub user_permissions: Vec<RepositoryPermission>,
    /// Organization permissions for this repository
    pub org_permissions: Vec<RepositoryPermission>,
}