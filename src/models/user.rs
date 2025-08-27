use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl<'r> FromRow<'r, sqlx::sqlite::SqliteRow> for User {
    fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(User {
            id: row.try_get("id")?,
            username: row.try_get("username")?,
            password_hash: row.try_get("password_hash")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}

impl User {
    pub async fn find_by_username(
        username: &str,
        pool: &sqlx::SqlitePool
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE username = ?"
        )
        .bind(username)
        .fetch_one(pool)
        .await
    }
}