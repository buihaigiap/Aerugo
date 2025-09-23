use axum::{
    routing::{post, get, put, delete},
    Router,
};
use crate::handlers::auth;
use crate::AppState;

pub fn auth_router() -> Router<AppState> {
    Router::new()
        .route("/register", post(auth::register))
        .route("/login", post(auth::login))
        .route("/logout", post(auth::logout))
        .route("/me", get(auth::me))
        .route("/api-keys", get(auth::get_user_api_keys))
        .route("/api-keys", post(auth::create_api_key))
        .route("/api-keys/:id", delete(auth::delete_api_key))
        .route("/refresh", post(auth::refresh))
        .route("/change-password", put(auth::change_password))
        .route("/forgot-password", post(auth::forgot_password))
        .route("/verify-otp", post(auth::verify_otp_and_reset))
}
