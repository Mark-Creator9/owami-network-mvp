use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub async fn find_by_username(
        username: &str, 
        pool: &sqlx::PgPool
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE username = $1"
        )
        .bind(username)
        .fetch_one(pool)
        .await
    }
}