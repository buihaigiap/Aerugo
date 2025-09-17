use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use axum_extra::headers::{authorization::Bearer, Authorization};
use axum_extra::TypedHeader;
use secrecy::ExposeSecret;
use utoipa::{OpenApi, ToSchema};

use crate::{
    auth::{extract_user_id, verify_token},
    database::models::{Organization, Repository},
    models::repository_with_org::RepositoryWithOrgRow,
    AppState,
};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateRepositoryRequest {
    pub name: String,
    pub description: Option<String>,
    pub is_public: bool,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RepositoryResponse {
    pub id: i64,
    pub organization_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub is_public: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub organization: OrganizationInfo,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct OrganizationInfo {
    pub id: i64,
    pub name: String,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub website_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RepositoryDetailsResponse {
    pub repository: RepositoryResponse,
    pub stats: RepositoryStats,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RepositoryStats {
    pub total_tags: i64,
    pub last_push: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ListRepositoriesQuery {
    pub namespace: Option<String>,
}

#[utoipa::path(
    get,
    path = "/api/v1/repos/repositories",
    params(
        ("namespace" = Option<String>, Query, description = "Filter by organization namespace")
    ),
    responses(
        (status = 200, description = "List of repositories", body = Vec<RepositoryResponse>),
        (status = 401, description = "Authentication required"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn list_repositories(
    State(state): State<AppState>,
    Query(query): Query<ListRepositoriesQuery>,
    auth: Option<TypedHeader<Authorization<Bearer>>>,
) -> Response {
    let secret = state.config.auth.jwt_secret.expose_secret().as_bytes();
    let user_id = match extract_user_id(auth, secret).await {
        Ok(id) => id,
        Err(_) => {
            return (StatusCode::UNAUTHORIZED, Json(json!({
                "error": "Authentication required"
            }))).into_response()
        }
    };

    let repositories = if let Some(namespace) = &query.namespace {
        // Filter by organization namespace
        let org = match sqlx::query_as::<_, Organization>(
            "SELECT * FROM organizations WHERE name = $1"
        )
        .bind(namespace)
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

        match sqlx::query_as::<_, RepositoryWithOrgRow>(
            r#"
            SELECT DISTINCT 
                r.id, r.organization_id, r.name, r.description, r.is_public, NULL::BIGINT as created_by, r.created_at, r.updated_at,
                o.id as org_id, o.name as org_name, o.display_name as org_display_name, o.description as org_description, o.website_url as org_website_url
            FROM repositories r
            JOIN organizations o ON r.organization_id = o.id
            JOIN organization_members om ON r.organization_id = om.organization_id
            WHERE om.user_id = $1
            AND o.name = $2
            "#
        )
        .bind(user_id)
        .bind(namespace)
        .fetch_all(&state.db_pool)
        .await {
            Ok(repos) => repos,
            Err(e) => {
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                    "error": format!("Database error: {}", e)
                }))).into_response()
            }
        }
    } else {
        // Get all repositories for user
        match sqlx::query_as::<_, RepositoryWithOrgRow>(
            r#"
            SELECT DISTINCT 
                r.id, r.organization_id, r.name, r.description, r.is_public, NULL::BIGINT as created_by, r.created_at, r.updated_at,
                o.id as org_id, o.name as org_name, o.display_name as org_display_name, o.description as org_description, o.website_url as org_website_url
            FROM repositories r
            JOIN organizations o ON r.organization_id = o.id
            JOIN organization_members om ON r.organization_id = om.organization_id
            WHERE om.user_id = $1
            "#
        )
        .bind(user_id)
        .fetch_all(&state.db_pool)
        .await {
            Ok(repos) => repos,
            Err(e) => {
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                    "error": format!("Database error: {}", e)
                }))).into_response()
            }
        }
    };

    let response_repos: Vec<RepositoryResponse> = repositories
        .into_iter()
        .map(|repo| RepositoryResponse {
            id: repo.id,
            organization_id: repo.organization_id,
            name: repo.name,
            description: repo.description,
            is_public: repo.is_public,
            created_at: repo.created_at,
            updated_at: repo.updated_at,
            organization: OrganizationInfo {
                id: repo.org_id,
                name: repo.org_name,
                display_name: Some(repo.org_display_name),
                description: repo.org_description,
                website_url: repo.org_website_url,
            },
        })
        .collect();

    (StatusCode::OK, Json(json!({
        "repositories": response_repos
    }))).into_response()
}

