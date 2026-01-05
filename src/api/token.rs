use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::{blockchain::Blockchain, transaction::Transaction, crypto_utils, audit_log};

#[derive(Serialize)]
pub struct TokenInfo {
    name: String,
    symbol: String,
    decimals: u8,
    total_supply: u64,
}

#[derive(Serialize)]
pub struct BalanceResponse {
    address: String,
    balance: u64,
}

#[derive(Deserialize)]
pub struct TransferRequest {
    from: String,
    to: String,
    amount: u64,
    private_key: String,
}

#[derive(Deserialize)]
pub struct MintRequest {
    to: String,
    amount: u64,
}

#[derive(Serialize)]
pub struct TransactionResponse {
    hash: String,
    from: String,
    to: String,
    amount: u64,
    timestamp: i64, // Changed from u64 to i64
}

pub async fn get_token_info() -> Json<TokenInfo> {
    Json(TokenInfo {
        name: "Owami Token".to_string(),
        symbol: "OWA".to_string(),
        decimals: 18,
        total_supply: 1000000,
    })
}

pub async fn get_balance(
    State(blockchain): State<Arc<Mutex<Blockchain>>>,
    Path(address): Path<String>,
) -> Result<Json<BalanceResponse>, StatusCode> {
    let blockchain = blockchain.lock().await;
    let balance = blockchain.get_balance(&address).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    audit_log::log_security_event(
        "Balance queried".to_string(),
        format!("Balance query for address: {}", address),
        "info".to_string(),
        None,
    ).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(BalanceResponse {
        address,
        balance,
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
    if let Err(e) = blockchain.add_transaction(transaction.clone()) {
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
        timestamp: transaction.timestamp as i64, // Cast to i64
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
    blockchain.mint(address.clone(), amount).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let balance = blockchain.get_balance(&address).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
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
    blockchain.mint(payload.to.clone(), payload.amount).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let balance = blockchain.get_balance(&payload.to).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
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
    
    // Get all blocks directly from the blockchain
    for block in &blockchain.blocks {
        for tx in &block.transactions {
            transactions.push(TransactionResponse {
                hash: tx.hash(),
                from: tx.from.clone(),
                to: tx.to.clone(),
                amount: tx.amount,
                timestamp: tx.timestamp as i64, // Cast to i64
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