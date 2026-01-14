use axum::{
    extract::Path,
    extract::State,
    http::{Method, StatusCode, Uri},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use chrono::{SecondsFormat, Utc};
use owami_network::blockchain::Blockchain;
use owami_network::config::AppConfig;
use owami_network::crypto_utils;
use owami_network::wallet::Wallet;
use rocksdb::DB;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::env;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    network: String,
    timestamp: String,
    database: String,
    wasm_support: bool,
}

#[derive(Serialize)]
struct BlockchainInfo {
    network: String,
    block_height: u64,
    block_count: u64,
    total_supply: String,
    version: String,
    database_status: String,
    smart_contracts: bool,
    pending_transactions: usize,
    latest_block_hash: String,
}

#[derive(Clone, Serialize, Deserialize)]
struct WalletData {
    user_id: String,
    username: String,
    address: String,
    private_key: String,
    balance: u64,
    created_at: u64,
}

#[derive(Clone)]
struct SimpleState {
    blockchain: Arc<Mutex<Blockchain>>,
    wallet_registry: Arc<Mutex<HashMap<String, WalletData>>>,
    #[allow(dead_code)]
    contract_registry: Arc<Mutex<HashMap<String, String>>>,
    faucet_requests: Arc<Mutex<HashMap<String, FaucetRequest>>>,
}

#[derive(Clone, Serialize, Deserialize)]
struct FaucetRequest {
    timestamp: i64,
    count: u32,
}