#[utoipa::path(
    post,
    path = "/api/v1/repos/{namespace}",
    params(
        ("namespace" = String, Path, description = "Organization namespace")
    ),
    request_body = CreateRepositoryRequest,
    responses(
        (status = 200, description = "Repository creation temporarily disabled"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn create_repository(
    Path(namespace): Path<String>,
    State(state): State<AppState>,
    Json(request): Json<CreateRepositoryRequest>,
) -> Response {
    // TODO: Implement proper authentication
    // For now, we'll skip authentication and use a default user
    
    // First, find the organization by name
    let org = match sqlx::query_as::<_, Organization>(
        "SELECT * FROM organizations WHERE name = $1"
    )
    .bind(&namespace)
    .fetch_optional(&state.db_pool)
    .await {
        Ok(Some(org)) => org,
        Ok(None) => {
            return (StatusCode::NOT_FOUND, Json(json!({
                "error": format!("Organization '{}' not found", namespace)
            }))).into_response()
        }
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "error": format!("Database error: {}", e)
            }))).into_response()
        }
    };

    // Check if repository already exists in this organization
    let existing_repo = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM repositories WHERE organization_id = $1 AND name = $2)"
    )
    .bind(org.id)
    .bind(&request.name)
    .fetch_one(&state.db_pool)
    .await;

    match existing_repo {
        Ok(true) => {
            return (StatusCode::CONFLICT, Json(json!({
                "error": format!("Repository '{}' already exists in organization '{}'", request.name, namespace)
            }))).into_response()
        }
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "error": format!("Database error: {}", e)
            }))).into_response()
        }
        _ => {} // Continue if repo doesn't exist
    }

    // Start a database transaction
    let mut tx = match state.db_pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "error": format!("Failed to start transaction: {}", e)
            }))).into_response()
        }
    };

    // Create the repository
    let repository = match sqlx::query_as::<_, crate::database::models::Repository>(
        "INSERT INTO repositories (organization_id, name, description, is_public, created_at, updated_at)
         VALUES ($1, $2, $3, $4, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
         RETURNING *",
    )
    .bind(org.id)
    .bind(&request.name)
    .bind(&request.description)
    .bind(request.is_public)
    .fetch_one(&mut *tx)
    .await {
        Ok(repo) => repo,
        Err(e) => {
            let _ = tx.rollback().await;
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "error": format!("Failed to create repository: {}", e)
            }))).into_response()
        }
    };

    // Commit the transaction
    if let Err(e) = tx.commit().await {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
            "error": format!("Failed to commit transaction: {}", e)
        }))).into_response()
    }

    // Return the created repository
    let response = RepositoryResponse {
        id: repository.id,
        organization_id: repository.organization_id,
        name: repository.name,
        description: repository.description,
        is_public: repository.is_public,
        created_at: repository.created_at,
        updated_at: repository.updated_at,
        organization: OrganizationInfo {
            id: org.id,
            name: org.name,
            display_name: Some(org.display_name),
            description: org.description,
            website_url: org.website_url,
        },
    };

    (StatusCode::CREATED, Json(response)).into_response()
}

