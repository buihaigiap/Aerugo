use utoipa::OpenApi;
use crate::handlers::{
    auth,
    health,
};
use crate::models::user::UserResponse;

/// Generate the OpenAPI documentation for the entire API
#[derive(OpenApi)]
#[openapi(
    paths(
        // Health endpoints
        health::check,
        
        // Auth endpoints
        auth::register,
        auth::login,
    ),
    components(
        schemas(
            // Health schemas
            health::HealthResponse,
            
            // User schemas
            UserResponse,
            auth::RegisterRequest,
            auth::LoginRequest,
            auth::AuthResponse,
        )
    ),
    tags(
        (name = "health", description = "Health check endpoints"),
        (name = "auth", description = "Authentication endpoints"),
    )
)]
pub struct ApiDoc;
