use axum::{
    routing::get,
    Router,
    Json,
    response::IntoResponse,
    http::StatusCode,
    extract::State,
};
use serde_json::json;

use crate::AppState;

pub fn health_router() -> Router<AppState> {
    Router::new()
        .route("/health", get(check_health))
        .route("/health/cache", get(cache_stats))
}

async fn check_health() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({
        "status": "healthy"
    })))
}

async fn cache_stats(State(state): State<AppState>) -> impl IntoResponse {
    if let Some(cache) = &state.cache {
        let stats = cache.get_stats().await;
        (StatusCode::OK, Json(json!({
            "status": "ok",
            "cache_stats": stats
        })))
    } else {
        (StatusCode::OK, Json(json!({
            "status": "disabled",
            "message": "Cache is not enabled"
        })))
    }
}