impl SimpleState {
    fn new(blockchain: Arc<Mutex<Blockchain>>) -> Self {
        Self {
            blockchain,
            wallet_registry: Arc::new(Mutex::new(HashMap::new())),
            contract_registry: Arc::new(Mutex::new(HashMap::new())),
            faucet_requests: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn register_wallet(
        &self,
        user_id: String,
        username: String,
        address: String,
        private_key: String,
    ) {
        let mut registry = self.wallet_registry.lock().unwrap();
        let wallet_data = WalletData {
            user_id: user_id.clone(),
            username: username.clone(),
            address: address.clone(),
            private_key: private_key.clone(),
            balance: 0,
            created_at: Utc::now().timestamp() as u64,
        };
        registry.insert(user_id.clone(), wallet_data.clone());

        let mut blockchain = self.blockchain.lock().unwrap();
        let _ = blockchain.mint(address.clone(), 1000);
    }

    fn get_wallet(&self, user_id: &str) -> Option<WalletData> {
        let registry = self.wallet_registry.lock().unwrap();
        registry.get(user_id).cloned()
    }

    fn get_all_wallets(&self) -> Vec<WalletData> {
        let registry = self.wallet_registry.lock().unwrap();
        registry.values().cloned().collect()
    }

    fn update_balance(&self, address: String, amount: i64) {
        let mut registry = self.wallet_registry.lock().unwrap();
        for (_id, wallet) in registry.iter_mut() {
            if wallet.address == address {
                wallet.balance = (wallet.balance as i64 + amount).max(0) as u64;
                break;
            }
        }
    }

    fn can_request_faucet(&self, user_id: &str, faucet_limit_seconds: i64) -> bool {
        let mut faucet_requests = self.faucet_requests.lock().unwrap();
        let now = Utc::now().timestamp();

        if let Some(request) = faucet_requests.get(user_id) {
            if now - request.timestamp < faucet_limit_seconds {
                return false;
            }
        }

        // Update or create the request
        if let Some(request) = faucet_requests.get_mut(user_id) {
            request.timestamp = now;
            request.count += 1;
        } else {
            faucet_requests.insert(
                user_id.to_string(),
                FaucetRequest {
                    timestamp: now,
                    count: 1,
                },
            );
        }

        true
    }

    fn get_faucet_next_request(&self, user_id: &str, faucet_limit_seconds: i64) -> Option<i64> {
        let faucet_requests = self.faucet_requests.lock().unwrap();
        let now = Utc::now().timestamp();

        if let Some(request) = faucet_requests.get(user_id) {
            let next_request = request.timestamp + faucet_limit_seconds;
            if next_request > now {
                return Some(next_request - now);
            }
        }

        None
    }
}

async fn health_check(State(_state): State<SimpleState>) -> Json<HealthResponse> {
    let db_status = match DB::open_default("data/rocksdb") {
        Ok(_) => "connected".to_string(),
        Err(_) => "disconnected".to_string(),
    };

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
    let height = blockchain.get_block_height();
    let pending_transactions = blockchain.pending_transactions.len();
    let latest_block_hash = blockchain.get_latest_block().hash();

    Json(BlockchainInfo {
        network: "owami-testnet".to_string(),
        block_height: height,
        block_count: height + 1,
        total_supply: "1000000".to_string(),
        version: "1.0.0".to_string(),
        database_status: "connected".to_string(),
        smart_contracts: true,
        pending_transactions,
        latest_block_hash,
    })
}

#[derive(Deserialize)]
struct CreateWalletRequest {
    username: String,
    #[allow(dead_code)]
    password: String,
}

async fn create_wallet(
    State(state): State<SimpleState>,
    Json(request): Json<CreateWalletRequest>,
) -> Json<serde_json::Value> {
    let mut wallet = Wallet::new().unwrap();
    let address = wallet.address().unwrap();
    let private_key = wallet.private_key().unwrap();
    let user_id = format!("user_{}", chrono::Utc::now().timestamp());

    state.register_wallet(
        user_id.clone(),
        request.username.clone(),
        address.clone(),
        private_key.clone(),
    );

    Json(serde_json::json!({
        "success": true,
        "user_id": user_id,
        "username": request.username,
        "address": address,
        "private_key": private_key,
        "message": "Wallet created successfully"
    }))
}

async fn get_test_tokens(
    State(state): State<SimpleState>,
    Json(request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    let user_id = request["user_id"].as_str().unwrap_or("").to_string();
    let amount = request["amount"].as_u64().unwrap_or(1000);
    let faucet_limit_seconds = env::var("FAUCET_RATE_LIMIT")
        .unwrap_or_else(|_| "86400".to_string())
        .parse::<i64>()
        .unwrap_or(86400);

    if let Some(next_request_seconds) =
        state.get_faucet_next_request(&user_id, faucet_limit_seconds)
    {
        let hours = next_request_seconds / 3600;
        let minutes = (next_request_seconds % 3600) / 60;

        return Json(serde_json::json!({
            "success": false,
            "error": format!("Rate limited. Please wait {}h {}m until next faucet request", hours, minutes),
            "next_request_seconds": next_request_seconds
        }));
    }

    if let Some(wallet) = state.get_wallet(&user_id) {
        let mut blockchain = state.blockchain.lock().unwrap();
        let _ = blockchain.mint(wallet.address.clone(), amount);
        state.update_balance(wallet.address.clone(), amount as i64);

        Json(serde_json::json!({
            "success": true,
            "message": format!("Received {} test tokens", amount),
            "new_balance": wallet.balance + amount
        }))
    } else {
        Json(serde_json::json!({
            "success": false,
            "error": "Wallet not found"
        }))
    }
}

async fn get_wallet_balance(
    State(state): State<SimpleState>,
    Path(user_id): Path<String>,
) -> Json<serde_json::Value> {
    if let Some(wallet) = state.get_wallet(&user_id) {
        let blockchain = state.blockchain.lock().unwrap();
        let real_balance = blockchain.get_balance(&wallet.address);

        Json(serde_json::json!({
            "address": wallet.address,
            "balance": real_balance
        }))
    } else {
        Json(serde_json::json!({
            "error": "Wallet not found"
        }))
    }
}

async fn get_token_balance(
    State(state): State<SimpleState>,
    Path(address): Path<String>,
) -> Json<serde_json::Value> {
    let blockchain = state.blockchain.lock().unwrap();
    let balance = blockchain.get_balance(&address);

    Json(serde_json::json!({
        "address": address,
        "balance": balance
    }))
}

async fn token_transfer(
    State(state): State<SimpleState>,
    Json(request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    let from = request["from"].as_str().unwrap_or("").to_string();
    let to = request["to"].as_str().unwrap_or("").to_string();
    let amount = request["amount"].as_u64().unwrap_or(0);
    let private_key_str = request["private_key"].as_str().unwrap_or("");

    if amount == 0 {
        return Json(serde_json::json!({
            "success": false,
            "error": "Amount must be greater than 0"
        }));
    }

    let signing_key = match crypto_utils::hex_to_signing_key(private_key_str) {
        Ok(key) => key,
        Err(_) => {
            return Json(serde_json::json!({
                "success": false,
                "error": "Invalid private key"
            }));
        }
    };

    let sender_address = hex::encode(signing_key.verifying_key().to_bytes());

    let mut transaction =
        owami_network::transaction::Transaction::new(sender_address, to.clone(), amount, None);

    if let Err(_) = transaction.sign(&signing_key) {
        return Json(serde_json::json!({
            "success": false,
            "error": "Failed to sign transaction"
        }));
    }

    let mut blockchain = state.blockchain.lock().unwrap();

    if let Err(e) = blockchain.add_transaction(transaction.clone()) {
        return Json(serde_json::json!({
            "success": false,
            "error": format!("Failed to add transaction: {}", e)
        }));
    }

    Json(serde_json::json!({
        "success": true,
        "hash": transaction.hash(),
        "from": transaction.from,
        "to": transaction.to,
        "amount": transaction.amount
    }))
}

async fn mine_block(
    State(state): State<SimpleState>,
    Json(request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    let private_key = request["private_key"].as_str().unwrap_or("");

    let signing_key = match crypto_utils::hex_to_signing_key(private_key) {
        Ok(key) => key,
        Err(_) => {
            return Json(serde_json::json!({
                "success": false,
                "error": "Invalid private key"
            }));
        }
    };

    let mut blockchain = state.blockchain.lock().unwrap();

    match blockchain.mine_block(&signing_key) {
        Ok(block) => Json(serde_json::json!({
            "success": true,
            "height": block.header.height,
            "hash": block.hash(),
            "transactions": block.transactions.len()
        })),
        Err(_) => Json(serde_json::json!({
            "success": false,
            "error": "Mining failed"
        })),
    }
}

async fn token_info() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "name": "Owami Token",
        "symbol": "OWA",
        "decimals": 18,
        "total_supply": 1000000
    }))
}

async fn get_blocks(State(state): State<SimpleState>) -> Json<serde_json::Value> {
    let blockchain = state.blockchain.lock().unwrap();
    let mut blocks = Vec::new();

    for (index, block) in blockchain.blocks.iter().enumerate() {
        blocks.push(serde_json::json!({
            "height": index,
            "hash": block.hash(),
            "previous_hash": block.header.previous_hash,
            "timestamp": block.header.timestamp,
            "transaction_count": block.transactions.len()
        }));
    }

    Json(serde_json::json!(blocks))
}

async fn get_transactions(State(state): State<SimpleState>) -> Json<serde_json::Value> {
    let blockchain = state.blockchain.lock().unwrap();
    let mut transactions = Vec::new();

    for block in &blockchain.blocks {
        for tx in &block.transactions {
            transactions.push(serde_json::json!({
                "hash": tx.hash(),
                "from": tx.from,
                "to": tx.to,
                "amount": tx.amount,
                "timestamp": tx.timestamp
            }));
        }
    }

    Json(serde_json::json!(transactions))
}

// Block Explorer Endpoints
async fn get_all_wallets(State(state): State<SimpleState>) -> Json<serde_json::Value> {
    let wallets = state.get_all_wallets();

    // Return only public information
    let public_wallets: Vec<serde_json::Value> = wallets
        .into_iter()
        .map(|w| {
            serde_json::json!({
                "address": w.address,
                "username": w.username,
                "balance": w.balance,
                "created_at": w.created_at
            })
        })
        .collect();

    Json(serde_json::json!({
        "count": public_wallets.len(),
        "wallets": public_wallets
    }))
}

async fn get_explorer_stats(State(state): State<SimpleState>) -> Json<serde_json::Value> {
    let blockchain = state.blockchain.lock().unwrap();
    let wallets = state.get_all_wallets();

    let total_balance: u64 = wallets.iter().map(|w| w.balance).sum();

    Json(serde_json::json!({
        "total_wallets": wallets.len(),
        "total_blocks": blockchain.blocks.len(),
        "total_transactions": blockchain.blocks.iter()
            .map(|b| b.transactions.len())
            .sum::<usize>(),
        "total_balance": total_balance,
        "network": "owami-testnet",
        "version": "1.0.0"
    }))
}

async fn serve_index() -> (StatusCode, &'static str) {
    (
        StatusCode::OK,
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <meta http-equiv="refresh" content="0;url=/mvp.html">
    <title>Redirecting to Owami Network MVP</title>
</head>
<body>
    <p>Redirecting to <a href="/mvp.html">Owami Network MVP</a>...</p>
</body>
</html>"#,
    )
}

async fn serve_mvp() -> impl axum::response::IntoResponse {
    match tokio::fs::read_to_string("landing/mvp.html").await {
        Ok(content) => axum::response::Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "text/html; charset=utf-8")
            .body(axum::body::Body::from(content))
            .unwrap(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to load mvp.html file",
        )
            .into_response(),
    }
}

