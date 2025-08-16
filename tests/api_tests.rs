use actix_web::{test, App, web};
use owami_network::api::{config, BatchProcessor};
use owami_network::db::DatabasePool;
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;

async fn create_test_pool() -> DatabasePool {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:owamitest@localhost:5432/owami_network")
        .await
        .expect("Failed to create test database pool");

    // Create a test user if it doesn't exist
    sqlx::query(
        "INSERT INTO users (id, username, password_hash, created_at, updated_at)
         VALUES ($1, $2, $3, NOW(), NOW())
         ON CONFLICT (id) DO NOTHING"
    )
    .bind(Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap())
    .bind("test_user")
    .bind("test_hash")
    .execute(&pool)
    .await
    .expect("Failed to create test user");

    pool
}

#[actix_web::test]
async fn test_wallet_creation() {
    let batch_processor = web::Data::new(BatchProcessor::new());
    let db_pool = web::Data::new(create_test_pool().await);
    let app = test::init_service(
        App::new()
            .app_data(db_pool)
            .configure(|cfg| config(cfg, batch_processor.clone()))
    ).await;
    let req = test::TestRequest::post().uri("/wallets/create").to_request();
    let resp = test::call_service(&app, req).await;
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body["address"].is_string());
    assert!(body["private_key"].is_string());
}

#[actix_web::test]
async fn test_balance_retrieval() {
    let batch_processor = web::Data::new(BatchProcessor::new());
    let db_pool = web::Data::new(create_test_pool().await);
    let app = test::init_service(
        App::new()
            .app_data(db_pool)
            .configure(|cfg| config(cfg, batch_processor.clone()))
    ).await;

    // First create a wallet
    let create_req = test::TestRequest::post().uri("/wallets/create").to_request();
    let create_resp = test::call_service(&app, create_req).await;
    let create_body: serde_json::Value = test::read_body_json(create_resp).await;
    assert!(create_body["address"].is_string());
    assert!(create_body["private_key"].is_string());

    // Then check its balance
    let balance_req = test::TestRequest::get()
        .uri(&format!("/wallets/{}/balance", create_body["address"]))
        .to_request();
    let balance_resp = test::call_service(&app, balance_req).await;
    let balance_body: serde_json::Value = test::read_body_json(balance_resp).await;
    assert!(balance_body["balance"].is_u64());
}

#[actix_web::test]
async fn test_faucet_request() {
    let batch_processor = web::Data::new(BatchProcessor::new());
    let db_pool = web::Data::new(create_test_pool().await);
    let app = test::init_service(
        App::new()
            .app_data(db_pool)
            .configure(|cfg| config(cfg, batch_processor.clone()))
    ).await;

    // First create a wallet
    let create_req = test::TestRequest::post().uri("/wallets/create").to_request();
    let create_resp = test::call_service(&app, create_req).await;
    let create_body: serde_json::Value = test::read_body_json(create_resp).await;
    assert!(create_body["address"].is_string());
    assert!(create_body["private_key"].is_string());

    // Then request test tokens
    let faucet_req = test::TestRequest::post()
        .uri("/faucet")
        .set_json(&serde_json::json!({"address": create_body["address"]}))
        .to_request();
    let faucet_resp = test::call_service(&app, faucet_req).await;
    let faucet_body: serde_json::Value = test::read_body_json(faucet_resp).await;
    assert!(faucet_body["status"].is_string());
}

#[actix_web::test]
async fn test_send_transaction() {
    let batch_processor = web::Data::new(BatchProcessor::new());
    let db_pool = web::Data::new(create_test_pool().await);
    let app = test::init_service(
        App::new()
            .app_data(db_pool)
            .configure(|cfg| config(cfg, batch_processor.clone()))
    ).await;

    // Create two wallets
    let wallet1 = test::call_service(&app,
        test::TestRequest::post().uri("/wallets/create").to_request()
    ).await;
    let wallet2 = test::call_service(&app,
        test::TestRequest::post().uri("/wallets/create").to_request()
    ).await;
    let wallet1: serde_json::Value = test::read_body_json(wallet1).await;
    let wallet2: serde_json::Value = test::read_body_json(wallet2).await;

    // Fund wallet1
    test::call_service(&app,
        test::TestRequest::post()
            .uri("/faucet")
            .set_json(&serde_json::json!({"address": wallet1["address"]}))
            .to_request()
    ).await;

    // Send transaction
    let tx_req = test::TestRequest::post()
        .uri("/transactions")
        .set_json(&serde_json::json!({
            "from": wallet1["address"],
            "to": wallet2["address"],
            "amount": 10
        }))
        .to_request();
    let tx_resp = test::call_service(&app, tx_req).await;
    let tx_body: serde_json::Value = test::read_body_json(tx_resp).await;
    if !tx_body["status"].as_str().unwrap_or("").to_lowercase().contains("queued") {
        println!("Error response: {}", tx_body);
    }
    assert!(tx_body["status"].as_str().unwrap_or("").to_lowercase().contains("queued"));
}

#[actix_web::test]
async fn test_transaction_history() {
    let batch_processor = web::Data::new(BatchProcessor::new());
    let db_pool = web::Data::new(create_test_pool().await);
    let app = test::init_service(
        App::new()
            .app_data(db_pool)
            .configure(|cfg| config(cfg, batch_processor.clone()))
    ).await;

    // Create and fund a wallet
    let wallet = test::call_service(&app,
        test::TestRequest::post().uri("/wallets/create").to_request()
    ).await;
    let wallet: serde_json::Value = test::read_body_json(wallet).await;
    test::call_service(&app,
        test::TestRequest::post()
            .uri("/faucet")
            .set_json(&serde_json::json!({"address": wallet["address"]}))
            .to_request()
    ).await;

    // Check transaction history
    let history_req = test::TestRequest::get()
        .uri(&format!("/wallets/{}/transactions", wallet["address"]))
        .to_request();
    let history_resp = test::call_service(&app, history_req).await;
    let history_body: serde_json::Value = test::read_body_json(history_resp).await;
    if !history_body.is_array() {
        println!("Error response: {}", history_body);
    }
    assert!(history_body.is_array());
}
