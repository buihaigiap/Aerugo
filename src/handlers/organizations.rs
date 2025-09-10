// src/handlers/organizations.rs - Fixed version
use anyhow::{bail, Context, Result};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::{FromRow, PgPool};
use validator::Validate;
use utoipa::ToSchema;

use crate::{
    models::organizations::{
        AddMemberRequest, CreateOrganizationRequest, Organization, OrganizationMember,
        OrganizationRole, UpdateMemberRequest, UpdateOrganizationRequest,
    },
    AppState,
};

/// Create a new organization
#[utoipa::path(
    post,
    path = "/api/v1/orgs",
    tag = "organizations",
    request_body = CreateOrganizationRequest,
    responses(
        (status = 201, description = "Organization created successfully"),
        (status = 400, description = "Validation failed or bad request"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_organization(
    State(state): State<AppState>,
    Json(req): Json<CreateOrganizationRequest>,
) -> impl IntoResponse {
    // Validate request
    if let Err(validation_errors) = req.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "Validation failed",
                "details": validation_errors
            })),
        );
    }

    // TODO: Get user_id from JWT token when auth middleware is implemented
    let user_id = 1i64; // Placeholder

    match create_org_internal(&state.db_pool, req, user_id).await {
        Ok(organization) => (
            StatusCode::CREATED,
            Json(serde_json::json!({
                "organization": organization
            })),
        ),
        Err(e) => {
            tracing::error!("Failed to create organization: {}", e);
            (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": e.to_string()
                })),
            )
        }
    }
}

/// Get organization details by ID
#[utoipa::path(
    get,
    path = "/api/v1/orgs/{id}",
    tag = "organizations",
    params(
        ("id" = i64, Path, description = "Organization ID")
    ),
    responses(
        (status = 200, description = "Organization details retrieved successfully"),
        (status = 404, description = "Organization not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_organization(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    match get_org_by_id_internal(&state.db_pool, id).await {
        Ok(Some(organization)) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "organization": organization
            })),
        ),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "Organization not found"
            })),
        ),
        Err(e) => {
            tracing::error!("Failed to get organization: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Internal server error"
                })),
            )
        }
    }
}

// Update organization
pub async fn update_organization(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateOrganizationRequest>,
) -> impl IntoResponse {
    if let Err(validation_errors) = req.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "Validation failed",
                "details": validation_errors
            })),
        );
    }

    // TODO: Get user_id from JWT token
    let user_id = 1i64; // Placeholder

    match update_org_by_id_internal(&state.db_pool, id, req, user_id).await {
        Ok(organization) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "organization": organization
            })),
        ),
        Err(e) => {
            tracing::error!("Failed to update organization: {}", e);
            (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": e.to_string()
                })),
            )
        }
    }
}

// Delete organization
pub async fn delete_organization(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    // TODO: Get user_id from JWT token
    let user_id = 1i64; // Placeholder

    match delete_org_by_id_internal(&state.db_pool, id, user_id).await {
        Ok(_) => (StatusCode::NO_CONTENT, Json(serde_json::json!({}))),
        Err(e) => {
            tracing::error!("Failed to delete organization: {}", e);
            (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": e.to_string()
                })),
            )
        }
    }
}

// Get organization members
pub async fn get_organization_members(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    // TODO: Get user_id from JWT token
    let user_id = Some(1i64); // Placeholder

    match get_members_by_org_id_internal(&state.db_pool, id, user_id).await {
        Ok(members) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "members": members
            })),
        ),
        Err(e) => {
            tracing::error!("Failed to get organization members: {}", e);
            (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": e.to_string()
                })),
            )
        }
    }
}

// Add member to organization
pub async fn add_organization_member(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(req): Json<AddMemberRequest>,
) -> impl IntoResponse {
    if let Err(validation_errors) = req.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "Validation failed",
                "details": validation_errors
            })),
        );
    }

    // TODO: Get user_id from JWT token
    let inviter_id = 1i64; // Placeholder

    match add_member_by_org_id_internal(&state.db_pool, id, req, inviter_id).await {
        Ok(member) => (
            StatusCode::CREATED,
            Json(serde_json::json!({
                "member": member
            })),
        ),
        Err(e) => {
            tracing::error!("Failed to add organization member: {}", e);
            (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": e.to_string()
                })),
            )
        }
    }
}

