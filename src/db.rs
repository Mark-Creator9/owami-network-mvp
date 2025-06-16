use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

pub type DatabasePool = Pool<Postgres>;

pub async fn create_pool(database_url: &str) -> Result<DatabasePool, Box<dyn std::error::Error>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;
    Ok(pool)
}
