// Registry handlers
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use utoipa::ToSchema;
use serde::{Deserialize, Serialize};
use crate::AppState;

/// Repository information
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Repository {
    /// Repository ID
    pub id: i64,
    /// Organization ID that owns this repository
    pub organization_id: i64,
    /// Repository name
    pub name: String,
    /// Repository description
    pub description: Option<String>,
    /// Repository visibility (public/private)
    pub visibility: String,
}

/// Image information
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ImageInfo {
    /// Image digest
    pub digest: String,
    /// Image tags
    pub tags: Vec<String>,
    /// Image size in bytes
    pub size_bytes: i64,
    /// When the image was pushed
    pub pushed_at: chrono::DateTime<chrono::Utc>,
}

/// List repositories for an organization
#[utoipa::path(
    get,
    path = "/api/v1/orgs/{org_id}/repos",
    tag = "registry",
    params(
        ("org_id" = i64, Path, description = "Organization ID")
    ),
    responses(
        (status = 200, description = "List of repositories", body = Vec<Repository>),
        (status = 404, description = "Organization not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn list_repositories(
    State(_state): State<AppState>,
    Path(org_id): Path<i64>,
) -> impl IntoResponse {
    // TODO: Implement actual repository listing from database
    let repositories = vec![
        Repository {
            id: 1,
            organization_id: org_id,
            name: "example-repo".to_string(),
            description: Some("Example repository".to_string()),
            visibility: "public".to_string(),
        }
    ];
    
    (StatusCode::OK, Json(repositories))
}

/// Get repository details
#[utoipa::path(
    get,
    path = "/api/v1/orgs/{org_id}/repos/{repo_name}",
    tag = "registry",
    params(
        ("org_id" = i64, Path, description = "Organization ID"),
        ("repo_name" = String, Path, description = "Repository name")
    ),
    responses(
        (status = 200, description = "Repository details", body = Repository),
        (status = 404, description = "Repository not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_repository(
    State(_state): State<AppState>,
    Path((org_id, repo_name)): Path<(i64, String)>,
) -> impl IntoResponse {
    // TODO: Implement actual repository lookup from database
    let repository = Repository {
        id: 1,
        organization_id: org_id,
        name: repo_name,
        description: Some("Example repository".to_string()),
        visibility: "public".to_string(),
    };
    
    (StatusCode::OK, Json(repository))
}

/// List images in a repository
#[utoipa::path(
    get,
    path = "/api/v1/orgs/{org_id}/repos/{repo_name}/images",
    tag = "registry",
    params(
        ("org_id" = i64, Path, description = "Organization ID"),
        ("repo_name" = String, Path, description = "Repository name")
    ),
    responses(
        (status = 200, description = "List of images", body = Vec<ImageInfo>),
        (status = 404, description = "Repository not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn list_images(
    State(_state): State<AppState>,
    Path((org_id, repo_name)): Path<(i64, String)>,
) -> impl IntoResponse {
    // TODO: Implement actual image listing from database
    let images = vec![
        ImageInfo {
            digest: "sha256:abcd1234".to_string(),
            tags: vec!["latest".to_string(), "v1.0.0".to_string()],
            size_bytes: 123456789,
            pushed_at: chrono::Utc::now(),
        }
    ];
    
    (StatusCode::OK, Json(images))
}
