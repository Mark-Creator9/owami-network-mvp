use actix_web::{post, get, web, HttpResponse, web::ServiceConfig};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::mpsc;
use std::sync::Arc;
use std::collections::VecDeque;
use tokio::time::{interval, Duration};

const TRANSACTION_BATCH_SIZE: usize = 50;
const BATCH_INTERVAL_MS: u64 = 100;

mod dapp;

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
    tx_queue: Arc<std::sync::Mutex<VecDeque<TransactionRequest>>>,
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
        let queue = Arc::new(std::sync::Mutex::new(VecDeque::new()));
        let queue_clone = queue.clone();

        // Start batch processing task
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_millis(BATCH_INTERVAL_MS));

            loop {
                interval.tick().await;
                let mut queue = queue_clone.lock().unwrap();
                if queue.len() >= TRANSACTION_BATCH_SIZE {
                    let batch: Vec<_> = queue.drain(..TRANSACTION_BATCH_SIZE).collect();
                    println!("Processing batch of {} transactions", batch.len());
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
        let mut queue = self.tx_queue.lock().unwrap();
        queue.push_back(tx);

        if queue.len() >= TRANSACTION_BATCH_SIZE {
            let batch: Vec<_> = queue.drain(..TRANSACTION_BATCH_SIZE).collect();
            if let Err(e) = self.batch_sender.send(batch).await {
                eprintln!("Failed to send batch: {}", e);
            }
        }
    }
}

#[post("/wallets/create")]
async fn create_wallet() -> HttpResponse {
    // Create new wallet (simulated)
    let address = "0x123456789abcdef".to_string();
    let private_key = "abcdef1234567890".to_string();

    HttpResponse::Ok().json(WalletResponse {
        address,
        private_key,
    })
}

#[get("/wallets/{address}/balance")]
async fn get_balance(path: web::Path<String>) -> HttpResponse {
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
        "version": "0.1.0"
    }))
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
        .configure(dapp::config);
}