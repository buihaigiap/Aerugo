use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get database URL from environment
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    // Create database connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Run the migration
    println!("Running migration: Adding content column to manifests table");
    
    sqlx::query("ALTER TABLE manifests ADD COLUMN IF NOT EXISTS content TEXT")
        .execute(&pool)
        .await?;
    
    println!("✅ Migration completed successfully");
    
    // Also clear existing manifests so we can test fresh
    println!("Clearing existing manifests for fresh test");
    sqlx::query("DELETE FROM tags").execute(&pool).await?;
    sqlx::query("DELETE FROM manifests").execute(&pool).await?;
    println!("✅ Cleared existing manifests and tags");

    Ok(())
}
