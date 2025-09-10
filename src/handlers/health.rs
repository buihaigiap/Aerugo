use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Health check response
#[derive(Serialize, Deserialize, ToSchema)]
pub struct HealthResponse {
    status: String,
    version: String,
}

/// Health check endpoint
#[utoipa::path(
    get,
    path = "/health",
    tag = "health",
    responses(
        (status = 200, description = "Server is healthy", body = HealthResponse)
    )
)]
pub async fn check() -> axum::Json<HealthResponse> {
    axum::Json(HealthResponse {
        status: "OK".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}