// Update member role
pub async fn update_member_role(
    State(state): State<AppState>,
    Path((id, member_id)): Path<(i64, i64)>,
    Json(req): Json<UpdateMemberRequest>,
) -> impl IntoResponse {
    // TODO: Get user_id from JWT token
    let updater_id = 1i64; // Placeholder

    match update_member_role_by_org_id_internal(&state.db_pool, id, member_id, req, updater_id)
        .await
    {
        Ok(member) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "member": member
            })),
        ),
        Err(e) => {
            tracing::error!("Failed to update member role: {}", e);
            (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": e.to_string()
                })),
            )
        }
    }
}

// Remove member from organization
pub async fn remove_organization_member(
    State(state): State<AppState>,
    Path((id, member_id)): Path<(i64, i64)>,
) -> impl IntoResponse {
    // TODO: Get user_id from JWT token
    let remover_id = 1i64; // Placeholder

    match remove_member_internal(&state.db_pool, id, member_id, remover_id).await {
        Ok(_) => (StatusCode::NO_CONTENT, Json(serde_json::json!({}))),
        Err(e) => {
            tracing::error!("Failed to remove organization member: {}", e);
            (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": e.to_string()
                })),
            )
        }
    }
}

/// List all organizations for the authenticated user
#[utoipa::path(
    get,
    path = "/api/v1/orgs",
    tag = "organizations",
    responses(
        (status = 200, description = "List of user's organizations retrieved successfully"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn list_user_organizations(State(state): State<AppState>) -> impl IntoResponse {
    // TODO: Get user_id from JWT token
    let user_id = 1i64; // Placeholder

    match list_user_orgs_internal(&state.db_pool, user_id).await {
        Ok(organizations) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "organizations": organizations
            })),
        ),
        Err(e) => {
            tracing::error!("Failed to list user organizations: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Internal server error"
                })),
            )
        }
    }
}

// Helper function to get user's role in organization
async fn get_user_role_in_org(
    pool: &PgPool,
    org_id: i64,
    user_id: i64,
) -> Result<Option<OrganizationRole>> {
    #[derive(FromRow)]
    struct RoleRow {
        role: String,
    }

    let result = sqlx::query_as::<_, RoleRow>("SELECT om.role FROM organization_members om JOIN organizations o ON om.organization_id = o.id WHERE o.id = $1 AND om.user_id = $2")
        .bind(org_id)
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

    match result {
        Some(row) => match row.role.as_str() {
            "owner" => Ok(Some(OrganizationRole::Owner)),
            "admin" => Ok(Some(OrganizationRole::Admin)),
            "member" => Ok(Some(OrganizationRole::Member)),
            _ => Ok(None),
        },
        None => Ok(None),
    }
}

// Internal database functions
async fn create_org_internal(
    pool: &PgPool,
    req: CreateOrganizationRequest,
    creator_id: i64,
) -> Result<Organization> {
    let mut tx = pool.begin().await?;

    // Check if organization name already exists
    let existing = sqlx::query("SELECT id FROM organizations WHERE name = $1")
        .bind(&req.name)
        .fetch_optional(&mut *tx)
        .await?;

    if existing.is_some() {
        bail!("Organization with name '{}' already exists", req.name);
    }

    // Create organization
    let org = sqlx::query_as::<_, Organization>(
        "INSERT INTO organizations (name, display_name, description, website_url, avatar_url)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, name, display_name, description, website_url, avatar_url, created_at, updated_at"
    )
    .bind(&req.name)
    .bind(&req.display_name)
    .bind(&req.description)
    .bind(&req.website_url)
    .bind(&req.avatar_url)
    .fetch_one(&mut *tx)
    .await?;

    // Add creator as owner
    sqlx::query(
        "INSERT INTO organization_members (organization_id, user_id, role)
        VALUES ($1, $2, $3)",
    )
    .bind(org.id)
    .bind(creator_id)
    .bind("owner")
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(org)
}

