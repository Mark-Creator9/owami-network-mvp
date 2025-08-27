use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, Row};
use std::process::Command;
use std::path::Path as StdPath;
use uuid::Uuid;

use crate::models::{DApp, CreateDAppRequest, ApiResponse};

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
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateDAppRequest>,
) -> Result<Json<ApiResponse<DAppResponse>>, StatusCode> {
    let dapp_id = Uuid::new_v4().to_string();
    let creator_id = Uuid::new_v4().to_string(); // In real app, this would come from auth
    
    let result = sqlx::query(
        r#"
        INSERT INTO dapps (id, name, description, contract_address, creator_id, created_at)
        VALUES (?, ?, ?, ?, ?, datetime('now'))
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
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<DAppResponse>>, StatusCode> {
    let result = sqlx::query(
        "SELECT id, name, description, contract_address, creator_id, created_at FROM dapps WHERE id = ?"
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
    State(pool): State<SqlitePool>,
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

// Legacy deploy/call functionality for CLI integration
#[derive(Deserialize)]
pub struct DeployRequest {
    pub contract_path: String,
    pub network: Option<String>, // e.g., "testnet"
}

#[derive(Serialize)]
pub struct DeployResponse {
    pub contract_address: String,
}

pub async fn deploy_contract(Json(req): Json<DeployRequest>) -> Result<Json<DeployResponse>, StatusCode> {
    let network = req.network.clone().unwrap_or_else(|| "testnet".to_string());
    
    // Check if file exists
    if !StdPath::new(&req.contract_path).exists() {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Execute the owami-cli deploy command
    let output = Command::new("./owami-cli/target/debug/owami-cli")
        .arg("deploy")
        .arg("--file")
        .arg(&req.contract_path)
        .arg("--network")
        .arg(&network)
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                // Parse the output to extract the contract address
                // For now, we'll simulate a contract address
                let contract_address = "0x1234567890abcdef1234567890abcdef12345678";
                Ok(Json(DeployResponse {
                    contract_address: contract_address.to_string(),
                }))
            } else {
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(Deserialize)]
pub struct CallRequest {
    pub contract_address: String,
    pub function_name: String,
    pub params: Option<serde_json::Value>,
    pub network: Option<String>,
}

pub async fn call_contract(Json(req): Json<CallRequest>) -> Result<String, StatusCode> {
    let network = req.network.clone().unwrap_or_else(|| "testnet".to_string());
    
    // Convert params to JSON string if present
    let params_str = if let Some(ref params) = req.params {
        match serde_json::to_string(params) {
            Ok(s) => Some(s),
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    } else {
        None
    };

    // Build command arguments
    let mut args = vec![
        "call",
        "--address",
        &req.contract_address,
        "--function",
        &req.function_name,
        "--network",
        &network,
    ];

    if let Some(params) = &params_str {
        args.push("--params");
        args.push(params);
    }

    // Execute the owami-cli call command
    let output = Command::new("./owami-cli/target/debug/owami-cli")
        .args(&args)
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                let result = String::from_utf8_lossy(&output.stdout);
                Ok(result.to_string())
            } else {
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[cfg(test)]
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
        
        let result = create_dapp(State(pool), Json(request)).await;
        assert!(result.is_ok());
    }
}
