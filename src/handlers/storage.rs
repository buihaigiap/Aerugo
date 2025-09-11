use axum::{
    extract::Path,
    http::{StatusCode, header},
    response::{Json, Response},
    body::Body,
};
use axum_extra::extract::Multipart;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Request/Response structures
#[derive(Deserialize)]
pub struct UploadRequest {
    pub digest: String,
}

#[derive(Serialize)]
pub struct UploadResponse {
    pub success: bool,
    pub message: String,
    pub digest: String,
}

#[derive(Serialize)]
pub struct ExistsResponse {
    pub exists: bool,
    pub digest: String,
}

#[derive(Serialize, Clone)]
pub struct BlobMetadataResponse {
    pub size: u64,
    pub digest: String,
    pub created_at: String,
    pub content_type: Option<String>,
}

#[derive(Serialize)]
pub struct DeleteResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub message: String,
}

// Mock storage for testing - in production this would use actual storage backend
static mut MOCK_STORAGE: Option<HashMap<String, Vec<u8>>> = None;
static mut MOCK_METADATA: Option<HashMap<String, BlobMetadataResponse>> = None;

fn get_mock_storage() -> &'static mut HashMap<String, Vec<u8>> {
    unsafe {
        if MOCK_STORAGE.is_none() {
            MOCK_STORAGE = Some(HashMap::new());
        }
        MOCK_STORAGE.as_mut().unwrap()
    }
}

fn get_mock_metadata() -> &'static mut HashMap<String, BlobMetadataResponse> {
    unsafe {
        if MOCK_METADATA.is_none() {
            MOCK_METADATA = Some(HashMap::new());
        }
        MOCK_METADATA.as_mut().unwrap()
    }
}

pub async fn upload_blob(mut multipart: Multipart) -> Result<Json<UploadResponse>, StatusCode> {
    let mut digest = String::new();
    let mut file_data: Vec<u8> = Vec::new();

    while let Some(field) = multipart.next_field().await.map_err(|_| StatusCode::BAD_REQUEST)? {
        let name = field.name().unwrap_or("").to_string();
        
        if name == "digest" {
            digest = field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?;
        } else if name == "file" {
            file_data = field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?.to_vec();
        }
    }

    if digest.is_empty() || file_data.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Store in mock storage
    let storage = get_mock_storage();
    let metadata_store = get_mock_metadata();
    
    storage.insert(digest.clone(), file_data.clone());
    metadata_store.insert(digest.clone(), BlobMetadataResponse {
        size: file_data.len() as u64,
        digest: digest.clone(),
        created_at: chrono::Utc::now().to_rfc3339(),
        content_type: Some("application/octet-stream".to_string()),
    });

    Ok(Json(UploadResponse {
        success: true,
        message: "Blob uploaded successfully".to_string(),
        digest,
    }))
}

pub async fn download_blob(Path(digest): Path<String>) -> Result<Response<Body>, StatusCode> {
    let storage = get_mock_storage();
    
    if let Some(data) = storage.get(&digest) {
        let response = Response::builder()
            .header(header::CONTENT_TYPE, "application/octet-stream")
            .header(header::CONTENT_LENGTH, data.len())
            .body(Body::from(data.clone()))
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        Ok(response)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn blob_exists(Path(digest): Path<String>) -> Json<ExistsResponse> {
    let storage = get_mock_storage();
    let exists = storage.contains_key(&digest);
    
    Json(ExistsResponse {
        exists,
        digest,
    })
}

pub async fn blob_metadata(Path(digest): Path<String>) -> Result<Json<BlobMetadataResponse>, StatusCode> {
    let metadata_store = get_mock_metadata();
    
    if let Some(metadata) = metadata_store.get(&digest) {
        Ok(Json(metadata.clone()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn delete_blob(Path(digest): Path<String>) -> Json<DeleteResponse> {
    let storage = get_mock_storage();
    let metadata_store = get_mock_metadata();
    
    let existed = storage.remove(&digest).is_some();
    metadata_store.remove(&digest);
    
    Json(DeleteResponse {
        success: existed,
        message: if existed {
            "Blob deleted successfully".to_string()
        } else {
            "Blob not found".to_string()
        },
    })
}

pub async fn upload_blob_streaming(
    headers: axum::http::HeaderMap,
    body: axum::body::Bytes,
) -> Result<Json<UploadResponse>, StatusCode> {
    let digest = headers
        .get("x-digest")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::BAD_REQUEST)?
        .to_string();

    let data = body.to_vec();
    
    // Store in mock storage
    let storage = get_mock_storage();
    let metadata_store = get_mock_metadata();
    
    storage.insert(digest.clone(), data.clone());
    metadata_store.insert(digest.clone(), BlobMetadataResponse {
        size: data.len() as u64,
        digest: digest.clone(),
        created_at: chrono::Utc::now().to_rfc3339(),
        content_type: Some("application/octet-stream".to_string()),
    });

    Ok(Json(UploadResponse {
        success: true,
        message: "Blob uploaded via streaming".to_string(),
        digest,
    }))
}

pub async fn download_blob_streaming(Path(digest): Path<String>) -> Result<Response<Body>, StatusCode> {
    // Same as regular download for this mock implementation
    download_blob(Path(digest)).await
}

pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        message: "Storage system is operational".to_string(),
    })
}
