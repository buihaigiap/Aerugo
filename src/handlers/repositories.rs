use axum::{
    extract::{Path, Query, State},
    http::{StatusCode, HeaderMap},
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
    auth::{extract_user_id_dual, extract_user_id, verify_token},
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
pub struct UpdateRepositoryRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_public: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RepositoryResponse {
    pub id: i64,
    pub organization_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub is_public: bool,
    pub created_by: Option<i64>,
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
    headers: HeaderMap,
    auth: Option<TypedHeader<Authorization<Bearer>>>,
) -> Response {
    let secret = state.config.auth.jwt_secret.expose_secret().as_bytes();
    
    let user_id = match extract_user_id_dual(
        auth, 
        &headers, 
        secret, 
        &state.db_pool, 
        state.cache.as_ref()
    ).await {
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
                r.id, r.organization_id, r.name, r.description, r.is_public, r.created_by, r.created_at, r.updated_at,
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
                r.id, r.organization_id, r.name, r.description, r.is_public, r.created_by, r.created_at, r.updated_at,
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
            created_by: repo.created_by,
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

    // If a namespace is specified, return a direct list for compatibility
    // If no namespace (all repositories), return wrapped in "repositories" object
    if query.namespace.is_some() {
        (StatusCode::OK, Json(json!(response_repos))).into_response()
    } else {
        (StatusCode::OK, Json(json!({
            "repositories": response_repos
        }))).into_response()
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/repos/repositories/{namespace}",
    params(
        ("namespace" = String, Path, description = "Organization namespace")
    ),
    responses(
        (status = 200, description = "List of repositories in namespace", body = Vec<RepositoryResponse>),
        (status = 401, description = "Authentication required"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn list_repositories_by_namespace(
    Path(namespace): Path<String>,
    State(state): State<AppState>,
    headers: HeaderMap,
    auth: Option<TypedHeader<Authorization<Bearer>>>,
) -> Response {
    let secret = state.config.auth.jwt_secret.expose_secret().as_bytes();
    
    let user_id = match extract_user_id_dual(
        auth, 
        &headers, 
        secret, 
        &state.db_pool, 
        state.cache.as_ref()
    ).await {
        Ok(id) => id,
        Err(_) => {
            return (StatusCode::UNAUTHORIZED, Json(json!({
                "error": "Authentication required"
            }))).into_response()
        }
    };

    // Find the organization by name
    let _org = match sqlx::query_as::<_, Organization>(
        "SELECT * FROM organizations WHERE name = $1"
    )
    .bind(&namespace)
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

    // Get repositories for the specific namespace
    let repositories = match sqlx::query_as::<_, RepositoryWithOrgRow>(
        r#"
        SELECT DISTINCT 
            r.id, r.organization_id, r.name, r.description, r.is_public, r.created_by, r.created_at, r.updated_at,
            o.id as org_id, o.name as org_name, o.display_name as org_display_name, o.description as org_description, o.website_url as org_website_url
        FROM repositories r
        JOIN organizations o ON r.organization_id = o.id
        JOIN organization_members om ON r.organization_id = om.organization_id
        WHERE om.user_id = $1
        AND o.name = $2
        "#
    )
    .bind(user_id)
    .bind(&namespace)
    .fetch_all(&state.db_pool)
    .await {
        Ok(repos) => repos,
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "error": format!("Database error: {}", e)
            }))).into_response()
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
            created_by: repo.created_by,
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

    // Return direct list for namespace-specific endpoint
    (StatusCode::OK, Json(json!(response_repos))).into_response()
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
    headers: HeaderMap,
    auth: Option<TypedHeader<Authorization<Bearer>>>,
    Json(request): Json<CreateRepositoryRequest>,
) -> Response {
    // Extract user ID from JWT token or API key
    let secret = state.config.auth.jwt_secret.expose_secret().as_bytes();
    
    let user_id = match extract_user_id_dual(
        auth, 
        &headers, 
        secret, 
        &state.db_pool, 
        state.cache.as_ref()
    ).await {
        Ok(id) => id,
        Err(_) => {
            return (StatusCode::UNAUTHORIZED, Json(json!({
                "error": "Authentication required"
            }))).into_response()
        }
    };
    
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
        "INSERT INTO repositories (organization_id, name, description, is_public, created_by, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
         RETURNING *",
    )
    .bind(org.id)
    .bind(&request.name)
    .bind(&request.description)
    .bind(request.is_public)
    .bind(user_id)
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
        created_by: repository.created_by,
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

/// Update repository information
#[utoipa::path(
    put,
    path = "/api/v1/repos/{namespace}/{repo_name}",
    params(
        ("namespace" = String, Path, description = "Organization namespace"),
        ("repo_name" = String, Path, description = "Repository name")
    ),
    request_body = UpdateRepositoryRequest,
    responses(
        (status = 200, description = "Repository updated successfully", body = RepositoryResponse),
        (status = 400, description = "Invalid request - empty name, invalid format, or name already exists"),
        (status = 401, description = "Authentication required"),
        (status = 403, description = "Permission denied"),
        (status = 404, description = "Repository not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn update_repository(
    Path((namespace, repo_name)): Path<(String, String)>,
    State(state): State<AppState>,
    headers: HeaderMap,
    auth: Option<TypedHeader<Authorization<Bearer>>>,
    Json(request): Json<UpdateRepositoryRequest>,
) -> Response {
    // Extract user ID from JWT token
    let user_id = match extract_user_id(auth, state.config.auth.jwt_secret.expose_secret().as_bytes()).await {
        Ok(id) => id,
        Err(e) => {
            return (StatusCode::UNAUTHORIZED, Json(json!({
                "error": format!("Authentication error: {}", e)
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

    // Check if user has permission to update the repository
    // User must be organization member to update repositories
    let is_member = match sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM organization_members WHERE organization_id = $1 AND user_id = $2)"
    )
    .bind(org.id)
    .bind(user_id)
    .fetch_one(&mut *tx)
    .await {
        Ok(exists) => exists,
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "error": format!("Database error checking permissions: {}", e)
            }))).into_response()
        }
    };

    if !is_member {
        return (StatusCode::FORBIDDEN, Json(json!({
            "error": "You don't have permission to update repositories in this organization"
        }))).into_response()
    }

    // Validate repository name if provided (should be unique in organization)
    if let Some(name) = &request.name {
        // Basic name validation
        if name.trim().is_empty() {
            return (StatusCode::BAD_REQUEST, Json(json!({
                "error": "Repository name cannot be empty"
            }))).into_response()
        }

        // Validate name format (allow alphanumeric, hyphens, underscores, dots)
        if !name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == '.') {
            return (StatusCode::BAD_REQUEST, Json(json!({
                "error": "Repository name can only contain letters, numbers, hyphens, underscores, and dots"
            }))).into_response()
        }

        // Check if new name already exists in the organization (and it's not the current repository)
        if name != &repository.name {
            let name_exists = match sqlx::query_scalar::<_, bool>(
                "SELECT EXISTS(SELECT 1 FROM repositories WHERE organization_id = $1 AND name = $2 AND id != $3)"
            )
            .bind(org.id)
            .bind(name)
            .bind(repository.id)
            .fetch_one(&mut *tx)
            .await {
                Ok(exists) => exists,
                Err(e) => {
                    return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                        "error": format!("Database error checking name uniqueness: {}", e)
                    }))).into_response()
                }
            };

            if name_exists {
                return (StatusCode::BAD_REQUEST, Json(json!({
                    "error": format!("Repository with name '{}' already exists in organization '{}'", name, namespace)
                }))).into_response()
            }
        }
    }

    // Build dynamic update query based on provided fields
    let mut update_fields = Vec::new();
    let mut query_params = Vec::new();
    let mut param_counter = 1;

    if let Some(name) = &request.name {
        update_fields.push(format!("name = ${}", param_counter));
        query_params.push(name.clone());
        param_counter += 1;
    }

    if let Some(description) = &request.description {
        update_fields.push(format!("description = ${}", param_counter));
        query_params.push(description.clone());
        param_counter += 1;
    }

    if let Some(is_public) = request.is_public {
        update_fields.push(format!("is_public = ${}", param_counter));
        query_params.push(is_public.to_string());
        param_counter += 1;
    }

    // Always update the updated_at timestamp
    update_fields.push("updated_at = CURRENT_TIMESTAMP".to_string());

    if update_fields.len() == 1 {  // Only has updated_at field
        return (StatusCode::BAD_REQUEST, Json(json!({
            "error": "No fields to update provided"
        }))).into_response()
    }

    // Build and execute update query
    let update_query = format!(
        "UPDATE repositories SET {} WHERE id = ${} RETURNING *",
        update_fields.join(", "),
        param_counter
    );

    let mut query = sqlx::query_as::<_, Repository>(&update_query);
    
    // Bind parameters in the same order they were added
    if let Some(name) = &request.name {
        query = query.bind(name);
    }
    if let Some(description) = &request.description {
        query = query.bind(description);
    }
    if let Some(is_public) = request.is_public {
        query = query.bind(is_public);
    }
    query = query.bind(repository.id);

    let updated_repository = match query.fetch_one(&mut *tx).await {
        Ok(repo) => repo,
        Err(e) => {
            let _ = tx.rollback().await;
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "error": format!("Failed to update repository: {}", e)
            }))).into_response()
        }
    };

    // Commit the transaction
    if let Err(e) = tx.commit().await {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
            "error": format!("Failed to commit transaction: {}", e)
        }))).into_response()
    }

    // Return the updated repository
    let response = RepositoryResponse {
        id: updated_repository.id,
        organization_id: updated_repository.organization_id,
        name: updated_repository.name,
        description: updated_repository.description,
        is_public: updated_repository.is_public,
        created_by: updated_repository.created_by,
        created_at: updated_repository.created_at,
        updated_at: updated_repository.updated_at,
        organization: OrganizationInfo {
            id: org.id,
            name: org.name,
            display_name: Some(org.display_name),
            description: org.description,
            website_url: org.website_url,
        },
    };

    (StatusCode::OK, Json(response)).into_response()
}

