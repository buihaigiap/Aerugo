use axum::{
    routing::{post, get},
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
        .route("/refresh", post(auth::refresh))
}
