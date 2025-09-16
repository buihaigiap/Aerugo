use axum::{
    routing::get,
    Router,
    Json,
    response::IntoResponse,
    http::StatusCode,
};
use serde_json::json;

use crate::AppState;

pub fn health_router() -> Router<AppState> {
    Router::new()
        .route("/health", get(check_health))
}

async fn check_health() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({
        "status": "healthy"
    })))
}
