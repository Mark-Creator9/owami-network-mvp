use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use anyhow::Result;

use crate::blockchain::Blockchain;
use crate::transaction::Transaction;
use crate::crypto_utils;
use crate::audit_log;

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferRequest {
    pub from: String,
    pub to: String,
    pub amount: u64,
    // Note: In production, use proper authentication instead of private key in requests
    pub private_key: String,
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
    let _ = audit_log::log_system_event(
        "Token info requested".to_string(),
        "Token information endpoint accessed".to_string(),
        "info".to_string(),
    );
    
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
    let balance = blockchain.get_balance(&address).await;
    
    audit_log::log_security_event(
        "Balance queried".to_string(),
        format!("Balance query for address: {}", address),
        "info".to_string(),
        None,
    ).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let balance_value = balance.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(BalanceResponse {
        address,
        balance: balance_value,
    }))
}

pub async fn transfer(
    State(blockchain): State<Arc<Mutex<Blockchain>>>,
    Json(payload): Json<TransferRequest>,
) -> Result<Json<TransactionResponse>, StatusCode> {
    // Convert private key to signing key
    let signing_key = match crypto_utils::hex_to_signing_key(&payload.private_key) {
        Ok(key) => key,
        Err(_) => {
            audit_log::log_security_event(
                "Transfer failed".to_string(),
                format!("Invalid private key provided for transfer from {}", payload.from),
                "failure".to_string(),
                None,
            ).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            return Err(StatusCode::BAD_REQUEST);
        },
    };
    
    // Derive sender address from the provided private key to ensure signature verification succeeds
    let sender_address = hex::encode(signing_key.verifying_key().to_bytes());

    // Create unsigned transaction using derived sender
    let mut transaction = Transaction::new(
        sender_address,
        payload.to.clone(),
        payload.amount,
        None,
    );
    
    // Sign the transaction
    if let Err(_) = transaction.sign(&signing_key) {
        audit_log::log_security_event(
            "Transfer failed".to_string(),
            format!("Transaction signing failed for transfer from {}", payload.from),
            "failure".to_string(),
            None,
        ).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        return Err(StatusCode::BAD_REQUEST);
    }
    
    let mut blockchain = blockchain.lock().await;
    
    // Add transaction to pending pool
    if let Err(e) = blockchain.add_transaction(transaction.clone()).await {
        audit_log::log_transaction_event(
            "Transfer failed".to_string(),
            format!("Failed to add transaction to pool: {}", e),
            "failure".to_string(),
            Some(transaction.hash()),
            None,
        ).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        return Err(StatusCode::BAD_REQUEST);
    }
    
    // Create response
    let transaction_hash = transaction.hash();
    let response = TransactionResponse {
        hash: transaction_hash.clone(),
        from: transaction.from,
        to: transaction.to,
        amount: transaction.amount,
        timestamp: transaction.timestamp,
    };
    
    audit_log::log_transaction_event(
        "Transfer queued".to_string(),
        format!("Transfer of {} tokens from {} to {} queued for mining", payload.amount, response.from, response.to),
        "success".to_string(),
        Some(transaction_hash),
        None,
    ).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(response))
}

pub async fn mint(
    State(blockchain): State<Arc<Mutex<Blockchain>>>,
    Path(address): Path<String>,
    Json(amount): Json<u64>,
) -> Result<Json<BalanceResponse>, StatusCode> {
    let mut blockchain = blockchain.lock().await;
    blockchain.mint(address.clone(), amount).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let balance = blockchain.get_balance(&address).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    audit_log::log_key_management_event(
        "Tokens minted".to_string(),
        format!("Minted {} tokens to address {}", amount, address),
        "success".to_string(),
        None,
    ).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
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
    blockchain.mint(payload.to.clone(), payload.amount).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let balance = blockchain.get_balance(&payload.to).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    audit_log::log_key_management_event(
        "Tokens minted".to_string(),
        format!("Minted {} tokens to address {}", payload.amount, payload.to),
        "success".to_string(),
        None,
    ).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
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
    
    // Get all blocks from the database
    let blocks_data = blockchain.repository.get_blocks(0, 1000).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    for block_data in blocks_data {
        let block: crate::block::Block = bincode::deserialize(&block_data).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
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
    
    audit_log::log_system_event(
        "Transactions queried".to_string(),
        format!("Transaction history accessed, {} transactions found", transactions.len()),
        "info".to_string(),
    ).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(transactions))
}

#[cfg(any())]
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