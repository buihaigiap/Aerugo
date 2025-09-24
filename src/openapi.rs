use utoipa::OpenApi;
use utoipa::Modify;
use utoipa::openapi::security::{SecurityScheme, Http, HttpAuthScheme};

use crate::handlers::{
    auth,
    docker_registry_v2,
    organizations,
    repositories,
};
use crate::models::{
    user::UserResponse,
    organizations::{
        Organization, CreateOrganizationRequest, UpdateOrganizationRequest,
        AddMemberRequest, UpdateMemberRequest, OrganizationMember,
    },
    repository::{Repository as RepositoryModel, CreateRepositoryRequest, RepositoryDetailsResponse},
};
use crate::handlers::docker_registry_v2::{ApiVersionResponse, CatalogResponse, TagListResponse, BlobUploadResponse, ErrorResponse, RegistryError};

/// Security addon to add Bearer Auth to OpenAPI
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
        // Auth endpoints
        auth::register,
        auth::login,
        auth::me, 
        auth::refresh,
        auth::change_password,
        auth::forgot_password,
        auth::verify_otp_and_reset,
        auth::get_user_api_keys,
        auth::create_api_key,
        auth::delete_api_key,     

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
        repositories::list_repositories_by_namespace,
        repositories::list_public_repositories,
        repositories::get_repository,
        repositories::delete_repository,

        // Docker Registry V2 API endpoints
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
            // User schemas
            UserResponse,
            auth::RegisterRequest,
            auth::LoginRequest,
            auth::RefreshRequest,
            auth::AuthResponse,
            auth::ChangePasswordRequest,
            auth::ForgotPasswordRequest,
            auth::VerifyOtpRequest,
            auth::ApiKeyResponse,     
            auth::CreateApiKeyRequest,
            auth::CreateApiKeyResponse,
            auth::DeleteApiKeyResponse,
            auth::ApiKeyErrorResponse, 

            // Organization schemas
            Organization,
            CreateOrganizationRequest,
            UpdateOrganizationRequest,
            AddMemberRequest,
            UpdateMemberRequest,
            OrganizationMember,

            // Repository schemas
            RepositoryModel,
            CreateRepositoryRequest,
            RepositoryDetailsResponse,
            
            // Additional repository schemas
            repositories::CreateRepositoryRequest,
            repositories::RepositoryResponse,
            repositories::OrganizationInfo,
            repositories::RepositoryDetailsResponse,
            repositories::RepositoryStats,
            repositories::ListRepositoriesQuery,
            
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
        (name = "auth", description = "Authentication endpoints"),
        (name = "organizations", description = "Organization management endpoints"),
        (name = "repositories", description = "Repository management endpoints"),
        (name = "docker-registry-v2", description = "Docker Registry V2 API - OCI Distribution Specification"),
    ),
      modifiers(&SecurityAddon)  // 👈 add this to get Bearer Auth
)]
pub struct ApiDoc;
