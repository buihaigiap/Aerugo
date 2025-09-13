use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::headers::{authorization::Bearer, Authorization};
use axum_extra::TypedHeader;
use secrecy::ExposeSecret;
use serde_json::json;
use utoipa;

use crate::{
    AppState,
    auth::extract_user_id,
    models::repository::{Repository, RepositoryPermission, CreateRepositoryRequest, SetRepositoryPermissionsRequest, RepositoryDetailsResponse},
};

/// List all repositories in an organization
/// GET /api/v1/repos/{namespace}/repositories
#[utoipa::path(
    get,
    path = "/api/v1/repos/{namespace}/repositories",
    tag = "repositories",
    params(
        ("namespace" = String, Path, description = "Organization namespace/name")
    ),
    responses(
        (status = 200, description = "List of repositories retrieved successfully", body = Vec<Repository>),
        (status = 404, description = "Organization not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn list_repositories(
    State(state): State<AppState>,
    Path(namespace): Path<String>,
) -> Response {
    // First get the organization ID from namespace
    let org = match sqlx::query!(
        "SELECT id FROM organizations WHERE name = $1",
        namespace
    )
    .fetch_optional(&state.db_pool)
    .await {
        Ok(Some(org)) => org,
        Ok(None) => {
            return (StatusCode::NOT_FOUND, Json(json!({
                "error": "Organization not found"
            }))).into_response()
        }
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "error": format!("Database error: {}", e)
            }))).into_response()
        }
    };

    // Get all repositories for the organization
    match sqlx::query_as!(Repository,
        "SELECT * FROM repositories WHERE organization_id = $1",
        org.id
    )
    .fetch_all(&state.db_pool)
    .await {
        Ok(repositories) => {
            (StatusCode::OK, Json(repositories)).into_response()
        }
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "error": format!("Database error: {}", e)
            }))).into_response()
        }
    }
}

/// Create a new repository
/// POST /api/v1/repos/{namespace}
#[utoipa::path(
    post,
    path = "/api/v1/repos/{namespace}",
    tag = "repositories",
    params(
        ("namespace" = String, Path, description = "Organization namespace/name")
    ),
    request_body = CreateRepositoryRequest,
    responses(
        (status = 201, description = "Repository created successfully", body = Repository),
        (status = 400, description = "Bad request - invalid input"),
        (status = 401, description = "Authentication required"),
        (status = 404, description = "Organization not found"),
        (status = 409, description = "Repository already exists"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn create_repository(
    State(state): State<AppState>,
    auth: Option<TypedHeader<Authorization<Bearer>>>,
    Path(namespace): Path<String>,
    Json(request): Json<CreateRepositoryRequest>,
) -> impl IntoResponse {
    // Extract user ID from JWT token
    let user_id = match extract_user_id(auth, state.config.auth.jwt_secret.expose_secret().as_bytes()).await {
        Ok(id) => id,
        Err(status) => {
            return (status, Json(json!({
                "error": "Authentication required"
            }))).into_response()
        }
    };
    // First get the organization ID from namespace
    let org = match sqlx::query!(
        "SELECT id FROM organizations WHERE name = $1",
        namespace
    )
    .fetch_optional(&state.db_pool)
    .await {
        Ok(Some(org)) => org,
        Ok(None) => {
            return (StatusCode::NOT_FOUND, Json(json!({
                "error": "Organization not found"
            }))).into_response()
        }
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "error": format!("Database error: {}", e)
            }))).into_response()
        }
    };

    // Check if repository already exists
    let exists = match sqlx::query!(
        "SELECT id FROM repositories WHERE organization_id = $1 AND name = $2",
        org.id, request.name
    )
    .fetch_optional(&state.db_pool)
    .await {
        Ok(Some(_)) => true,
        Ok(None) => false,
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "error": format!("Database error: {}", e)
            }))).into_response()
        }
    };

    if exists {
        return (StatusCode::CONFLICT, Json(json!({
            "error": "Repository already exists"
        }))).into_response();
    }

    // Create repository with user ID
    match sqlx::query_as!(Repository,
        r#"
        INSERT INTO repositories (organization_id, name, description, is_public, created_by, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
        RETURNING *
        "#,
        org.id,
        request.name,
        request.description,
        request.is_public,
        user_id
    )
    .fetch_one(&state.db_pool)
    .await {
        Ok(repo) => {
            // Grant admin permissions to the creator
            if let Err(e) = sqlx::query!(
                "INSERT INTO repository_permissions (user_id, repository_id, permission) VALUES ($1, $2, 'admin')",
                user_id,
                repo.id
            )
            .execute(&state.db_pool)
            .await {
                eprintln!("Warning: Failed to grant admin permissions to repository creator: {}", e);
            }
            
            (StatusCode::CREATED, Json(repo)).into_response()
        },
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "error": format!("Database error: {}", e)
            }))).into_response()
        }
    }
}

