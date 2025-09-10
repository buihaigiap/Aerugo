// src/models/organization.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow, ToSchema)]
pub struct Organization {
    /// Unique organization ID
    pub id: i64,
    /// Organization name (URL-friendly)
    pub name: String,
    /// Display name for the organization
    pub display_name: String,
    /// Optional description
    pub description: Option<String>,
    /// Optional website URL
    pub website_url: Option<String>,
    /// Optional avatar URL
    pub avatar_url: Option<String>,
    /// When the organization was created
    pub created_at: DateTime<Utc>,
    /// When the organization was last updated
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateOrganizationRequest {
    /// Organization name (3-50 characters, URL-friendly)
    #[validate(length(min = 3, max = 50))]
    pub name: String,
    /// Display name (1-100 characters)
    #[validate(length(min = 1, max = 100))]
    pub display_name: String,
    /// Optional description (max 500 characters)
    #[validate(length(max = 500))]
    pub description: Option<String>,
    /// Optional website URL
    pub website_url: Option<String>,
    /// Optional avatar URL
    pub avatar_url: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateOrganizationRequest {
    /// Updated display name (1-100 characters)
    #[validate(length(min = 1, max = 100))]
    pub display_name: Option<String>,
    /// Updated description (max 500 characters)
    #[validate(length(max = 500))]
    pub description: Option<String>,
    /// Updated website URL
    pub website_url: Option<String>,
    /// Updated avatar URL
    pub avatar_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct OrganizationMember {
    pub id: i64,
    pub organization_id: i64,
    pub user_id: i64,
    pub role: String, // Changed from OrganizationRole to String for now
    pub joined_at: DateTime<Utc>,
    pub invited_at: Option<DateTime<Utc>>,
    pub invited_by: Option<i64>,
    // User details (from JOIN)
    pub username: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrganizationInvitation {
    pub id: i64,
    pub organization_id: i64,
    pub email: String,
    pub role: String, // Changed from OrganizationRole to String for now
    pub token: String,
    pub invited_by: i64,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub accepted_at: Option<DateTime<Utc>>,
    pub accepted_by: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum OrganizationRole {
    Owner,
    Admin,
    Member,
}

impl std::fmt::Display for OrganizationRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrganizationRole::Owner => write!(f, "owner"),
            OrganizationRole::Admin => write!(f, "admin"),
            OrganizationRole::Member => write!(f, "member"),
        }
    }
}

impl std::str::FromStr for OrganizationRole {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "owner" => Ok(OrganizationRole::Owner),
            "admin" => Ok(OrganizationRole::Admin),
            "member" => Ok(OrganizationRole::Member),
            _ => Err(format!("Invalid organization role: {}", s)),
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct AddMemberRequest {
    #[validate(email)]
    pub email: String,
    pub role: OrganizationRole,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateMemberRequest {
    pub role: OrganizationRole,
}

impl OrganizationRole {
    pub fn can_manage_members(&self) -> bool {
        matches!(self, OrganizationRole::Owner | OrganizationRole::Admin)
    }

    pub fn can_manage_organization(&self) -> bool {
        matches!(self, OrganizationRole::Owner | OrganizationRole::Admin)
    }

    pub fn can_delete_organization(&self) -> bool {
        matches!(self, OrganizationRole::Owner)
    }

    pub fn can_remove_member(&self, target_role: &OrganizationRole) -> bool {
        match self {
            OrganizationRole::Owner => true,
            OrganizationRole::Admin => !matches!(target_role, OrganizationRole::Owner),
            OrganizationRole::Member => false,
        }
    }

    pub fn can_change_role_to(&self, target_role: &OrganizationRole) -> bool {
        match self {
            OrganizationRole::Owner => true,
            OrganizationRole::Admin => !matches!(target_role, OrganizationRole::Owner),
            OrganizationRole::Member => false,
        }
    }
}
