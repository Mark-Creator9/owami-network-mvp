// Main file without RocksDB dependency - for Render deployment
// This avoids the libclang issue while maintaining functionality

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use chrono::{SecondsFormat, Utc};
use owami_network::{
    blockchain::{Block, Blockchain},
    config::AppConfig,
    crypto_utils::generate_key_pair,
    wallet::Wallet,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    env,
    sync::Arc,
};
use tower_http::cors::{Any, CorsLayer};

// Import our pure Rust database instead of RocksDB
mod db_pure_rust;
use db_pure_rust::PureRustDatabase;

#[derive(Clone)]
struct SimpleState {
    blockchain: Arc<Blockchain>,
    wallets: Arc<HashMap<String, Wallet>>,
    database: Arc<PureRustDatabase>,
}

#[derive(Serialize, Deserialize)]
struct HealthResponse {
    status: String,
    network: String,
    timestamp: String,
    database: String,
    wasm_support: bool,
}

#[derive(Serialize, Deserialize)]
struct BlockchainInfo {
    chain_length: usize,
    latest_block_hash: String,
    difficulty: u32,
    total_transactions: usize,
}

#[derive(Serialize, Deserialize)]
struct MineBlockRequest {
    transactions: Vec<String>,
    difficulty: Option<u32>,
}

#[derive(Serialize, Deserialize)]
struct MineBlockResponse {
    success: bool,
    block: Option<Block>,
    message: String,
}

#[derive(Serialize, Deserialize)]
struct AddTransactionRequest {
    sender: String,
    receiver: String,
    amount: f64,
    data: String,
}

#[derive(Serialize, Deserialize)]
struct AddTransactionResponse {
    success: bool,
    transaction_hash: Option<String>,
    message: String,
}

#[derive(Serialize, Deserialize)]
struct GetBlockResponse {
    success: bool,
    block: Option<Block>,
    message: String,
}

#[derive(Serialize, Deserialize)]
struct GetBlocksResponse {
    success: bool,
    blocks: Vec<Block>,
    message: String,
}

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    error: String,
    details: String,
}

async fn health_check(State(_state): State<SimpleState>) -> Json<HealthResponse> {
    // Use our pure Rust database health check instead of RocksDB
    let db_status = PureRustDatabase::health_check();

    Json(HealthResponse {
        status: "healthy".to_string(),
        network: "owami-testnet".to_string(),
        timestamp: Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
        database: db_status,
        wasm_support: true,
    })
}

async fn blockchain_info(State(state): State<SimpleState>) -> Json<BlockchainInfo> {
    let blockchain = state.blockchain;
    let chain = blockchain.read().unwrap();

    Json(BlockchainInfo {
        chain_length: chain.len(),
        latest_block_hash: chain.latest_hash(),
        difficulty: chain.difficulty(),
        total_transactions: chain.total_transactions(),
    })
}

async fn mine_block(
    State(state): State<SimpleState>,
    Json(request): Json<MineBlockRequest>,
) -> Json<MineBlockResponse> {
    let mut blockchain = state.blockchain.write().unwrap();
    
    let difficulty = request.difficulty.unwrap_or(4);
    let transactions = request
        .transactions
        .iter()
        .map(|t| t.as_str())
        .collect();
    
    match blockchain.mine_block(transactions, difficulty) {
        Ok(block) => Json(MineBlockResponse {
            success: true,
            block: Some(block),
            message: "Block mined successfully".to_string(),
        }),
        Err(e) => Json(MineBlockResponse {
            success: false,
            block: None,
            message: format!("Failed to mine block: {}", e),
        }),
    }
}

async fn add_transaction(
    State(state): State<SimpleState>,
    Json(request): Json<AddTransactionRequest>,
) -> Json<AddTransactionResponse> {
    let mut blockchain = state.blockchain.write().unwrap();
    
    match blockchain.add_transaction(
        request.sender,
        request.receiver,
        request.amount,
        request.data,
    ) {
        Ok(tx_hash) => Json(AddTransactionResponse {
            success: true,
            transaction_hash: Some(tx_hash),
            message: "Transaction added successfully".to_string(),
        }),
        Err(e) => Json(AddTransactionResponse {
            success: false,
            transaction_hash: None,
            message: format!("Failed to add transaction: {}", e),
        }),
    }
}

async fn get_block(
    State(state): State<SimpleState>,
    axum::extract::Path(block_index): axum::extract::Path<usize>,
) -> Json<GetBlockResponse> {
    let blockchain = state.blockchain.read().unwrap();
    
    match blockchain.get_block(block_index) {
        Some(block) => Json(GetBlockResponse {
            success: true,
            block: Some(block),
            message: "Block retrieved successfully".to_string(),
        }),
        None => Json(GetBlockResponse {
            success: false,
            block: None,
            message: "Block not found".to_string(),
        }),
    }
}

async fn get_blocks(State(state): State<SimpleState>) -> Json<GetBlocksResponse> {
    let blockchain = state.blockchain.read().unwrap();
    let blocks = blockchain.get_all_blocks();
    
    Json(GetBlocksResponse {
        success: true,
        blocks,
        message: "Blocks retrieved successfully".to_string(),
    })
}

async fn create_wallet() -> Json<serde_json::Value> {
    let (private_key, public_key) = generate_key_pair();
    
    Json(serde_json::json!({
        "private_key": private_key,
        "public_key": public_key,
    }))
}

async fn handle_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(ErrorResponse {
            error: "Not Found".to_string(),
            details: "The requested resource was not found".to_string(),
        }),
    )
}

#[tokio::main]
async fn main() {
    if env::var("CONFIG_PATH").is_err() {
        env::set_var("CONFIG_PATH", "config/testnet.toml");
    }

    // Create data directory
    std::fs::create_dir_all("data/rocksdb").unwrap_or(());

    let config = match AppConfig::load() {
        Ok(cfg) => cfg,
        Err(_) => AppConfig {
            server: owami_network::config::ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 8080,
                workers: 4,
            },
            database: owami_network::config::DatabaseConfig {
                data_dir: "./data".to_string(),
            },
        },
    };

    // Initialize blockchain
    let blockchain = Arc::new(owami_network::blockchain::Blockchain::new());

    // Initialize wallets
    let wallets = Arc::new(HashMap::new());

    // Initialize pure Rust database instead of RocksDB
    let database = Arc::new(
        PureRustDatabase::new("data/rocksdb")
            .expect("Failed to initialize database")
    );

    let state = SimpleState {
        blockchain: blockchain.clone(),
        wallets,
        database,
    };

    // Set up CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build router
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/blockchain/info", get(blockchain_info))
        .route("/api/blockchain/mine", post(mine_block))
        .route("/api/blockchain/transactions", post(add_transaction))
        .route("/api/blockchain/blocks/:block_index", get(get_block))
        .route("/api/blockchain/blocks", get(get_blocks))
        .route("/api/wallet/create", get(create_wallet))
        .layer(cors)
        .with_state(state)
        .fallback(handle_404);

    // Start server
    let server_host = env::var("HOST").unwrap_or_else(|_| config.server.host);
    let server_port = env::var("PORT")
        .unwrap_or_else(|_| config.server.port.to_string())
        .parse()
        .unwrap_or(8080);

    println!(
        "Owami Network Testnet MVP Server running on http://{}:{}",
        server_host, server_port
    );

    axum::Server::bind(&format!("{}:{}", server_host, server_port).parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}