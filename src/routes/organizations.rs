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
        .route("/:id", get(organizations::get_organization))
        .route("/:id", put(organizations::update_organization))
        .route("/:id", delete(organizations::delete_organization))
        // Member management
        .route(
            "/:id/members",
            get(organizations::get_organization_members),
        )
        .route(
            "/:id/members",
            post(organizations::add_organization_member),
        )
        .route(
            "/:id/members/:member_id/role",
            put(organizations::update_member_role),
        )
        .route(
            "/:id/members/:member_id",
            delete(organizations::remove_organization_member),
        )
}