#[utoipa::path(
    delete,
    path = "/api/v1/repos/{namespace}/{repo_name}",
    params(
        ("namespace" = String, Path, description = "Organization namespace"),
        ("repo_name" = String, Path, description = "Repository name")
    ),
    responses(
        (status = 200, description = "Repository deleted successfully"),
        (status = 401, description = "Authentication required"),
        (status = 403, description = "Permission denied"),
        (status = 404, description = "Repository not found"),
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

    // Return 200 OK with success message
    (StatusCode::OK, Json(json!({
        "message": format!("Repository '{}/{}' has been deleted successfully", namespace, repo_name)
    }))).into_response()
}

#[utoipa::path(
    get,
    path = "/api/v1/repos/{namespace}/repositories/{repo_name}",
    params(
        ("namespace" = String, Path, description = "Organization namespace"),
        ("repo_name" = String, Path, description = "Repository name")
    ),
    responses(
        (status = 200, description = "Repository details"),
        (status = 404, description = "Repository not found"),
        (status = 401, description = "Authentication required"),
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn get_repository(
    Path((namespace, repo_name)): Path<(String, String)>,
    State(state): State<AppState>,
    headers: HeaderMap,
    auth: Option<TypedHeader<Authorization<Bearer>>>,
) -> Response {
    // Extract user ID from JWT token or API key
    let secret = state.config.auth.jwt_secret.expose_secret().as_bytes();
    
    let user_id = match extract_user_id_dual(
        auth, 
        &headers, 
        secret, 
        &state.db_pool, 
        state.cache.as_ref()
    ).await {
        Ok(id) => id,
        Err(_) => {
            return (StatusCode::UNAUTHORIZED, Json(json!({
                "error": "Authentication required"
            }))).into_response()
        }
    };

    // Find the organization by name
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

    // Find the repository
    let repository = match sqlx::query_as::<_, crate::database::models::Repository>(
        "SELECT * FROM repositories WHERE organization_id = $1 AND name = $2"
    )
    .bind(org.id)
    .bind(&repo_name)
    .fetch_optional(&state.db_pool)
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

    // Check if user has access to this repository (member of organization)
    let has_access = match sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM organization_members WHERE organization_id = $1 AND user_id = $2)"
    )
    .bind(org.id)
    .bind(user_id)
    .fetch_one(&state.db_pool)
    .await {
        Ok(has_access) => has_access,
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "error": format!("Permission check error: {}", e)
            }))).into_response()
        }
    };

    // If repository is private, check access permissions
    if !repository.is_public && !has_access {
        return (StatusCode::NOT_FOUND, Json(json!({
            "error": format!("Repository '{}/{}' not found", namespace, repo_name)
        }))).into_response()
    }

    // Get repository tags (for now return empty list)
    let tags: Vec<String> = vec![];

    // Build user permissions (simplified)
    let user_permissions = if has_access {
        vec![json!({
            "user_id": user_id,
            "permission": "admin"
        })]
    } else {
        vec![]
    };

    // Build org permissions (simplified)
    let org_permissions = vec![json!({
        "organization_id": org.id,
        "permission": "read"
    })];

    let response = RepositoryResponse {
        id: repository.id,
        organization_id: repository.organization_id,
        name: repository.name,
        description: repository.description,
        is_public: repository.is_public,
        created_by: repository.created_by,
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

    (StatusCode::OK, Json(json!({
        "repository": response,
        "tags": tags,
        "user_permissions": user_permissions,
        "org_permissions": org_permissions
    }))).into_response()
}

