use anyhow::{Context, Result};
use sqlx::{PgPool, Postgres, Transaction};
use tracing::{info, error};

use super::models::*;
use crate::models::repository_with_org::{RepositoryWithOrg, RepositoryWithOrgRow};

// Blob upload queries (simplified)
pub async fn create_blob_upload(
    pool: &PgPool,
    uuid: &str,
    repository_id: i64,
    user_id: Option<&str>,
) -> Result<BlobUpload> {
    info!("🔧 Creating blob upload: uuid={}, repository_id={}, user_id={:?}", uuid, repository_id, user_id);
    
    // Convert user_id string to i64 if present
    let user_id_int: Option<i64> = match user_id {
        Some(id_str) => {
            // Extract numeric part from user_id like "user_12345" -> 12345
            if id_str.starts_with("user_") {
                id_str[5..].parse().ok()
            } else {
                id_str.parse().ok()
            }
        }
        None => None,
    };
    
    let result = sqlx::query_as::<_, BlobUpload>(
        "INSERT INTO blob_uploads (uuid, repository_id, user_id)
         VALUES ($1, $2, $3)
         RETURNING id, uuid, repository_id, user_id, created_at",
    )
    .bind(uuid)
    .bind(repository_id)
    .bind(user_id_int)
    .fetch_one(pool)
    .await;
    
    match &result {
        Ok(_) => info!("✅ Blob upload created successfully"),
        Err(e) => error!("❌ Database insert error: {}", e),
    }
    
    result.context("Failed to create blob upload record")
}

pub async fn update_blob_upload_completed(
    pool: &PgPool,
    uuid: &str,
) -> Result<()> {
    sqlx::query(
        "UPDATE blob_uploads SET completed_at = NOW() WHERE uuid = $1"
    )
    .bind(uuid)
    .execute(pool)
    .await
    .context("Failed to update blob upload completion")?;
    
    Ok(())
}

// Repository queries
pub async fn repository_exists(
    pool: &PgPool,
    repository_id: i64,
) -> Result<bool> {
    let exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM repositories WHERE id = $1)"
    )
    .bind(repository_id)
    .fetch_one(pool)
    .await
    .context("Failed to check repository existence")?;
    
    Ok(exists)
}

pub async fn get_repository_id_by_name(
    pool: &PgPool,
    repository_name: &str,
) -> Result<Option<i64>> {
    // Handle both formats: "repo" and "org/repo"
    let (org_name, repo_name) = if repository_name.contains('/') {
        let parts: Vec<&str> = repository_name.splitn(2, '/').collect();
        (Some(parts[0]), parts[1])
    } else {
        (None, repository_name)
    };

    let id = if let Some(org_name) = org_name {
        // Look for repository with org/repo format
        sqlx::query_scalar::<_, i64>(
            "SELECT r.id FROM repositories r 
             JOIN organizations o ON r.organization_id = o.id 
             WHERE r.name = $1 AND o.name = $2"
        )
        .bind(repo_name)
        .bind(org_name)
        .fetch_optional(pool)
        .await
        .context("Failed to get repository ID by org/name")?
    } else {
        // Look for repository with just repo name
        sqlx::query_scalar::<_, i64>(
            "SELECT id FROM repositories WHERE name = $1"
        )
        .bind(repo_name)
        .fetch_optional(pool)
        .await
        .context("Failed to get repository ID by name")?
    };
    
    Ok(id)
}

// User queries
pub async fn create_user(
    pool: &PgPool,
    username: &str,
    email: &str,
    password_hash: &str,
) -> Result<User> {
    sqlx::query_as::<_, User>(
        "INSERT INTO users (username, email, password_hash)
         VALUES ($1, $2, $3)
         RETURNING *",
    )
    .bind(username)
    .bind(email)
    .bind(password_hash)
    .fetch_one(pool)
    .await
    .context("Failed to create user")
}

pub async fn get_user_by_id(pool: &PgPool, user_id: i64) -> Result<Option<User>> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(pool)
        .await
        .context("Failed to get user")
}

// Organization queries
pub async fn create_organization(
    tx: &mut Transaction<'_, Postgres>,
    name: &str,
    display_name: &str,
    description: Option<&str>,
    website_url: Option<&str>,
    creator_id: i64,
) -> Result<Organization> {
    // Create organization
    let org = sqlx::query_as::<_, Organization>(
        "INSERT INTO organizations (name, display_name, description, website_url)
         VALUES ($1, $2, $3, $4)
         RETURNING *",
    )
    .bind(name)
    .bind(display_name)
    .bind(description)
    .bind(website_url)
    .fetch_one(&mut **tx)
    .await
    .context("Failed to create organization")?;

    // Add creator as owner
    sqlx::query(
        "INSERT INTO organization_members (organization_id, user_id, role)
         VALUES ($1, $2, $3)",
    )
    .bind(org.id)
    .bind(creator_id)
    .bind("owner")
    .execute(&mut **tx)
    .await
    .context("Failed to add organization owner")?;

    Ok(org)
}

// Repository queries
pub async fn create_repository(
    tx: &mut Transaction<'_, Postgres>,
    org_id: i64,
    name: &str,
    description: Option<&str>,
    visibility: &str,
) -> Result<Repository> {
    sqlx::query_as::<_, Repository>(
        "INSERT INTO repositories (organization_id, name, description, visibility)
         VALUES ($1, $2, $3, $4)
         RETURNING *",
    )
    .bind(org_id)
    .bind(name)
    .bind(description)
    .bind(visibility)
    .fetch_one(&mut **tx)
    .await
    .context("Failed to create repository")
}

pub async fn get_repository_with_org(pool: &PgPool, repo_id: i64) -> Result<Option<RepositoryWithOrg>> {
    let row = sqlx::query_as::<_, RepositoryWithOrgRow>(
        "SELECT 
            r.id, r.organization_id, r.name, r.description, r.is_public, r.created_by, r.created_at, r.updated_at,
            o.id as org_id, o.name as org_name, o.display_name as org_display_name, o.description as org_description, o.website_url as org_website_url
         FROM repositories r
         JOIN organizations o ON r.organization_id = o.id
         WHERE r.id = $1"
    )
    .bind(repo_id)
    .fetch_optional(pool)
    .await
    .context("Failed to get repository with organization details")?;

    Ok(row.map(|r| r.into()))
}

// Permission queries - simplified to only check organization membership
pub async fn check_permission(
    pool: &PgPool,
    user_id: i64,
    resource_type: ResourceType,
    resource_id: i64,
    _required_permission: &str,
) -> Result<bool> {
    // Only check if user is member of the organization that owns the resource
    let org_permission = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS (
            SELECT 1 FROM organization_members om
            WHERE om.user_id = $1
            AND om.organization_id = (
                CASE $2
                    WHEN 'Organization' THEN $3
                    WHEN 'Repository' THEN (SELECT organization_id FROM repositories WHERE id = $3)
                END
            )
        )",
    )
    .bind(user_id)
    .bind(resource_type.to_string())
    .bind(resource_id)
    .fetch_one(pool)
    .await
    .context("Failed to check organization permission")?;

    Ok(org_permission)
}

// Transaction helpers
pub async fn transaction<'a, F, R>(pool: &PgPool, f: F) -> Result<R>
where
    F: for<'b> FnOnce(
        &'b mut Transaction<'_, Postgres>,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<R>> + 'b>>,
{
    let mut tx = pool.begin().await?;

    match f(&mut tx).await {
        Ok(result) => {
            tx.commit().await?;
            Ok(result)
        }
        Err(e) => {
            tx.rollback().await?;
            Err(e)
        }
    }
}
