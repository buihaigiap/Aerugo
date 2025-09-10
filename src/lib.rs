use axum::extract::FromRef;
use sqlx::PgPool;

pub mod auth;
pub mod config;
pub mod database;
pub mod db;
pub mod handlers;
pub mod models;
pub mod openapi;
pub mod routes;
pub mod storage;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub config: crate::config::settings::Settings,
}
