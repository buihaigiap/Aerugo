use axum::{
    extract::{Path, Query, State},
    response::Json,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    database::models::BlobUpload,
    AppState,
};

#[derive(Debug, Deserialize)]
pub struct UserUploadsQuery {
    pub limit: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct UserUploadsResponse {
    pub user_id: String,
    pub uploads: Vec<BlobUpload>,
    pub total_count: usize,
}

// Get uploads by user ID
pub async fn get_user_uploads(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
    Query(params): Query<UserUploadsQuery>,
) -> Result<Json<UserUploadsResponse>, StatusCode> {
    
    match crate::database::queries::get_user_uploads(
        &state.db_pool, 
        &user_id,
        params.limit,
    ).await {
        Ok(uploads) => {
            let total_count = uploads.len();
            Ok(Json(UserUploadsResponse {
                user_id,
                uploads,
                total_count,
            }))
        }
        Err(e) => {
            eprintln!("Failed to get user uploads: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Get uploads by repository
pub async fn get_repository_uploads(
    State(state): State<AppState>,
    Path(repo_name): Path<String>,
    Query(params): Query<UserUploadsQuery>,
) -> Result<Json<Vec<BlobUpload>>, StatusCode> {
    
    match crate::database::queries::get_repository_uploads(
        &state.db_pool,
        &repo_name,
        params.limit,
    ).await {
        Ok(uploads) => Ok(Json(uploads)),
        Err(e) => {
            eprintln!("Failed to get repository uploads: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