#[utoipa::path(
    delete,
    path = "/api/v1/repos/{namespace}/{repo_name}",
    params(
        ("namespace" = String, Path, description = "Organization namespace"),
        ("repo_name" = String, Path, description = "Repository name")
    ),
    responses(
        (status = 200, description = "Repository deletion temporarily disabled"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn delete_repository(
    Path((namespace, repo_name)): Path<(String, String)>,
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
) -> Response {
    // Extract JWT token from Authorization header
    let auth_header = match headers.get("authorization") {
        Some(header) => header.to_str().unwrap_or(""),
        None => {
            return (StatusCode::UNAUTHORIZED, Json(json!({
                "error": "Missing authorization header"
            }))).into_response()
        }
    };

    let token = if let Some(token) = auth_header.strip_prefix("Bearer ") {
        token
    } else {
        return (StatusCode::UNAUTHORIZED, Json(json!({
            "error": "Invalid authorization header format"
        }))).into_response()
    };

    // Verify JWT token and get user_id
    let claims = match crate::auth::verify_token(token, state.config.auth.jwt_secret.expose_secret().as_bytes()) {
        Ok(claims) => claims,
        Err(_) => {
            return (StatusCode::UNAUTHORIZED, Json(json!({
                "error": "Invalid or expired token"
            }))).into_response()
        }
    };

    let user_id: i64 = match claims.sub.parse() {
        Ok(id) => id,
        Err(_) => {
            return (StatusCode::UNAUTHORIZED, Json(json!({
                "error": "Invalid user ID in token"
            }))).into_response()
        }
    };

    // Start a database transaction
    let mut tx = match state.db_pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "error": format!("Database transaction error: {}", e)
            }))).into_response()
        }
    };

    // Find organization by namespace
    let org = match sqlx::query_as::<_, Organization>(
        "SELECT id, name, display_name, description, website_url, avatar_url, created_at, updated_at FROM organizations WHERE name = $1"
    )
    .bind(&namespace)
    .fetch_optional(&mut *tx)
    .await {
        Ok(Some(org)) => org,
        Ok(None) => {
            return (StatusCode::NOT_FOUND, Json(json!({
                "error": format!("Organization '{}' not found", namespace)
            }))).into_response()
        }
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "error": format!("Database error: {}", e)
            }))).into_response()
        }
    };

    // Check if repository exists
    let repository = match sqlx::query_as::<_, Repository>(
        "SELECT id, organization_id, name, description, is_public, created_at, updated_at, created_by FROM repositories WHERE organization_id = $1 AND name = $2"
    )
    .bind(org.id)
    .bind(&repo_name)
    .fetch_optional(&mut *tx)
    .await {
        Ok(Some(repo)) => repo,
        Ok(None) => {
            return (StatusCode::NOT_FOUND, Json(json!({
                "error": format!("Repository '{}/{}' not found", namespace, repo_name)
            }))).into_response()
        }
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "error": format!("Database error: {}", e)
            }))).into_response()
        }
    };

    // Check if user has permission to delete the repository
    // User must be organization member to delete repositories
    let has_permission = match sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM organization_members WHERE organization_id = $1 AND user_id = $2)"
    )
    .bind(org.id)
    .bind(user_id)
    .fetch_one(&mut *tx)
    .await {
        Ok(has_perm) => has_perm,
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "error": format!("Permission check error: {}", e)
            }))).into_response()
        }
    };

    if !has_permission {
        return (StatusCode::FORBIDDEN, Json(json!({
            "error": format!("You don't have permission to delete repositories in organization '{}'", namespace)
        }))).into_response()
    }

    // Delete the repository
    match sqlx::query("DELETE FROM repositories WHERE id = $1")
        .bind(repository.id)
        .execute(&mut *tx)
        .await {
        Ok(_) => {},
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "error": format!("Failed to delete repository: {}", e)
            }))).into_response()
        }
    }

    // Commit transaction
    if let Err(e) = tx.commit().await {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
            "error": format!("Transaction commit error: {}", e)
        }))).into_response()
    }

    (StatusCode::OK, Json(json!({
        "message": format!("Repository '{}/{}' deleted successfully", namespace, repo_name)
    }))).into_response()
}
