use std::sync::Arc;
use sqlx::{Pool, Sqlite};
use sqlx::sqlite::SqlitePoolOptions;

#[derive(Clone)]
pub enum DatabasePool {
    Sqlite(Arc<Pool<Sqlite>>),
}

pub async fn create_pool() -> Result<DatabasePool, Box<dyn std::error::Error>> {
    // Always create a Sqlite pool for local development and testing
    let pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await?;
    Ok(DatabasePool::Sqlite(Arc::new(pool)))
}
