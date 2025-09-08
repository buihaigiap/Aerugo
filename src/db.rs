use sqlx::PgPool;
use anyhow::Result;
use crate::config::settings::Settings;

pub async fn create_pool(settings: &Settings) -> Result<PgPool> {
    let pool = PgPool::connect(&settings.database.connection_string()).await?;
    
    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    Ok(pool)
}