async fn get_org_by_id_internal(pool: &PgPool, org_id: i64) -> Result<Option<Organization>> {
    sqlx::query_as::<_, Organization>(
        "SELECT id, name, display_name, description, website_url, avatar_url, created_at, updated_at
         FROM organizations
         WHERE id = $1"
    )
    .bind(org_id)
    .fetch_optional(pool)
    .await
    .context("Failed to fetch organization")
}

async fn update_org_by_id_internal(
    pool: &PgPool,
    org_id: i64,
    req: UpdateOrganizationRequest,
    user_id: i64,
) -> Result<Organization> {
    // Check if user has permission to update
    let user_role = get_user_role_in_org(pool, org_id, user_id).await?;
    if !user_role
        .map(|r| r.can_manage_organization())
        .unwrap_or(false)
    {
        bail!("Insufficient permissions to update organization");
    }

    sqlx::query_as::<_, Organization>(
        "UPDATE organizations
         SET 
             display_name = COALESCE($2, display_name),
             description = COALESCE($3, description),
             website_url = COALESCE($4, website_url),
             avatar_url = COALESCE($5, avatar_url),
             updated_at = CURRENT_TIMESTAMP
         WHERE id = $1
         RETURNING id, name, display_name, description, website_url, avatar_url, created_at, updated_at"
    )
    .bind(org_id)
    .bind(&req.display_name)
    .bind(&req.description)
    .bind(&req.website_url)
    .bind(&req.avatar_url)
    .fetch_one(pool)
    .await
    .context("Organization not found")
}

async fn delete_org_by_id_internal(pool: &PgPool, org_id: i64, user_id: i64) -> Result<()> {
    let user_role = get_user_role_in_org(pool, org_id, user_id).await?;
    if !user_role
        .map(|r| r.can_delete_organization())
        .unwrap_or(false)
    {
        bail!("Only organization owners can delete organizations");
    }

    let result = sqlx::query("DELETE FROM organizations WHERE id = $1")
        .bind(org_id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        bail!("Organization not found");
    }

    Ok(())
}

async fn get_members_by_org_id_internal(
    pool: &PgPool,
    org_id: i64,
    user_id: Option<i64>,
) -> Result<Vec<OrganizationMember>> {
    // Check if user has access to view members
    if let Some(uid) = user_id {
        let user_role = get_user_role_in_org(pool, org_id, uid).await?;
        if user_role.is_none() {
            bail!("Access denied: not a member of this organization");
        }
    }

    sqlx::query_as::<_, OrganizationMember>(
        "SELECT 
            om.id, om.organization_id, om.user_id, om.role,
            om.joined_at, om.invited_at, om.invited_by,
            u.username, u.email
        FROM organization_members om
        JOIN users u ON om.user_id = u.id
        JOIN organizations o ON om.organization_id = o.id
        WHERE o.id = $1
        ORDER BY om.joined_at ASC",
    )
    .bind(org_id)
    .fetch_all(pool)
    .await
    .context("Failed to fetch organization members")
}

async fn add_member_by_org_id_internal(
    pool: &PgPool,
    org_id: i64,
    req: AddMemberRequest,
    inviter_id: i64,
) -> Result<OrganizationMember> {
    let inviter_role = get_user_role_in_org(pool, org_id, inviter_id).await?;
    if !inviter_role
        .map(|r| r.can_manage_members())
        .unwrap_or(false)
    {
        bail!("Insufficient permissions to add members");
    }

    // Find user by email
    #[derive(FromRow)]
    struct User {
        id: i64,
        username: String,
        email: String,
    }

    let user = sqlx::query_as::<_, User>("SELECT id, username, email FROM users WHERE email = $1")
        .bind(&req.email)
        .fetch_one(pool)
        .await
        .context("User not found with that email")?;

    // Check if user is already a member
    let existing = sqlx::query(
        "SELECT id FROM organization_members WHERE organization_id = $1 AND user_id = $2",
    )
    .bind(org_id)
    .bind(user.id)
    .fetch_optional(pool)
    .await?;

    if existing.is_some() {
        bail!("User is already a member of this organization");
    }

    // Add member
    let member_id: i64 = sqlx::query_scalar(
        "INSERT INTO organization_members (organization_id, user_id, role, invited_by)
         VALUES ($1, $2, $3, $4)
         RETURNING id",
    )
    .bind(org_id)
    .bind(user.id)
    .bind(&req.role.to_string())
    .bind(inviter_id)
    .fetch_one(pool)
    .await?;

    // Return the created member
    let member = OrganizationMember {
        id: member_id,
        organization_id: org_id,
        user_id: user.id,
        role: req.role.to_string(),
        joined_at: chrono::Utc::now(),
        invited_at: Some(chrono::Utc::now()),
        invited_by: Some(inviter_id),
        username: user.username,
        email: user.email,
    };

    Ok(member)
}

