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

/// Docker build request
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DockerBuildRequest {
    /// Path to Dockerfile or build context
    pub dockerfile_path: String,
    /// Image name and tag
    pub image_tag: String,
    /// Build arguments
    pub build_args: Option<std::collections::HashMap<String, String>>,
}

/// Docker push request
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DockerPushRequest {
    /// Image name and tag to push
    pub image_tag: String,
    /// Registry URL (optional, defaults to Docker Hub)
    pub registry_url: Option<String>,
}

/// Docker build and S3 upload request
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DockerBuildUploadS3Request {
    /// Path to Dockerfile or build context
    pub dockerfile_path: String,
    /// Image name and tag
    pub image_tag: String,
    /// S3 bucket name
    pub s3_bucket: String,
    /// S3 object key (file path in S3)
    pub s3_key: String,
    /// Build arguments
    pub build_args: Option<std::collections::HashMap<String, String>>,
}

/// S3 upload request
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct S3UploadRequest {
    /// Local file path to upload
    pub file_path: String,
    /// S3 bucket name
    pub bucket: String,
    /// S3 object key
    pub key: String,
}

/// S3 download request
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct S3DownloadRequest {
    /// S3 bucket name
    pub bucket: String,
    /// S3 object key
    pub key: String,
    /// Local path to save file
    pub local_path: String,
}

/// S3 delete request
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct S3DeleteRequest {
    /// S3 bucket name
    pub bucket: String,
    /// S3 object key
    pub key: String,
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
    Path((_org_id, _repo_name)): Path<(i64, String)>,
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

/// Build a Docker image
#[utoipa::path(
    post,
    path = "/api/v1/docker/build",
    tag = "docker",
    request_body = DockerBuildRequest,
    responses(
        (status = 200, description = "Build successful"),
        (status = 500, description = "Build failed")
    )
)]
pub async fn docker_build(
    Json(request): Json<DockerBuildRequest>,
) -> impl axum::response::IntoResponse {
    use std::process::Command;
    
    // Build Docker command
    let mut cmd = Command::new("docker");
    cmd.arg("build")
       .arg("-t")
       .arg(&request.image_tag)
       .arg(&request.dockerfile_path);
    
    // Add build arguments if provided
    if let Some(build_args) = &request.build_args {
        for (key, value) in build_args {
            cmd.arg("--build-arg").arg(format!("{}={}", key, value));
        }
    }
    
    // Execute Docker build
    match cmd.output() {
        Ok(output) => {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                (axum::http::StatusCode::OK, Json(serde_json::json!({
                    "success": true,
                    "message": "Docker image built successfully",
                    "image_tag": request.image_tag,
                    "output": stdout.to_string()
                })))
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                    "success": false,
                    "message": "Docker build failed",
                    "error": stderr.to_string()
                })))
            }
        }
        Err(e) => {
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "success": false,
                "message": "Failed to execute docker build command",
                "error": e.to_string()
            })))
        }
    }
}

/// Push a Docker image
#[utoipa::path(
    post,
    path = "/api/v1/docker/push",
    tag = "docker",
    request_body = DockerPushRequest,
    responses(
        (status = 200, description = "Push successful"),
        (status = 500, description = "Push failed")
    )
)]
pub async fn docker_push(
    Json(request): Json<DockerPushRequest>,
) -> impl axum::response::IntoResponse {
    use std::process::Command;
    
    let image_to_push = if let Some(registry) = &request.registry_url {
        format!("{}/{}", registry, request.image_tag)
    } else {
        request.image_tag.clone()
    };
    
    // Execute Docker push
    let output = Command::new("docker")
        .arg("push")
        .arg(&image_to_push)
        .output();
    
    match output {
        Ok(result) => {
            if result.status.success() {
                let stdout = String::from_utf8_lossy(&result.stdout);
                (axum::http::StatusCode::OK, Json(serde_json::json!({
                    "success": true,
                    "message": "Docker image pushed successfully",
                    "image_tag": image_to_push,
                    "output": stdout.to_string()
                })))
            } else {
                let stderr = String::from_utf8_lossy(&result.stderr);
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                    "success": false,
                    "message": "Docker push failed",
                    "error": stderr.to_string()
                })))
            }
        }
        Err(e) => {
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "success": false,
                "message": "Failed to execute docker push command",
                "error": e.to_string()
            })))
        }
    }
}

/// Pull a Docker image
#[utoipa::path(
    post,
    path = "/api/v1/docker/pull",
    tag = "docker",
    responses(
        (status = 200, description = "Pull successful"),
        (status = 500, description = "Pull failed")
    )
)]
pub async fn docker_pull() -> impl axum::response::IntoResponse {
    // TODO: Implement Docker pull logic
    (axum::http::StatusCode::OK, "Docker image pull stub")
}

