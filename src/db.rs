use sqlx::Pool;
use sqlx::sqlite::SqlitePoolOptions;

pub type DatabasePool = Pool<sqlx::Sqlite>;

pub async fn create_pool(database_url: &str) -> Result<DatabasePool, Box<dyn std::error::Error>> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;
    Ok(pool)
}
