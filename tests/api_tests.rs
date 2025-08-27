use owami_network::api::blockchain as blockchain_api;
use owami_network::api::token as token_api;
use owami_network::api::dapp as dapp_api;
use owami_network::blockchain::Blockchain;
use owami_network::crypto_utils;
use sqlx::sqlite::SqlitePoolOptions;
use axum::extract::State;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::test]
async fn test_blockchain_api() {
    // Setup blockchain state
    let validator_key = crypto_utils::default_signing_key();
    let blockchain = Arc::new(Mutex::new(Blockchain::new(&validator_key)));
    
    // Test blockchain API endpoints
    let result = blockchain_api::get_info(State(blockchain)).await;
    assert!(result.is_ok());
    
    let blockchain_info = result.unwrap();
    assert!(blockchain_info.block_count > 0);
}

#[tokio::test]
async fn test_token_api() {
    // Test token API endpoints
    let result = token_api::get_token_info().await;
    assert!(result.is_ok());
    
    let token_info = result.unwrap();
    assert!(!token_info.name.is_empty());
    assert!(!token_info.symbol.is_empty());
}

#[tokio::test]
async fn test_dapp_api() {
    // Create a test database
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite::memory:")
        .await
        .expect("Failed to create test database");

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

    // Test DApp API endpoints
    let result = dapp_api::list_dapps(State(pool)).await;
    assert!(result.is_ok());
    
    let dapps = result.unwrap();
    // Should be an empty array initially
    assert!(dapps.data.is_none() || dapps.data.as_ref().unwrap().is_empty());
}