/// Get repository details and tags
/// GET /api/v1/repos/{namespace}/repositories/{repo_name}
#[utoipa::path(
    get,
    path = "/api/v1/repos/{namespace}/repositories/{repo_name}",
    tag = "repositories",
    params(
        ("namespace" = String, Path, description = "Organization namespace/name"),
        ("repo_name" = Option<String>, Path, description = "Repository name (optional)")
    ),
    responses(
        (status = 200, description = "Repository details retrieved successfully", body = RepositoryDetailsResponse),
        (status = 404, description = "Organization or repository not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_repository(
    State(state): State<AppState>,
    Path((namespace, repo_name)): Path<(String, Option<String>)>,
) -> Response {
    // If repo_name is None, return all repositories
    if repo_name.is_none() {
        return list_repositories(State(state), Path(namespace)).await;
    }
    // First get the organization ID from namespace
    let org = match sqlx::query!(
        "SELECT id FROM organizations WHERE name = $1",
        namespace
    )
    .fetch_optional(&state.db_pool)
    .await {
        Ok(Some(org)) => org,
        Ok(None) => {
            return (StatusCode::NOT_FOUND, Json(json!({
                "error": format!("Organization '{}' not found", namespace),
                "code": "ORGANIZATION_NOT_FOUND",
                "details": "Please check if the organization namespace is correct"
            }))).into_response()
        }
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "error": format!("Database error: {}", e),
                "code": "DATABASE_ERROR"
            }))).into_response()
        }
    };

    // Get repository details
    let repo_name = repo_name.unwrap(); // Safe to unwrap since we checked for None earlier
    let repo = match sqlx::query_as!(Repository,
        "SELECT * FROM repositories WHERE organization_id = $1 AND name = $2",
        org.id, repo_name
    )
    .fetch_optional(&state.db_pool)
    .await {
        Ok(Some(repo)) => repo,
        Ok(None) => {
            return (StatusCode::NOT_FOUND, Json(json!({
                "error": format!("Repository '{}' not found in organization '{}'", repo_name, namespace),
                "code": "REPOSITORY_NOT_FOUND",
                "details": "Please check if the repository name is correct and exists in this organization"
            }))).into_response()
        }
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "error": format!("Database error: {}", e),
                "code": "DATABASE_ERROR"
            }))).into_response()
        }
    };

    // Get tags for this repository
    let tags = match sqlx::query!(
        "SELECT DISTINCT tag FROM images WHERE repository_id = $1 ORDER BY tag",
        repo.id
    )
    .fetch_all(&state.db_pool)
    .await {
        Ok(tags) => tags.into_iter().filter_map(|t| t.tag).collect(),
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "error": format!("Database error: {}", e)
            }))).into_response()
        }
    };

    // Get user permissions
    let user_permissions = match sqlx::query!(
        "SELECT id, repository_id, user_id, organization_id, permission::TEXT as permission, granted_by, created_at, updated_at FROM repository_permissions WHERE repository_id = $1 AND user_id IS NOT NULL",
        repo.id
    )
    .fetch_all(&state.db_pool)
    .await {
        Ok(rows) => rows.into_iter().map(|row| RepositoryPermission {
            id: row.id,
            repository_id: row.repository_id,
            user_id: row.user_id,
            organization_id: row.organization_id,
            permission: row.permission.unwrap_or_default(),
            granted_by: row.granted_by,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }).collect(),
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "error": format!("Database error: {}", e)
            }))).into_response()
        }
    };

    // Get organization permissions
    let org_permissions = match sqlx::query!(
        "SELECT id, repository_id, user_id, organization_id, permission::TEXT as permission, granted_by, created_at, updated_at FROM repository_permissions WHERE repository_id = $1 AND organization_id IS NOT NULL",
        repo.id
    )
    .fetch_all(&state.db_pool)
    .await {
        Ok(rows) => rows.into_iter().map(|row| RepositoryPermission {
            id: row.id,
            repository_id: row.repository_id,
            user_id: row.user_id,
            organization_id: row.organization_id,
            permission: row.permission.unwrap_or_default(),
            granted_by: row.granted_by,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }).collect(),
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "error": format!("Database error: {}", e)
            }))).into_response()
        }
    };

    let response = RepositoryDetailsResponse {
        repository: repo,
        tags,
        user_permissions,
        org_permissions,
    };

    (StatusCode::OK, Json(response)).into_response()
}

