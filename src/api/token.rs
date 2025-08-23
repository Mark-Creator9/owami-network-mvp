use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::blockchain::Blockchain;
use crate::transaction::Transaction;
use crate::crypto_utils;

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferRequest {
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub private_key: String, // In production, use proper key management
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BalanceResponse {
    pub address: String,
    pub balance: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionResponse {
    pub hash: String,
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenInfo {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: u64,
    pub contract_address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MintRequest {
    pub to: String,
    pub amount: u64,
}

pub async fn get_token_info() -> Result<Json<TokenInfo>, StatusCode> {
    Ok(Json(TokenInfo {
        name: "OWami Token".to_string(),
        symbol: "OWA".to_string(),
        decimals: 18,
        total_supply: 1000000000, // 1 billion tokens
        contract_address: "0x742d35Cc6634C0532925a3b8D4e6D3b6e8d3e8A0".to_string(),
    }))
}

pub async fn get_balance(
    State(blockchain): State<Arc<Mutex<Blockchain>>>,
    Path(address): Path<String>,
) -> Result<Json<BalanceResponse>, StatusCode> {
    let blockchain = blockchain.lock().await;
    let balance = blockchain.get_balance(&address);
    
    Ok(Json(BalanceResponse {
        address,
        balance,
    }))
}

pub async fn transfer(
    State(blockchain): State<Arc<Mutex<Blockchain>>>,
    Json(payload): Json<TransferRequest>,
) -> Result<Json<TransactionResponse>, StatusCode> {
    // In production, validate private key properly
    let signing_key = crypto_utils::default_signing_key();
    
    let transaction = Transaction::new(
        payload.from.clone(),
        payload.to.clone(),
        payload.amount,
        None,
        &signing_key,
    );
    
    let mut blockchain = blockchain.lock().await;
    
    // Add transaction to pending pool
    blockchain.add_transaction(transaction.clone())
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    // Create response
    let response = TransactionResponse {
        hash: transaction.hash(),
        from: transaction.from,
        to: transaction.to,
        amount: transaction.amount,
        timestamp: transaction.timestamp,
    };
    
    Ok(Json(response))
}

pub async fn mint(
    State(blockchain): State<Arc<Mutex<Blockchain>>>,
    Path(address): Path<String>,
    Json(amount): Json<u64>,
) -> Result<Json<BalanceResponse>, StatusCode> {
    let mut blockchain = blockchain.lock().await;
    blockchain.mint(address.clone(), amount);
    let balance = blockchain.get_balance(&address);
    
    Ok(Json(BalanceResponse {
        address,
        balance,
    }))
}

// Alternative mint endpoint that matches test expectations
pub async fn mint_tokens(
    State(blockchain): State<Arc<Mutex<Blockchain>>>,
    Json(payload): Json<MintRequest>,
) -> Result<Json<BalanceResponse>, StatusCode> {
    let mut blockchain = blockchain.lock().await;
    blockchain.mint(payload.to.clone(), payload.amount);
    let balance = blockchain.get_balance(&payload.to);
    
    Ok(Json(BalanceResponse {
        address: payload.to,
        balance,
    }))
}

pub async fn get_transactions(
    State(blockchain): State<Arc<Mutex<Blockchain>>>,
) -> Result<Json<Vec<TransactionResponse>>, StatusCode> {
    let blockchain = blockchain.lock().await;
    
    // Get transactions from all blocks
    let mut transactions = Vec::new();
    
    for block in &blockchain.blocks {
        for tx in &block.transactions {
            transactions.push(TransactionResponse {
                hash: tx.hash(),
                from: tx.from.clone(),
                to: tx.to.clone(),
                amount: tx.amount,
                timestamp: tx.timestamp,
            });
        }
    }
    
    // Sort by timestamp (newest first)
    transactions.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    
    Ok(Json(transactions))
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn setup_blockchain() -> Arc<Mutex<Blockchain>> {
        let validator_key = crypto_utils::default_signing_key();
        let blockchain = Blockchain::new(&validator_key);
        Arc::new(Mutex::new(blockchain))
    }

    #[tokio::test]
    async fn test_get_token_info() {
        let result = get_token_info().await;
        assert!(result.is_ok());
        let info = result.unwrap();
        assert_eq!(info.symbol, "OWA");
    }

    #[tokio::test]
    async fn test_get_balance() {
        let blockchain = setup_blockchain().await;
        
        // Mint some tokens
        let mut bc = blockchain.lock().await;
        bc.mint("test_address".to_string(), 1000);
        drop(bc);
        
        let result = get_balance(State(blockchain), Path("test_address".to_string())).await;
        assert!(result.is_ok());
        let balance = result.unwrap();
        assert_eq!(balance.balance, 1000);
    }

    #[tokio::test]
    async fn test_mint() {
        let blockchain = setup_blockchain().await;
        
        let result = mint(
            State(blockchain.clone()),
            Path("test_address".to_string()),
            Json(500),
        ).await;
        
        assert!(result.is_ok());
        let balance = result.unwrap();
        assert_eq!(balance.balance, 500);
    }
}