async fn serve_static_files(uri: Uri) -> (StatusCode, &'static str) {
    let path = uri.path();

    if path == "/" || path.is_empty() {
        return serve_index().await;
    }

    (StatusCode::NOT_FOUND, "Not Found")
}

#[tokio::main]
async fn main() {
    if env::var("CONFIG_PATH").is_err() {
        env::set_var("CONFIG_PATH", "config/testnet.toml");
    }

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
                    requests: 200,
                    per_seconds: 60,
                },
            },
            consensus: owami_network::config::ConsensusConfig {
                consensus_type: "dpos".to_string(),
                dpos: owami_network::config::DposConfig {
                    validator_count: 7,
                    block_interval: 3,
                    stake_threshold: 1000,
                    slashing_penalty: 50,
                },
            },
        },
    };

    let blockchain = Arc::new(Mutex::new(Blockchain::new(&config)));
    let app_state = SimpleState::new(blockchain.clone());

    let cors = CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(tower_http::cors::Any);

    let app = Router::new()
        .nest_service("/landing", ServeDir::new("landing"))
        .route("/", get(serve_index))
        .route("/mvp.html", get(serve_mvp))
        .route("/index.html", get(serve_index))
        .route("/health", get(health_check))
        .route("/api/health", get(health_check))
        .route("/api/blockchain/info", get(blockchain_info))
        .route("/api/auth/register", post(create_wallet))
        .route("/api/auth/login", post(|_req: Json<CreateWalletRequest>| async { Json(serde_json::json!({"success": true, "token": "demo", "message": "Login successful"})) }))
        .route("/api/wallet/create", post(create_wallet))
        .route("/api/wallet/balance/:user_id", get(get_wallet_balance))
        .route("/api/wallet/faucet", post(get_test_tokens))
        .route("/api/wallet/all", get(get_all_wallets))
        .route("/api/token/info", get(token_info))
        .route("/api/token/balance/:address", get(get_token_balance))
        .route("/api/token/transfer", post(token_transfer))
        .route("/api/token/transactions", get(get_transactions))
        .route("/api/blockchain/blocks", get(get_blocks))
        .route("/api/blockchain/mine", post(mine_block))
        .route("/api/dapps", get(|| async { Json(serde_json::json!({ "dapps": [] })) }))
        .route("/api/explorer/stats", get(get_explorer_stats))
        .route("/api/explorer/wallets", get(get_all_wallets))
        .fallback(get(serve_static_files))
        .layer(cors)
        .with_state(app_state);

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8081".to_string())
        .parse::<u16>()
        .unwrap_or(config.server.port);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    println!("\n==========================================");
    println!("  Owami Network Server Started");
    println!("==========================================");
    println!("  Frontend: http://localhost:{}", port);
    println!("  API:      http://localhost:{}/api", port);
    println!("  Explorer: http://localhost:{}/api/explorer/stats", port);
    println!("==========================================\n");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap_or_else(|e| {
            panic!("Failed to bind to {}: {}", addr, e);
        });
    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}
