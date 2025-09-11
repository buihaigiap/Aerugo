use utoipa::OpenApi;
use crate::handlers::{
    auth,
    health,
    organizations,
    registry,
};
use crate::models::{
    user::UserResponse,
    organizations::{Organization, CreateOrganizationRequest, UpdateOrganizationRequest, AddMemberRequest, UpdateMemberRequest, OrganizationMember},
};
use crate::handlers::registry::{Repository, ImageInfo};

/// Generate the OpenAPI documentation for the entire API
#[derive(OpenApi)]
#[openapi(
    paths(
        // Health endpoints
        health::check,
        
        // Auth endpoints
        auth::register,
        auth::login,
        auth::refresh,
        
        // Organization endpoints
        organizations::create_organization,
        organizations::get_organization,
        organizations::list_user_organizations,
        organizations::update_organization,
        organizations::delete_organization,
        organizations::get_organization_members,
        organizations::add_organization_member,
        organizations::update_member_role,
        organizations::remove_organization_member,
        
        // Registry endpoints
        registry::list_repositories,
        registry::get_repository,
        registry::list_images,
    ),
    components(
        schemas(
            // Health schemas
            health::HealthResponse,
            
            // User schemas
            UserResponse,
            auth::RegisterRequest,
            auth::LoginRequest,
            auth::RefreshRequest,
            auth::AuthResponse,
            
            // Organization schemas
            Organization,
            CreateOrganizationRequest,
            UpdateOrganizationRequest,
            AddMemberRequest,
            UpdateMemberRequest,
            OrganizationMember,
            
            // Registry schemas
            Repository,
            ImageInfo,
        )
    ),
    tags(
        (name = "health", description = "Health check endpoints"),
        (name = "auth", description = "Authentication endpoints"),
        (name = "organizations", description = "Organization management endpoints"),
        (name = "registry", description = "Container registry operations"),
    )
)]
pub struct ApiDoc;
