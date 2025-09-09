// src/models/organization.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
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

#[derive(Debug, Deserialize, Validate)]
pub struct CreateOrganizationRequest {
    #[validate(length(min = 3, max = 50))]
    pub name: String,
    #[validate(length(min = 1, max = 100))]
    pub display_name: String,
    #[validate(length(max = 500))]
    pub description: Option<String>,
    pub website_url: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateOrganizationRequest {
    #[validate(length(min = 1, max = 100))]
    pub display_name: Option<String>,
    #[validate(length(max = 500))]
    pub description: Option<String>,
    pub website_url: Option<String>,
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