/// List public repositories (is_public = true) - No authentication required
#[utoipa::path(
    get,
    path = "/api/v1/repos/repositories/public",
    params(
        ("namespace" = Option<String>, Query, description = "Filter by organization namespace")
    ),
    responses(
        (status = 200, description = "Public repositories retrieved successfully", body = Vec<RepositoryResponse>),
        (status = 500, description = "Internal server error")
    ),
    tag = "repositories",
    security()
)]
pub async fn list_public_repositories(
    State(state): State<AppState>,
    Query(query): Query<ListRepositoriesQuery>,
) -> Response {
    let repositories = if let Some(namespace) = &query.namespace {
        // Filter by organization namespace and is_public = true
        match sqlx::query_as::<_, RepositoryWithOrgRow>(
            r#"
            SELECT 
                r.id, r.organization_id, r.name, r.description, r.is_public, r.created_by, r.created_at, r.updated_at,
                o.id as org_id, o.name as org_name, o.display_name as org_display_name, o.description as org_description, o.website_url as org_website_url
            FROM repositories r
            JOIN organizations o ON r.organization_id = o.id
            WHERE r.is_public = true
            AND o.name = $1
            ORDER BY r.created_at DESC
            "#
        )
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
        // Get all public repositories (is_public = true)
        match sqlx::query_as::<_, RepositoryWithOrgRow>(
            r#"
            SELECT 
                r.id, r.organization_id, r.name, r.description, r.is_public, r.created_by, r.created_at, r.updated_at,
                o.id as org_id, o.name as org_name, o.display_name as org_display_name, o.description as org_description, o.website_url as org_website_url
            FROM repositories r
            JOIN organizations o ON r.organization_id = o.id
            WHERE r.is_public = true
            ORDER BY r.created_at DESC
            "#
        )
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

    let response_repositories: Vec<RepositoryResponse> = repositories
        .into_iter()
        .map(|repo| RepositoryResponse {
            id: repo.id,
            organization_id: repo.organization_id,
            name: repo.name,
            description: repo.description,
            is_public: repo.is_public,
            created_by: repo.created_by,
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
        "repositories": response_repositories,
        "total": response_repositories.len()
    }))).into_response()
}
