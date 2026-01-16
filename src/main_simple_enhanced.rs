// Main file without RocksDB dependency - for Render deployment
// This avoids the libclang issue while maintaining functionality

use axum::{
    extract::Path,
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use chrono::{SecondsFormat, Utc};
use owami_network::{
    block::Block, blockchain::Blockchain, config::AppConfig, crypto_utils::generate_keypair,
    wallet::Wallet,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    env,
    sync::{Arc, Mutex},
};
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

// Import our pure Rust database instead of RocksDB
mod db_pure_rust;
use db_pure_rust::PureRustDatabase;

#[derive(Clone, Serialize, Deserialize)]
struct SimpleContract {
    pub id: String,
    pub name: String,
    pub owner_address: String,
    pub description: String,
    pub category: String,
    pub code: String,
    pub created_at: u64,
    pub state: serde_json::Value,
}

#[derive(Clone)]
struct SimpleState {
    blockchain: Arc<Mutex<Blockchain>>,
    wallets: Arc<HashMap<String, Wallet>>,
    database: Arc<PureRustDatabase>,
    #[allow(dead_code)]
    dapps: Arc<Mutex<HashMap<String, SimpleContract>>>,
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
    let blockchain = state.blockchain.lock().unwrap();

    Json(BlockchainInfo {
        chain_length: blockchain.blocks.len(),
        latest_block_hash: blockchain.get_latest_block().hash(),
        difficulty: 4,
        total_transactions: blockchain.blocks.iter().map(|b| b.transactions.len()).sum(),
    })
}

async fn mine_block(
    State(state): State<SimpleState>,
    Json(_request): Json<MineBlockRequest>,
) -> Json<MineBlockResponse> {
    let mut blockchain = state.blockchain.lock().unwrap();
    let signing_key = owami_network::crypto_utils::default_signing_key();

    match blockchain.mine_block(&signing_key) {
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
    let mut blockchain = state.blockchain.lock().unwrap();

    let mut tx = owami_network::transaction::Transaction::new(
        request.sender,
        request.receiver,
        request.amount as u64,
        Some(request.data),
    );

    let signing_key = owami_network::crypto_utils::default_signing_key();
    match tx.sign(&signing_key) {
        Ok(_) => {
            let tx_hash = tx.hash();
            match blockchain.add_transaction(tx) {
                Ok(_) => Json(AddTransactionResponse {
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
        Err(e) => Json(AddTransactionResponse {
            success: false,
            transaction_hash: None,
            message: format!("Failed to sign transaction: {}", e),
        }),
    }
}

async fn get_block(
    State(state): State<SimpleState>,
    axum::extract::Path(block_index): axum::extract::Path<usize>,
) -> Json<GetBlockResponse> {
    let blockchain = state.blockchain.lock().unwrap();

    match blockchain.get_block_by_height(block_index as u64) {
        Some(block) => Json(GetBlockResponse {
            success: true,
            block: Some(block.clone()),
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
    let blockchain = state.blockchain.lock().unwrap();
    let blocks = blockchain.blocks.clone();

    Json(GetBlocksResponse {
        success: true,
        blocks,
        message: "Blocks retrieved successfully".to_string(),
    })
}

async fn create_wallet() -> Json<serde_json::Value> {
    let (private_key, public_key) = generate_keypair();

    Json(serde_json::json!({
        "private_key": hex::encode(private_key.to_bytes()),
        "public_key": hex::encode(public_key.to_bytes()),
    }))
}

async fn get_transactions(State(state): State<SimpleState>) -> Json<serde_json::Value> {
    let blockchain = state.blockchain.lock().unwrap();
    let mut transactions: Vec<serde_json::Value> = Vec::new();

    for block in &blockchain.blocks {
        for tx in &block.transactions {
            transactions.push(serde_json::json!({
                "hash": tx.hash(),
                "from": tx.from,
                "to": tx.to,
                "amount": tx.amount,
                "data": tx.data,
                "timestamp": block.header.timestamp,
                "block_height": block.header.height
            }));
        }
    }

    Json(serde_json::json!({
        "success": true,
        "transactions": transactions,
        "total": transactions.len()
    }))
}

async fn get_balance(
    State(state): State<SimpleState>,
    Path(address): Path<String>,
) -> Json<serde_json::Value> {
    let blockchain = state.blockchain.lock().unwrap();
    let balance = blockchain.get_balance(&address);

    Json(serde_json::json!({
        "success": true,
        "address": address,
        "balance": balance
    }))
}

async fn mint_tokens(
    State(state): State<SimpleState>,
    Json(request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    let address = request["address"].as_str().unwrap_or("");
    let amount = request["amount"].as_u64().unwrap_or(0);

    if address.is_empty() || amount == 0 {
        return Json(serde_json::json!({
            "success": false,
            "error": "Invalid address or amount"
        }));
    }

    let mut blockchain = state.blockchain.lock().unwrap();
    match blockchain.mint(address.to_string(), amount) {
        Ok(_) => Json(serde_json::json!({
            "success": true,
            "message": "Tokens minted successfully",
            "address": address,
            "amount": amount
        })),
        Err(e) => Json(serde_json::json!({
            "success": false,
            "error": e
        })),
    }
}

async fn serve_static(Path(file_path): Path<String>) -> impl IntoResponse {
    let path = std::path::Path::new("landing").join(&file_path);
    match std::fs::read_to_string(&path) {
        Ok(content) => {
            let mime_type = if file_path.ends_with(".css") {
                "text/css"
            } else if file_path.ends_with(".js") {
                "application/javascript"
            } else {
                "text/plain"
            };
            (
                [(axum::http::header::CONTENT_TYPE, mime_type)],
                Html(content),
            )
                .into_response()
        }
        Err(_) => (StatusCode::NOT_FOUND, "File not found").into_response(),
    }
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

async fn root() -> impl IntoResponse {
    let html_content = std::fs::read_to_string("landing/index.html").unwrap_or_else(|_| {
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Owami Network - UI Not Found</title>
    <style>
        body {
            font-family: Arial, sans-serif;
            display: flex;
            justify-content: center;
            align-items: center;
            height: 100vh;
            margin: 0;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
        }
        .container {
            text-align: center;
            padding: 40px;
            background: rgba(255, 255, 255, 0.1);
            border-radius: 20px;
            box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
            backdrop-filter: blur(10px);
        }
        h1 { margin-bottom: 20px; }
        .api-link {
            color: #fff;
            background: rgba(255, 255, 255, 0.2);
            padding: 10px 20px;
            border-radius: 5px;
            text-decoration: none;
            display: inline-block;
            margin-top: 20px;
        }
        .api-link:hover { background: rgba(255, 255, 255, 0.3); }
    </style>
</head>
<body>
    <div class="container">
        <h1>Owami Network MVP</h1>
        <p>The UI files were not found. Please check the landing directory.</p>
        <p>API is available at <a href="/api/blockchain/info" class="api-link">/api/blockchain/info</a></p>
    </div>
</body>
</html>"#.to_string()
    });
    Html(html_content)
}

async fn deploy_dapp(
    State(state): State<SimpleState>,
    Json(request): Json<serde_json::Value>,
) -> impl IntoResponse {
    let name = request["name"].as_str().unwrap_or("");
    let description = request["description"].as_str().unwrap_or("");
    let category = request["category"].as_str().unwrap_or("");
    let code = request["code"].as_str().unwrap_or("");
    let owner_address = request["owner_address"].as_str().unwrap_or("");

    let dapp = SimpleContract {
        id: format!("dapp_{}", chrono::Utc::now().timestamp()),
        name: name.to_string(),
        owner_address: owner_address.to_string(),
        description: description.to_string(),
        category: category.to_string(),
        code: code.to_string(),
        created_at: chrono::Utc::now().timestamp() as u64,
        state: serde_json::json!({
            "count": 0
        }),
    };

    let mut dapps = state.dapps.lock().unwrap();
    dapps.insert(dapp.id.clone(), dapp.clone());

    Json(serde_json::json!({
        "success": true,
        "dapp_id": dapp.id,
        "message": "DApp deployed successfully"
    }))
}

async fn interact_dapp(
    State(state): State<SimpleState>,
    Json(request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    let dapp_id = request["dapp_id"].as_str().unwrap_or("");
    let function_name = request["function_name"].as_str().unwrap_or("");
    let args = request["args"]
        .as_object()
        .cloned()
        .unwrap_or_else(|| serde_json::Map::new());

    let mut dapps = state.dapps.lock().unwrap();
    let dapp = dapps.get(dapp_id);

    match dapp {
        Some(dapp) => {
            let code = dapp.code.clone();
            let result = match function_name {
                "increment" => {
                    if let serde_json::Value::Object(state) = &dapp.state {
                        let count = state.get("count").and_then(|v| v.as_i64()).unwrap_or(0);
                        serde_json::json!(count + 1)
                    } else {
                        serde_json::json!(1)
                    }
                }
                "decrement" => {
                    if let serde_json::Value::Object(state) = &dapp.state {
                        let count = state.get("count").and_then(|v| v.as_i64()).unwrap_or(0);
                        serde_json::json!(count - 1)
                    } else {
                        serde_json::json!(0)
                    }
                }
                "getCount" => if let serde_json::Value::Object(state) = &dapp.state {
                    state.get("count").unwrap_or(&serde_json::Value::Null)
                } else {
                    &serde_json::Value::Null
                }
                .clone(),
                _ => serde_json::json!({"error": "Unknown function"}),
            };

            Json(serde_json::json!({
                "success": true,
                "result": result
            }))
        }
        None => Json(serde_json::json!({
            "success": false,
            "error": "DApp not found"
        })),
    }
}

async fn get_dapps(State(state): State<SimpleState>) -> Json<serde_json::Value> {
    let dapps = state.dapps.lock().unwrap();
    let dapps_vec: Vec<SimpleContract> = dapps.values().cloned().collect();

    Json(serde_json::json!({
        "success": true,
        "dapps": dapps_vec
    }))
}

fn main() {
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
            logging: owami_network::config::LoggingConfig {
                level: "info".to_string(),
                format: "json".to_string(),
            },
            monitoring: owami_network::config::MonitoringConfig {
                health_check_interval: 30,
                metrics_port: 9090,
            },
            security: owami_network::config::SecurityConfig {
                cors_origins: vec!["*".to_string()],
                rate_limiting: owami_network::config::RateLimitingConfig {
                    requests: 100,
                    per_seconds: 60,
                },
            },
            consensus: owami_network::config::ConsensusConfig {
                consensus_type: "dpos".to_string(),
                dpos: owami_network::config::DposConfig {
                    validator_count: 21,
                    block_interval: 5,
                    stake_threshold: 1000000,
                    slashing_penalty: 5000,
                },
            },
        },
    };

    // Initialize blockchain
    let blockchain = Arc::new(Mutex::new(owami_network::blockchain::Blockchain::new(
        &config,
    )));

    // Initialize wallets
    let wallets = Arc::new(HashMap::new());

    // Initialize pure Rust database instead of RocksDB
    let database =
        Arc::new(PureRustDatabase::new("data/rocksdb").expect("Failed to initialize database"));

    let state = SimpleState {
        blockchain: blockchain.clone(),
        wallets,
        database,
        dapps: Arc::new(std::sync::Mutex::new(HashMap::new())),
    };

    // Set up CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build router
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .route("/api/blockchain/info", get(blockchain_info))
        .route("/api/blockchain/mine", post(mine_block))
        .route("/api/blockchain/transactions", post(add_transaction))
        .route("/api/blockchain/blocks/:block_index", get(get_block))
        .route("/api/blockchain/blocks", get(get_blocks))
        .route("/api/wallet/create", get(create_wallet))
        .route("/api/wallet/balance/:address", get(get_balance))
        .route("/api/wallet/mint", post(mint_tokens))
        .route("/api/transactions", get(get_transactions))
        .route("/api/dapps/deploy", post(deploy_dapp))
        .route("/api/dapps/interact", post(interact_dapp))
        .route("/api/dapps", get(get_dapps))
        .nest_service("/css", ServeDir::new("landing/css"))
        .nest_service("/js", ServeDir::new("landing/js"))
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

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let listener =
                tokio::net::TcpListener::bind(format!("{}:{}", server_host, server_port))
                    .await
                    .unwrap();

            axum::serve(listener, app).await.unwrap();
        });
}
