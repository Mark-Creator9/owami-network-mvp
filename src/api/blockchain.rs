use crate::{audit_log, blockchain::Blockchain, crypto_utils};
use axum::{extract::State, http::StatusCode, response::Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Serialize)]
pub struct BlockchainInfo {
    height: u64,
    block_count: u64,
    pending_transactions: usize,
    latest_block_hash: String,
    network: String,
}

#[derive(Serialize)]
pub struct BlockInfo {
    height: u64,
    hash: String,
    previous_hash: String,
    timestamp: i64, // Changed from u64 to i64
    transaction_count: usize,
}

#[derive(Serialize)]
pub struct TransactionInfo {
    hash: String,
    from: String,
    to: String,
    amount: u64,
    timestamp: i64, // Changed from u64 to i64
}

#[derive(Deserialize)]
pub struct MineBlockRequest {
    private_key: String,
}

pub async fn get_info(
    State(blockchain): State<Arc<Mutex<Blockchain>>>,
) -> Result<Json<BlockchainInfo>, StatusCode> {
    let blockchain = blockchain.lock().await;
    let height = blockchain.get_block_height();
    let block_count = blockchain.blocks.len() as u64;
    let pending_transactions = blockchain.pending_transactions.len();
    let latest_block_hash = blockchain.get_latest_block().hash();

    Ok(Json(BlockchainInfo {
        height,
        block_count,
        pending_transactions,
        latest_block_hash,
        network: "owami-testnet".to_string(),
    }))
}

pub async fn get_blocks(
    State(blockchain): State<Arc<Mutex<Blockchain>>>,
) -> Result<Json<Vec<BlockInfo>>, StatusCode> {
    let blockchain = blockchain.lock().await;
    let mut blocks_info = Vec::new();

    // Get blocks data directly from the blockchain
    for (index, block) in blockchain.blocks.iter().enumerate() {
        blocks_info.push(BlockInfo {
            height: index as u64,
            hash: block.hash(),
            previous_hash: block.header.previous_hash.clone(),
            timestamp: block.header.timestamp as i64, // Cast to i64
            transaction_count: block.transactions.len(),
        });
    }

    Ok(Json(blocks_info))
}

pub async fn get_pending_transactions(
    State(blockchain): State<Arc<Mutex<Blockchain>>>,
) -> Result<Json<Vec<TransactionInfo>>, StatusCode> {
    let blockchain = blockchain.lock().await;
    let mut transactions_info = Vec::new();

    // Get pending transactions directly from the blockchain
    let _pending_count = blockchain.pending_transactions.len();
    for tx in &blockchain.pending_transactions {
        transactions_info.push(TransactionInfo {
            hash: tx.hash(),
            from: tx.from.clone(),
            to: tx.to.clone(),
            amount: tx.amount,
            timestamp: tx.timestamp as i64, // Cast to i64
        });
    }

    Ok(Json(transactions_info))
}

pub async fn mine_block(
    State(blockchain): State<Arc<Mutex<Blockchain>>>,
    Json(payload): Json<MineBlockRequest>,
) -> Result<Json<BlockInfo>, StatusCode> {
    // Convert private key to signing key
    let signing_key = match crypto_utils::hex_to_signing_key(&payload.private_key) {
        Ok(key) => key,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

    let mut blockchain = blockchain.lock().await;

    // Mine a new block
    let new_block = match blockchain.mine_block(&signing_key) {
        Ok(block) => block,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let block_info = BlockInfo {
        height: new_block.header.height,
        hash: new_block.hash(),
        previous_hash: new_block.header.previous_hash,
        timestamp: new_block.header.timestamp as i64, // Cast to i64
        transaction_count: new_block.transactions.len(),
    };

    // Log the mining event
    if let Err(_) = audit_log::log_system_event(
        "Block mined".to_string(),
        format!("New block {} mined successfully", block_info.hash),
        "success".to_string(),
    ) {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(Json(block_info))
}
