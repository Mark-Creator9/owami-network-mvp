use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct BlockModel {
    pub id: i64,
    pub height: u64,
    pub previous_hash: String,
    pub hash: String,
    pub timestamp: i64,
    pub validator: String,
    pub transactions: Vec<u8>, // Serialized transactions
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TransactionModel {
    pub id: i64,
    pub hash: String,
    pub from_address: String,
    pub to_address: String,
    pub amount: u64,
    pub signature: String,
    pub block_height: Option<u64>,
    pub created_at: i64,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct BalanceModel {
    pub address: String,
    pub balance: u64,
    pub updated_at: i64,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PendingTransactionModel {
    pub id: i64,
    pub transaction_data: Vec<u8>, // Serialized Transaction
    pub created_at: i64,
}