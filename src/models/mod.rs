pub mod user;

use serde::{Deserialize, Serialize};
use chrono::DateTime;
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenBalance {
    pub address: String,
    pub balance: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenTransaction {
    pub transaction_hash: String,
    pub from_address: String,
    pub to_address: String,
    pub amount: String,
    pub block_number: i32,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenApproval {
    pub owner: String,
    pub spender: String,
    pub amount: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DApp {
    pub id: String,
    pub name: String,
    pub description: String,
    pub contract_address: String,
    pub creator_id: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DAppState {
    pub dapp_id: String,
    pub key: String,
    pub value: String,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDAppRequest {
    pub name: String,
    pub description: String,
    pub contract_address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        ApiResponse {
            success: true,
            data: Some(data),
            message: None,
        }
    }

    pub fn error(message: String) -> Self {
        ApiResponse {
            success: false,
            data: None,
            message: Some(message),
        }
    }
}