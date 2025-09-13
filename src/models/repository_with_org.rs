use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Organization {
    pub id: i64,
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub website_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct RepositoryWithOrg {
    pub id: i64,
    pub organization_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub is_public: bool,
    pub created_by: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub organization: Organization,
}

#[derive(Debug, FromRow)]
pub struct RepositoryWithOrgRow {
    // Repository fields
    pub id: i64,
    pub organization_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub is_public: bool,
    pub created_by: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    
    // Organization fields
    pub org_id: i64,
    pub org_name: String,
    pub org_display_name: String,
    pub org_description: Option<String>,
    pub org_website_url: Option<String>,
}

impl From<RepositoryWithOrgRow> for RepositoryWithOrg {
    fn from(row: RepositoryWithOrgRow) -> Self {
        RepositoryWithOrg {
            id: row.id,
            organization_id: row.organization_id,
            name: row.name,
            description: row.description,
            is_public: row.is_public,
            created_by: row.created_by,
            created_at: row.created_at,
            updated_at: row.updated_at,
            organization: Organization {
                id: row.org_id,
                name: row.org_name,
                display_name: row.org_display_name,
                description: row.org_description,
                website_url: row.org_website_url,
            },
        }
    }
}