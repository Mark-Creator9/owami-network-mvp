use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::blockchain::Blockchain;

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockchainInfo {
    pub height: u64,
    pub block_count: usize,
    pub pending_transactions: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockResponse {
    pub height: u64,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: u64,
    pub transaction_count: usize,
}

pub async fn get_info(
    State(blockchain): State<Arc<Mutex<Blockchain>>>,
) -> Result<Json<BlockchainInfo>, StatusCode> {
    let blockchain = blockchain.lock().await;
    
    let info = BlockchainInfo {
        height: blockchain.get_height(),
        block_count: blockchain.blocks.len(),
        pending_transactions: blockchain.pending_transactions.len(),
    };
    
    Ok(Json(info))
}

pub async fn get_blocks(
    State(blockchain): State<Arc<Mutex<Blockchain>>>,
) -> Result<Json<Vec<BlockResponse>>, StatusCode> {
    let blockchain = blockchain.lock().await;
    
    let blocks: Vec<BlockResponse> = blockchain.blocks
        .iter()
        .map(|block| BlockResponse {
            height: block.header.height,
            hash: block.hash(),
            previous_hash: block.header.previous_hash.clone(),
            timestamp: block.header.timestamp,
            transaction_count: block.transactions.len(),
        })
        .collect();
    
    Ok(Json(blocks))
}

pub async fn mine_block(
    State(blockchain): State<Arc<Mutex<Blockchain>>>,
) -> Result<Json<BlockResponse>, StatusCode> {
    use crate::crypto_utils;
    
    let mut blockchain = blockchain.lock().await;
    
    if blockchain.pending_transactions.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    
    let signing_key = crypto_utils::default_signing_key();
    
    let latest_height = blockchain.get_latest_block()
        .map(|b| b.header.height)
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let latest_hash = blockchain.get_latest_block()
        .map(|b| b.hash())
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let transactions = std::mem::take(&mut blockchain.pending_transactions);
    
    let block = crate::block::Block::new(
        latest_height + 1,
        latest_hash,
        transactions,
        &signing_key,
    );
    
    blockchain.add_block(block.clone())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let response = BlockResponse {
        height: block.header.height,
        hash: block.hash(),
        previous_hash: block.header.previous_hash.clone(),
        timestamp: block.header.timestamp,
        transaction_count: block.transactions.len(),
    };
    
    Ok(Json(response))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transaction::Transaction;

    async fn setup_blockchain() -> Arc<Mutex<Blockchain>> {
        let validator_key = crate::crypto_utils::default_signing_key();
        let blockchain = Blockchain::new(&validator_key);
        Arc::new(Mutex::new(blockchain))
    }

    #[tokio::test]
    async fn test_get_info() {
        let blockchain = setup_blockchain().await;
        
        let result = get_info(State(blockchain)).await;
        assert!(result.is_ok());
        
        let info = result.unwrap();
        assert_eq!(info.height, 0);
        assert_eq!(info.block_count, 1);
    }

    #[tokio::test]
    async fn test_get_blocks() {
        let blockchain = setup_blockchain().await;
        
        let result = get_blocks(State(blockchain)).await;
        assert!(result.is_ok());
        
        let blocks = result.unwrap();
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].height, 0);
    }

    #[tokio::test]
    async fn test_mine_block() {
        let blockchain = setup_blockchain().await;
        
        // Add a transaction
        let mut bc = blockchain.lock().await;
        let signing_key = crate::crypto_utils::default_signing_key();
        let public_key = signing_key.verifying_key();
        let address = hex::encode(public_key.to_bytes());
        
        bc.mint(address.clone(), 1000);
        
        let tx = Transaction::new(
            address,
            "recipient".to_string(),
            100,
            None,
            &signing_key,
        );
        
        bc.add_transaction(tx).unwrap();
        drop(bc);
        
        let result = mine_block(State(blockchain)).await;
        assert!(result.is_ok());
        
        let block = result.unwrap();
        assert_eq!(block.height, 1);
        assert_eq!(block.transaction_count, 1);
    }
}