/// Build Docker image and upload to S3
#[utoipa::path(
    post,
    path = "/api/v1/docker/build-upload-s3",
    tag = "docker",
    request_body = DockerBuildUploadS3Request,
    responses(
        (status = 200, description = "Build and upload to S3 successful"),
        (status = 500, description = "Failed")
    )
)]
pub async fn docker_build_upload_s3(
    Json(request): Json<DockerBuildUploadS3Request>,
) -> impl axum::response::IntoResponse {
    use std::process::Command;
    
    // Step 1: Build Docker image
    let mut build_cmd = Command::new("docker");
    build_cmd.arg("build")
             .arg("-t")
             .arg(&request.image_tag)
             .arg(&request.dockerfile_path);
    
    if let Some(build_args) = &request.build_args {
        for (key, value) in build_args {
            build_cmd.arg("--build-arg").arg(format!("{}={}", key, value));
        }
    }
    
    let build_result = build_cmd.output();
    
    match build_result {
        Ok(output) => {
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                    "success": false,
                    "step": "build",
                    "message": "Docker build failed",
                    "error": stderr.to_string()
                })));
            }
        }
        Err(e) => {
            return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "success": false,
                "step": "build",
                "message": "Failed to execute docker build",
                "error": e.to_string()
            })));
        }
    }
    
    // Step 2: Save Docker image to tar file
    let temp_file = format!("/tmp/{}.tar", request.image_tag.replace(":", "_"));
    
    let save_result = Command::new("docker")
        .arg("save")
        .arg(&request.image_tag)
        .arg("-o")
        .arg(&temp_file)
        .output();
    
    match save_result {
        Ok(output) => {
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                    "success": false,
                    "step": "save",
                    "message": "Docker save failed",
                    "error": stderr.to_string()
                })));
            }
        }
        Err(e) => {
            return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "success": false,
                "step": "save",
                "message": "Failed to execute docker save",
                "error": e.to_string()
            })));
        }
    }
    
    // Step 3: Upload to S3 (using AWS CLI)
    let upload_result = Command::new("aws")
        .arg("s3")
        .arg("cp")
        .arg(&temp_file)
        .arg(format!("s3://{}/{}", request.s3_bucket, request.s3_key))
        .output();
    
    // Clean up temp file
    let _ = std::fs::remove_file(&temp_file);
    
    match upload_result {
        Ok(output) => {
            if output.status.success() {
                (axum::http::StatusCode::OK, Json(serde_json::json!({
                    "success": true,
                    "message": "Docker image built and uploaded to S3 successfully",
                    "image_tag": request.image_tag,
                    "s3_location": format!("s3://{}/{}", request.s3_bucket, request.s3_key)
                })))
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                    "success": false,
                    "step": "upload",
                    "message": "S3 upload failed",
                    "error": stderr.to_string()
                })))
            }
        }
        Err(e) => {
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "success": false,
                "step": "upload",
                "message": "Failed to execute S3 upload",
                "error": e.to_string()
            })))
        }
    }
}

/// Upload file to S3
#[utoipa::path(
    post,
    path = "/api/v1/s3/upload",
    tag = "s3",
    request_body = S3UploadRequest,
    responses(
        (status = 200, description = "Upload successful"),
        (status = 500, description = "Upload failed")
    )
)]
pub async fn s3_upload(
    Json(request): Json<S3UploadRequest>,
) -> impl axum::response::IntoResponse {
    use std::process::Command;
    
    let output = Command::new("aws")
        .arg("s3")
        .arg("cp")
        .arg(&request.file_path)
        .arg(format!("s3://{}/{}", request.bucket, request.key))
        .output();
    
    match output {
        Ok(result) => {
            if result.status.success() {
                (axum::http::StatusCode::OK, Json(serde_json::json!({
                    "success": true,
                    "message": "File uploaded to S3 successfully",
                    "s3_location": format!("s3://{}/{}", request.bucket, request.key)
                })))
            } else {
                let stderr = String::from_utf8_lossy(&result.stderr);
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                    "success": false,
                    "message": "S3 upload failed",
                    "error": stderr.to_string()
                })))
            }
        }
        Err(e) => {
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "success": false,
                "message": "Failed to execute S3 upload command",
                "error": e.to_string()
            })))
        }
    }
}

