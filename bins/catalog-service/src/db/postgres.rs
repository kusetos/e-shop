use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

pub async fn create_pool() -> PgPool {
    let database_url = std::env::var("CATALOG_DATABASE_URL")
        .expect("CATALOG_DATABASE_URL must be set in environment");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to PostgreSQL")
}
