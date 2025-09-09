use crate::handlers::organizations;
use crate::AppState;
use axum::{
    routing::{delete, get, post, put},
    Router,
};

pub fn organization_router() -> Router<AppState> {
    Router::new()
        // Organization management
        .route("/", post(organizations::create_organization))
        .route("/my", get(organizations::list_user_organizations))
        .route("/:org_name", get(organizations::get_organization))
        .route("/:org_name", put(organizations::update_organization))
        .route("/:org_name", delete(organizations::delete_organization))
        // Member management
        .route(
            "/:org_name/members",
            get(organizations::get_organization_members),
        )
        .route(
            "/:org_name/members",
            post(organizations::add_organization_member),
        )
        .route(
            "/:org_name/members/:member_id/role",
            put(organizations::update_member_role),
        )
        .route(
            "/:org_name/members/:member_id",
            delete(organizations::remove_organization_member),
        )
}
