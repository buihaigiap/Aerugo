use sqlx::{postgres::PgPoolOptions, PgPool};
use anyhow::{Context, Result};
use crate::config::settings::Settings;
use std::time::Duration;

pub async fn create_pool(settings: &Settings) -> Result<PgPool> {
    // Create connection pool with configuration
    let pool = PgPoolOptions::new()
        .max_connections(settings.database.max_connections)
        .min_connections(settings.database.min_connections)
        .acquire_timeout(Duration::from_secs(10))
        .idle_timeout(Duration::from_secs(600))
        .connect(&settings.database.connection_string())
        .await
        .context("Failed to create database connection pool")?;
    
    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .context("Failed to run database migrations")?;

    // Test database connection
    pool.acquire()
        .await
        .context("Failed to acquire a test database connection")?;

    Ok(pool)
}

// Transaction helper function
pub async fn transaction<'a, F, R>(pool: &PgPool, f: F) -> Result<R> 
where
    F: for<'b> FnOnce(&'b mut sqlx::Transaction<'_, sqlx::Postgres>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<R>> + 'b>>,
{
    let mut tx = pool.begin().await?;
    
    match f(&mut tx).await {
        Ok(result) => {
            tx.commit().await?;
            Ok(result)
        }
        Err(e) => {
            tx.rollback().await?;
            Err(e)
        }
    }
}

// Helper function to check if a record exists
pub async fn exists<'a>(
    pool: &PgPool,
    table: &str,
    column: &str,
    value: &str,
) -> Result<bool> {
    let query = format!(
        "SELECT EXISTS(SELECT 1 FROM {} WHERE {} = $1)",
        table, column
    );
    
    let exists = sqlx::query_scalar::<_, bool>(&query)
        .bind(value)
        .fetch_one(pool)
        .await
        .context("Failed to check record existence")?;
    
    Ok(exists)
}

// Error type for database errors
#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("Record not found")]
    NotFound,
    #[error("Record already exists")]
    AlreadyExists,
    #[error("Database error: {0}")]
    Other(#[from] sqlx::Error),
}
