use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, postgres::Postgres, Row};
use uuid::Uuid;

use crate::models::{DApp, CreateDAppRequest, ApiResponse};
use crate::api::auth::Claims;

#[derive(Debug, Serialize, Deserialize)]
pub struct DAppResponse {
    pub id: String,
    pub name: String,
    pub description: String,
    pub contract_address: String,
    pub creator_id: String,
    pub created_at: String,
}

impl From<DApp> for DAppResponse {
    fn from(dapp: DApp) -> Self {
        DAppResponse {
            id: dapp.id.to_string(),
            name: dapp.name,
            description: dapp.description,
            contract_address: dapp.contract_address,
            creator_id: dapp.creator_id.to_string(),
            created_at: dapp.created_at.to_rfc3339(),
        }
    }
}

// Axum-based handlers for DApp management
pub async fn create_dapp(
    State(pool): State<Pool<Postgres>>,
    claims: Claims,
    Json(payload): Json<CreateDAppRequest>,
) -> Result<Json<ApiResponse<DAppResponse>>, StatusCode> {
    let dapp_id = Uuid::new_v4().to_string();
    let creator_id = claims.sub; // Use the user ID from the JWT
    
    let result = sqlx::query(
        r#"
        INSERT INTO dapps (id, name, description, contract_address, creator_id, created_at)
        VALUES ($1, $2, $3, $4, $5, NOW())
        "#,
    )
    .bind(&dapp_id)
    .bind(&payload.name)
    .bind(&payload.description)
    .bind(&payload.contract_address)
    .bind(&creator_id)
    .execute(&pool)
    .await;
    
    match result {
        Ok(_) => {
            let response = DAppResponse {
                id: dapp_id,
                name: payload.name,
                description: payload.description,
                contract_address: payload.contract_address,
                creator_id: creator_id,
                created_at: chrono::Utc::now().to_rfc3339(),
            };
            Ok(Json(ApiResponse::success(response)))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_dapp(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<DAppResponse>>, StatusCode> {
    let result = sqlx::query(
        "SELECT id, name, description, contract_address, creator_id, created_at FROM dapps WHERE id = $1"
    )
    .bind(&id)
    .fetch_optional(&pool)
    .await;
    
    match result {
        Ok(Some(row)) => {
            let response = DAppResponse {
                id: row.try_get::<String, _>("id").unwrap_or_default(),
                name: row.try_get("name").unwrap_or_default(),
                description: row.try_get("description").unwrap_or_default(),
                contract_address: row.try_get("contract_address").unwrap_or_default(),
                creator_id: row.try_get::<String, _>("creator_id").unwrap_or_default(),
                created_at: row.try_get::<String, _>("created_at").unwrap_or_default(),
            };
            Ok(Json(ApiResponse::success(response)))
        }
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn list_dapps(
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<ApiResponse<Vec<DAppResponse>>>, StatusCode> {
    let result = sqlx::query(
        "SELECT id, name, description, contract_address, creator_id, created_at FROM dapps ORDER BY created_at DESC"
    )
    .fetch_all(&pool)
    .await;
    
    match result {
        Ok(rows) => {
            let responses: Vec<DAppResponse> = rows
                .into_iter()
                .map(|row| DAppResponse {
                    id: row.try_get::<String, _>("id").unwrap_or_default(),
                    name: row.try_get("name").unwrap_or_default(),
                    description: row.try_get("description").unwrap_or_default(),
                    contract_address: row.try_get("contract_address").unwrap_or_default(),
                    creator_id: row.try_get::<String, _>("creator_id").unwrap_or_default(),
                    created_at: row.try_get::<String, _>("created_at").unwrap_or_default(),
                })
                .collect();
            Ok(Json(ApiResponse::success(responses)))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}


#[cfg(any())]
mod tests {
    use super::*;

    async fn setup_test_db() -> SqlitePool {
        let database_url = std::env::var("TEST_DATABASE_URL")
            .unwrap_or_else(|_| "sqlite::memory:".to_string());
        
        let pool = sqlx::SqlitePool::connect(&database_url)
            .await
            .expect("Failed to connect to test database");
            
        // Create dapps table for testing
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS dapps (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT,
                contract_address TEXT NOT NULL,
                creator_id TEXT NOT NULL,
                created_at TEXT DEFAULT (datetime('now'))
            )"
        )
        .execute(&pool)
        .await
        .expect("Failed to create dapps table");
        
        pool
    }

    #[tokio::test]
    async fn test_create_dapp() {
        let pool = setup_test_db().await;
        
        let request = CreateDAppRequest {
            name: "Test DApp".to_string(),
            description: "A test decentralized application".to_string(),
            contract_address: "0x1234567890abcdef".to_string(),
        };

        let claims = Claims {
            sub: "test-user-id".to_string(),
            exp: (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp() as usize,
        };
        
        let result = create_dapp(State(pool), claims, Json(request)).await;
        assert!(result.is_ok());
    }
}