/// Download file from S3
#[utoipa::path(
    post,
    path = "/api/v1/s3/download",
    tag = "s3",
    request_body = S3DownloadRequest,
    responses(
        (status = 200, description = "Download successful"),
        (status = 404, description = "File not found"),
        (status = 500, description = "Download failed")
    )
)]
pub async fn s3_download(
    Json(request): Json<S3DownloadRequest>,
) -> impl axum::response::IntoResponse {
    use std::process::Command;
    
    let output = Command::new("aws")
        .arg("s3")
        .arg("cp")
        .arg(format!("s3://{}/{}", request.bucket, request.key))
        .arg(&request.local_path)
        .output();
    
    match output {
        Ok(result) => {
            if result.status.success() {
                (axum::http::StatusCode::OK, Json(serde_json::json!({
                    "success": true,
                    "message": "File downloaded from S3 successfully",
                    "local_path": request.local_path
                })))
            } else {
                let stderr = String::from_utf8_lossy(&result.stderr);
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                    "success": false,
                    "message": "S3 download failed",
                    "error": stderr.to_string()
                })))
            }
        }
        Err(e) => {
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "success": false,
                "message": "Failed to execute S3 download command",
                "error": e.to_string()
            })))
        }
    }
}

/// S3 List request for query parameters
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct S3ListRequest {
    /// S3 bucket name
    pub bucket: String,
    /// Prefix to filter files (optional)
    pub prefix: Option<String>,
}

/// S3 Object information
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct S3Object {
    /// Object key (file path)
    pub key: String,
    /// File size in bytes
    pub size: i64,
    /// Last modified time
    pub last_modified: String,
    /// ETag (checksum)
    pub etag: String,
}

/// List files in S3
#[utoipa::path(
    post,
    path = "/api/v1/s3/list",
    tag = "s3",
    request_body = S3ListRequest,
    responses(
        (status = 200, description = "List files successfully", body = Vec<S3Object>),
        (status = 500, description = "Error while listing files")
    )
)]
pub async fn s3_list(
    Json(request): Json<S3ListRequest>,
) -> impl axum::response::IntoResponse {
    // Use AWS SDK for Rust (rusoto_s3) to list objects
    use rusoto_core::Region;
    use rusoto_s3::{S3Client, S3, ListObjectsV2Request};
    use tokio_stream::StreamExt;

    let s3_client = S3Client::new(Region::Custom {
        name: "minio".to_string(),
        endpoint: "http://localhost:9001".to_string(),
    });

    let prefix = request.prefix.clone().unwrap_or_default();
    let list_req = ListObjectsV2Request {
        bucket: request.bucket.clone(),
        prefix: if prefix.is_empty() { None } else { Some(prefix) },
        ..Default::default()
    };

    match s3_client.list_objects_v2(list_req).await {
        Ok(output) => {
            let mut objects = Vec::new();
            if let Some(contents) = output.contents {
                for obj in contents {
                    objects.push(S3Object {
                        key: obj.key.unwrap_or_default(),
                        size: obj.size.unwrap_or(0),
                        last_modified: obj.last_modified.unwrap_or_default(),
                        etag: obj.e_tag.unwrap_or_default(),
                    });
                }
            }
            (axum::http::StatusCode::OK, Json(serde_json::json!({
                "success": true,
                "message": format!("Found {} objects in bucket '{}'", objects.len(), request.bucket),
                "bucket": request.bucket,
                "prefix": request.prefix,
                "objects": objects
            })))
        }
        Err(e) => {
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "success": false,
                "message": "Failed to list S3 objects",
                "error": e.to_string()
            })))
        }
    }
}

/// Delete file from S3
#[utoipa::path(
    delete,
    path = "/api/v1/s3/delete",
    tag = "s3",
    request_body = S3DeleteRequest,
    responses(
        (status = 200, description = "Delete file successful"),
        (status = 404, description = "File not found"),
        (status = 500, description = "Delete file failed")
    )
)]
pub async fn s3_delete(
    Json(request): Json<S3DeleteRequest>,
) -> impl axum::response::IntoResponse {
    use std::process::Command;
    
    let output = Command::new("aws")
        .arg("s3")
        .arg("rm")
        .arg(format!("s3://{}/{}", request.bucket, request.key))
        .output();
    
    match output {
        Ok(result) => {
            if result.status.success() {
                (axum::http::StatusCode::OK, Json(serde_json::json!({
                    "success": true,
                    "message": "File deleted from S3 successfully"
                })))
            } else {
                let stderr = String::from_utf8_lossy(&result.stderr);
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                    "success": false,
                    "message": "S3 delete failed",
                    "error": stderr.to_string()
                })))
            }
        }
        Err(e) => {
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "success": false,
                "message": "Failed to execute S3 delete command",
                "error": e.to_string()
            })))
        }
    }
}