async fn update_member_role_by_org_id_internal(
    pool: &PgPool,
    org_id: i64,
    member_user_id: i64,
    req: UpdateMemberRequest,
    updater_id: i64,
) -> Result<OrganizationMember> {
    let updater_role = get_user_role_in_org(pool, org_id, updater_id).await?;
    let target_current_role = get_user_role_in_org(pool, org_id, member_user_id).await?;

    if let (Some(updater), Some(target)) = (updater_role, target_current_role) {
        if !updater.can_change_role_to(&req.role) {
            bail!("Insufficient permissions to assign this role");
        }
        if !updater.can_remove_member(&target) {
            bail!("Insufficient permissions to modify this member");
        }
    } else {
        bail!("Invalid member or insufficient permissions");
    }

    // Organization ID is already provided

    // Update the role
    sqlx::query(
        "UPDATE organization_members SET role = $3 WHERE organization_id = $1 AND user_id = $2",
    )
    .bind(org_id)
    .bind(member_user_id)
    .bind(&req.role.to_string())
    .execute(pool)
    .await?;

    // Fetch and return updated member info
    let member = sqlx::query_as::<_, OrganizationMember>(
        "SELECT 
            om.id, om.organization_id, om.user_id, om.role,
            om.joined_at, om.invited_at, om.invited_by,
            u.username, u.email
        FROM organization_members om
        JOIN users u ON om.user_id = u.id
        WHERE om.organization_id = $1 AND om.user_id = $2",
    )
    .bind(org_id)
    .bind(member_user_id)
    .fetch_one(pool)
    .await
    .context("Member not found")?;

    Ok(member)
}

async fn remove_member_internal(
    pool: &PgPool,
    org_id: i64,
    member_user_id: i64,
    remover_id: i64,
) -> Result<()> {
    let remover_role = get_user_role_in_org(pool, org_id, remover_id).await?;
    let target_role = get_user_role_in_org(pool, org_id, member_user_id).await?;

    // Allow self-removal for any role
    if remover_id != member_user_id {
        if let (Some(remover), Some(target)) = (remover_role, target_role) {
            if !remover.can_remove_member(&target) {
                bail!("Insufficient permissions to remove this member");
            }
        } else {
            bail!("Invalid member or insufficient permissions");
        }
    }

    // Organization ID is already provided

    let result =
        sqlx::query("DELETE FROM organization_members WHERE organization_id = $1 AND user_id = $2")
            .bind(org_id)
            .bind(member_user_id)
            .execute(pool)
            .await?;

    if result.rows_affected() == 0 {
        bail!("Member not found");
    }

    Ok(())
}

async fn list_user_orgs_internal(pool: &PgPool, user_id: i64) -> Result<Vec<Organization>> {
    sqlx::query_as!(
        Organization,
        r#"
        SELECT o.id, o.name, o.display_name, o.description, 
               o.website_url, o.avatar_url, o.created_at, o.updated_at
        FROM organizations o
        JOIN organization_members om ON o.id = om.organization_id
        WHERE om.user_id = $1
        ORDER BY o.name
        "#,
        user_id
    )
    .fetch_all(pool)
    .await
    .context("Failed to fetch user organizations")
}