/// Delete a repository
/// DELETE /api/v1/repos/{namespace}/{repo_name}
#[utoipa::path(
    delete,
    path = "/api/v1/repos/{namespace}/{repo_name}",
    tag = "repositories",
    params(
        ("namespace" = String, Path, description = "Organization namespace/name"),
        ("repo_name" = String, Path, description = "Repository name")
    ),
    responses(
        (status = 200, description = "Repository deleted successfully"),
        (status = 404, description = "Organization or repository not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn delete_repository(
    State(state): State<AppState>,
    Path((namespace, repo_name)): Path<(String, String)>,
) -> impl IntoResponse {
    // First get the organization ID from namespace
    let org = match sqlx::query!(
        "SELECT id FROM organizations WHERE name = $1",
        namespace
    )
    .fetch_optional(&state.db_pool)
    .await {
        Ok(Some(org)) => org,
        Ok(None) => {
            return (StatusCode::NOT_FOUND, Json(json!({
                "error": "Organization not found"
            }))).into_response()
        }
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "error": format!("Database error: {}", e)
            }))).into_response()
        }
    };

    // Delete repository
    match sqlx::query!(
        "DELETE FROM repositories WHERE organization_id = $1 AND name = $2",
        org.id, repo_name
    )
    .execute(&state.db_pool)
    .await {
        Ok(result) if result.rows_affected() > 0 => {
            (StatusCode::NO_CONTENT).into_response()
        }
        Ok(_) => {
            (StatusCode::NOT_FOUND, Json(json!({
                "error": "Repository not found"
            }))).into_response()
        }
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "error": format!("Database error: {}", e)
            }))).into_response()
        }
    }
}

/// Set user/team permissions for a repository
/// PUT /api/v1/repos/{namespace}/{repo_name}/permissions
#[utoipa::path(
    put,
    path = "/api/v1/repos/{namespace}/{repo_name}/permissions",
    tag = "repositories",
    params(
        ("namespace" = String, Path, description = "Organization namespace/name"),
        ("repo_name" = String, Path, description = "Repository name")
    ),
    request_body = SetRepositoryPermissionsRequest,
    responses(
        (status = 200, description = "Permissions updated successfully"),
        (status = 400, description = "Bad request - invalid input"),
        (status = 404, description = "Repository not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn set_repository_permissions(
    State(state): State<AppState>,
    Path((namespace, repo_name)): Path<(String, String)>,
    Json(request): Json<SetRepositoryPermissionsRequest>,
) -> impl IntoResponse {
    if request.user_id.is_none() && request.organization_id.is_none() {
        return (StatusCode::BAD_REQUEST, Json(json!({
            "error": "Either user_id or organization_id must be provided"
        }))).into_response();
    }

    if request.user_id.is_some() && request.organization_id.is_some() {
        return (StatusCode::BAD_REQUEST, Json(json!({
            "error": "Only one of user_id or organization_id can be provided"
        }))).into_response();
    }

    // First get the organization ID from namespace
    let org = match sqlx::query!(
        "SELECT id FROM organizations WHERE name = $1",
        namespace
    )
    .fetch_optional(&state.db_pool)
    .await {
        Ok(Some(org)) => org,
        Ok(None) => {
            return (StatusCode::NOT_FOUND, Json(json!({
                "error": "Organization not found"
            }))).into_response()
        }
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "error": format!("Database error: {}", e)
            }))).into_response()
        }
    };

    // Get repository
    let repo = match sqlx::query!(
        "SELECT id FROM repositories WHERE organization_id = $1 AND name = $2",
        org.id, repo_name
    )
    .fetch_optional(&state.db_pool)
    .await {
        Ok(Some(repo)) => repo,
        Ok(None) => {
            return (StatusCode::NOT_FOUND, Json(json!({
                "error": "Repository not found"
            }))).into_response()
        }
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "error": format!("Database error: {}", e)
            }))).into_response()
        }
    };

    // Upsert permission (assuming user_id=1 as granted_by for now)
    let granted_by = 1i64; // TODO: Get from auth context
    
    if let Some(user_id) = request.user_id {
        // User permission
        match sqlx::query(
            r#"
            INSERT INTO repository_permissions (repository_id, user_id, organization_id, permission, granted_by)
            VALUES ($1, $2, NULL, $3, $4)
            ON CONFLICT (repository_id, user_id)
            DO UPDATE SET permission = EXCLUDED.permission, updated_at = CURRENT_TIMESTAMP
            "#
        )
        .bind(repo.id)
        .bind(user_id)
        .bind(&request.permission)
        .bind(granted_by)
        .execute(&state.db_pool)
        .await {
            Ok(_) => (StatusCode::NO_CONTENT).into_response(),
            Err(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                    "error": format!("Database error: {}", e)
                }))).into_response()
            }
        }
    } else if let Some(org_id) = request.organization_id {
        // Organization permission
        match sqlx::query(
            r#"
            INSERT INTO repository_permissions (repository_id, user_id, organization_id, permission, granted_by)
            VALUES ($1, NULL, $2, $3, $4)
            ON CONFLICT (repository_id, organization_id)
            DO UPDATE SET permission = EXCLUDED.permission, updated_at = CURRENT_TIMESTAMP
            "#
        )
        .bind(repo.id)
        .bind(org_id)
        .bind(&request.permission)
        .bind(granted_by)
        .execute(&state.db_pool)
        .await {
            Ok(_) => (StatusCode::NO_CONTENT).into_response(),
            Err(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                    "error": format!("Database error: {}", e)
                }))).into_response()
            }
        }
    } else {
        (StatusCode::BAD_REQUEST, Json(json!({
            "error": "Either user_id or organization_id must be provided"
        }))).into_response()
    }
}