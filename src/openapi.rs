use utoipa::OpenApi;
use utoipa::Modify;
use utoipa::openapi::security::{SecurityScheme, Http, HttpAuthScheme};

use crate::handlers::{
    auth,
    docker_registry_v2,
    health,
    organizations,
    registry,
    repositories,
};
use crate::models::{
    user::UserResponse,
    organizations::{
        Organization, CreateOrganizationRequest, UpdateOrganizationRequest,
        AddMemberRequest, UpdateMemberRequest, OrganizationMember,
    },
    repository::{Repository as RepositoryModel, RepositoryPermission, CreateRepositoryRequest, SetRepositoryPermissionsRequest, RepositoryDetailsResponse},
};
use crate::handlers::registry::{Repository, ImageInfo};
use crate::handlers::docker_registry_v2::{ApiVersionResponse, CatalogResponse, TagListResponse, BlobUploadResponse, ErrorResponse, RegistryError};

/// Security addon để thêm Bearer Auth vào OpenAPI
pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        use utoipa::openapi::security::{SecurityScheme, Http, HttpAuthScheme};

        // Add Bearer auth scheme
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearerAuth",
                SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer))
            );
        }

        // Set security requirement globally
        let requirement = utoipa::openapi::SecurityRequirement::new(
            "bearerAuth",
            std::iter::empty::<String>()
        );
        openapi.security = Some(vec![requirement]);
    }
}

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

        // Repository endpoints
        repositories::create_repository,
        repositories::list_repositories,
        repositories::get_repository,
        repositories::delete_repository,
        repositories::set_repository_permissions,

        // Registry endpoints
        registry::list_repositories,
        registry::get_repository,
        registry::list_images,
        
        // Docker Registry V2 API endpoints
        docker_registry_v2::base_api,
        docker_registry_v2::get_catalog,
        docker_registry_v2::get_manifest,
        docker_registry_v2::head_manifest,
        docker_registry_v2::put_manifest,
        docker_registry_v2::delete_manifest,
        docker_registry_v2::get_blob,
        docker_registry_v2::head_blob,
        docker_registry_v2::start_blob_upload,
        docker_registry_v2::upload_blob_chunk,
        docker_registry_v2::complete_blob_upload,
        docker_registry_v2::get_upload_status,
        docker_registry_v2::cancel_blob_upload,
        docker_registry_v2::list_tags,
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

            // Repository schemas
            RepositoryModel,
            RepositoryPermission,
            CreateRepositoryRequest,
            SetRepositoryPermissionsRequest,
            RepositoryDetailsResponse,

            // Registry schemas
            Repository,
            ImageInfo,
            
            // Docker Registry V2 API schemas
            ApiVersionResponse,
            CatalogResponse,
            TagListResponse,
            BlobUploadResponse,
            ErrorResponse,
            RegistryError,
        )
    ),
    tags(
        (name = "health", description = "Health check endpoints"),
        (name = "auth", description = "Authentication endpoints"),
        (name = "organizations", description = "Organization management endpoints"),
        (name = "repositories", description = "Repository management endpoints"),
        (name = "registry", description = "Container registry operations"),
        (name = "docker-registry-v2", description = "Docker Registry V2 API - OCI Distribution Specification"),
    ),
      modifiers(&SecurityAddon)  // 👈 add this to get Bearer Auth
)]
pub struct ApiDoc;
