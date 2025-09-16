use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    auth::get_user_id_from_request,
    database::{
        models::{Organization, Repository, RepositoryWithOrgRow, User},
        queries::{get_organization_by_name, get_repositories_for_user, get_repository_by_name},
    },
    AppState,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRepositoryRequest {
    pub name: String,
    pub description: Option<String>,
    pub is_public: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RepositoryPermissionsRequest {
    pub user_id: Option<i64>,
    pub organization_id: Option<i64>,
    pub can_read: bool,
    pub can_write: bool,
    pub can_admin: bool,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizationInfo {
    pub id: i64,
    pub name: String,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub website_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RepositoryDetailsResponse {
    pub repository: RepositoryResponse,
    pub stats: RepositoryStats,
    pub permissions: RepositoryPermissionsInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RepositoryStats {
    pub total_images: i64,
    pub total_tags: i64,
    pub last_push: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RepositoryPermissionsInfo {
    pub can_read: bool,
    pub can_write: bool,
    pub can_admin: bool,
}

#[derive(Debug, Deserialize)]
pub struct ListRepositoriesQuery {
    pub namespace: Option<String>,
}

pub async fn list_repositories(
    State(state): State<AppState>,
    Query(query): Query<ListRepositoriesQuery>,
) -> Response {
    let user_id = match get_user_id_from_request(&state).await {
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
            LEFT JOIN repository_permissions rp ON r.id = rp.repository_id
            LEFT JOIN organization_members om ON r.organization_id = om.organization_id
            WHERE (rp.user_id = $1 OR om.user_id = $1)
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
            LEFT JOIN repository_permissions rp ON r.id = rp.repository_id
            LEFT JOIN organization_members om ON r.organization_id = om.organization_id
            WHERE (rp.user_id = $1 OR om.user_id = $1)
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
                display_name: repo.org_display_name,
                description: repo.org_description,
                website_url: repo.org_website_url,
            },
        })
        .collect();

    (StatusCode::OK, Json(json!({
        "repositories": response_repos
    }))).into_response()
}

pub async fn create_repository(
    Path(namespace): Path<String>,
    State(state): State<AppState>,
    Json(request): Json<CreateRepositoryRequest>,
) -> Response {
    (StatusCode::OK, Json(json!({
        "message": "Repository creation temporarily disabled"
    }))).into_response()
}

pub async fn get_repository_details(
    Path((namespace, repo_name)): Path<(String, String)>,
    State(state): State<AppState>,
) -> Response {
    (StatusCode::OK, Json(json!({
        "message": "Repository details temporarily disabled"
    }))).into_response()
}

pub async fn delete_repository(
    Path((namespace, repo_name)): Path<(String, String)>,
    State(state): State<AppState>,
) -> Response {
    (StatusCode::OK, Json(json!({
        "message": "Repository deletion temporarily disabled"
    }))).into_response()
}

pub async fn update_repository_permissions(
    Path((namespace, repo_name)): Path<(String, String)>,
    State(state): State<AppState>,
    Json(request): Json<RepositoryPermissionsRequest>,
) -> Response {
    (StatusCode::OK, Json(json!({
        "message": "Repository permissions temporarily disabled"
    }))).into_response()
}
