use actix_web::{post, get, web, HttpResponse};
use actix_web::web::ServiceConfig;
use crate::wallet::Wallet;
use crate::vesting::VestingManager;
use crate::auth::key_manager::ApiKeyManager;
use serde::{Serialize, Deserialize};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::mpsc;
use std::sync::Arc;
use std::collections::VecDeque;
use tokio::time::{interval, Duration};

const TRANSACTION_BATCH_SIZE: usize = 50;
const BATCH_INTERVAL_MS: u64 = 100;

#[derive(Serialize, Deserialize, Clone)]
struct WalletResponse {
    address: String,
    private_key: String,
}

#[derive(Serialize, Deserialize)]
struct BalanceResponse {
    balance: u64,
}

#[derive(Serialize, Deserialize)]
struct FaucetResponse {
    status: String,
}

#[derive(Clone)]
pub struct BatchProcessor {
    tx_queue: Arc<parking_lot::Mutex<VecDeque<TransactionRequest>>>,
    batch_sender: mpsc::Sender<Vec<TransactionRequest>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransactionRequest {
    pub from: String,
    pub to: String,
    pub amount: u64,
}

impl BatchProcessor {
    pub fn new() -> Self {
        let (tx, _) = mpsc::channel(100);
        let queue = Arc::new(parking_lot::Mutex::new(VecDeque::new()));
        let queue_clone = queue.clone();

        // Start batch processing task
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_millis(BATCH_INTERVAL_MS));

            loop {
                interval.tick().await;
                let mut queue = queue_clone.lock();
                if queue.len() >= TRANSACTION_BATCH_SIZE {
                    let batch: Vec<_> = queue.drain(..TRANSACTION_BATCH_SIZE).collect();
                    log::info!("Processing batch of {} transactions", batch.len());
                    // Process the batch here
                }
            }
        });

        BatchProcessor {
            tx_queue: queue,
            batch_sender: tx,
        }
    }

    pub async fn add_transaction(&self, tx: TransactionRequest) {
        let mut queue = self.tx_queue.lock();
        queue.push_back(tx);

        if queue.len() >= TRANSACTION_BATCH_SIZE {
            let batch: Vec<_> = queue.drain(..TRANSACTION_BATCH_SIZE).collect();
            if let Err(e) = self.batch_sender.send(batch).await {
                log::error!("Failed to send batch: {}", e);
            }
        }
    }
}

#[post("/wallets/create")]
async fn create_wallet() -> HttpResponse {
    let wallet = Wallet::new();
    let address = wallet.address().to_string();
    let private_key = wallet.private_key().unwrap().to_string();

    // Removed database insert, just return wallet info

    HttpResponse::Ok().json(WalletResponse {
        address,
        private_key,
    })
}

#[get("/wallets/{address}/balance")]
async fn get_balance(path: web::Path<String>) -> HttpResponse {
    // This is a placeholder. In a real implementation, you would query
    // the blockchain state for the actual balance.
    let _address = path.into_inner();
    HttpResponse::Ok().json(BalanceResponse { balance: 1000 })
}

#[post("/faucet")]
async fn faucet(req: web::Json<serde_json::Value>) -> HttpResponse {
    let address = req.get("address");
    if address.is_none() {
        return HttpResponse::BadRequest().json(serde_json::json!({"error": "Missing address"}));
    }

    HttpResponse::Ok().json(FaucetResponse { status: "success".to_string() })
}

#[post("/transactions")]
async fn send_transaction(
    batch_processor: web::Data<BatchProcessor>,
    tx: web::Json<TransactionRequest>,
) -> HttpResponse {
    batch_processor.add_transaction(tx.into_inner()).await;
    HttpResponse::Ok().json(serde_json::json!({"status": "Transaction queued"}))
}

#[get("/wallets/{address}/transactions")]
async fn transactions(path: web::Path<String>) -> HttpResponse {
    let _address = path.into_inner();
    // This is a placeholder. In a real implementation, you would query
    // the blockchain state for the actual transaction history.
    HttpResponse::Ok().json(Vec::<serde_json::Value>::new())
}

#[get("/health")]
async fn health() -> HttpResponse {
    let uptime = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    HttpResponse::Ok().json(serde_json::json!({
        "status": "up",
        "uptime": uptime,
        "version": env!("CARGO_PKG_VERSION")
    }))
}

#[derive(Deserialize)]
struct VestingAddress {
    address: String,
}

#[get("/vesting/claimable")]
async fn get_claimable(
    vesting_manager: web::Data<Arc<Mutex<VestingManager>>>,
    query: web::Query<VestingAddress>,
) -> HttpResponse {
    let manager = vesting_manager.lock().unwrap();
    let claimable = manager.get_claimable_amount(&query.address);
    HttpResponse::Ok().json(serde_json::json!({ "claimable": claimable }))
}

#[post("/vesting/claim")]
async fn claim_vested(
    vesting_manager: web::Data<Arc<Mutex<VestingManager>>>,
    req: web::Json<VestingAddress>,
) -> HttpResponse {
    let mut manager = vesting_manager.lock().unwrap();
    manager.claim_vested_tokens(&req.address);
    HttpResponse::Ok().json(serde_json::json!({ "status": "success" }))
}

#[derive(Deserialize)]
struct GenerateKeyRequest {
    email: String,
}

#[post("/keys/generate")]
async fn generate_api_key(req: web::Json<GenerateKeyRequest>) -> HttpResponse {
    let key_manager = ApiKeyManager::new();
    let api_key = key_manager.generate_key(&req.email);
    HttpResponse::Ok().json(api_key)
}

pub fn config(cfg: &mut ServiceConfig, batch_processor: web::Data<BatchProcessor>) {
    cfg
        .app_data(batch_processor)
        .service(health)
        .service(create_wallet)
        .service(get_balance)
        .service(faucet)
        .service(send_transaction)
        .service(transactions)
        .service(get_claimable)
        .service(claim_vested)
        .service(generate_api_key);
}
