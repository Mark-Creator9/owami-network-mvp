use axum::{
    routing::{get, post},
    Router,
    http::Method,
};
use tower_http::cors::{Any as CorsAny, CorsLayer};
use tower_http::services::ServeDir;
use std::sync::Arc;
use std::str::FromStr;
use tokio::sync::Mutex;
use tracing::{info, error, warn};
use tracing_subscriber;

use owami_network::blockchain::Blockchain;
use owami_network::api::{blockchain as blockchain_api, token as token_api, dapp as dapp_api};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    info!("Starting Owami Network Testnet...");

    // Database connection - SQLite for testing
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:owami_testnet.db".to_string());
    
    info!("Connecting to database: {}", database_url);
    
    // Create SQLite connection with create_if_missing option
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(
            sqlx::sqlite::SqliteConnectOptions::from_str(&database_url)?
                .create_if_missing(true)
        )
        .await
        .unwrap_or_else(|e| {
            error!("Failed to connect to database: {}", e);
            panic!("Database connection required for operation");
        });
    
    // Run migrations
    if let Err(e) = sqlx::migrate!("./migrations").run(&pool).await {
        warn!("Migration failed: {}. This is expected for some database types.", e);
    } else {
        info!("Database migrations completed successfully");
    }

    // Initialize blockchain
    let validator_key = owami_network::crypto_utils::default_signing_key();
    let blockchain = Arc::new(Mutex::new(Blockchain::new(&validator_key)));

    // Setup CORS
    let cors = CorsLayer::new()
        .allow_origin(CorsAny)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(CorsAny);

    // Build our application with routes
    let blockchain_routes = Router::new()
        // Blockchain routes
        .route("/api/blockchain/info", get(blockchain_api::get_info))
        .route("/api/blockchain/blocks", get(blockchain_api::get_blocks))
        .route("/api/blockchain/mine", post(blockchain_api::mine_block))
        // Token routes
        .route("/api/token/info", get(token_api::get_token_info))
        .route("/api/token/balance/:address", get(token_api::get_balance))
        .route("/api/token/transfer", post(token_api::transfer))
        .route("/api/token/mint/:address", post(token_api::mint))
        .route("/api/token/mint", post(token_api::mint_tokens))
        .route("/api/token/transactions", get(token_api::get_transactions))
        .with_state(blockchain);
    
    let dapp_routes = Router::new()
        // DApp management routes
        .route("/api/dapps", get(dapp_api::list_dapps))
        .route("/api/dapps", post(dapp_api::create_dapp))
        .route("/api/dapps/:id", get(dapp_api::get_dapp))
        // Legacy deploy/call routes
        .route("/api/deploy", post(dapp_api::deploy_contract))
        .route("/api/call", post(dapp_api::call_contract))
        .with_state(pool);

    // Health check endpoint
    let health_routes = Router::new()
        .route("/api/health", get(|| async {
            axum::Json(serde_json::json!({
                "status": "healthy",
                "network": "owami-testnet",
                "token": "0x742d35Cc6634C0532925a3b8D4e6D3b6e8d3e8A0"
            }))
        }));
    
    let app = blockchain_routes
        .merge(dapp_routes)
        .merge(health_routes)
        // Serve static files from landing directory
        .nest_service("/landing", ServeDir::new("landing"))
        .nest_service("/", ServeDir::new("landing"))
        .layer(cors);

    // Get port from environment variable
    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    // Run the server
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    info!("Server listening on http://0.0.0.0:{}", port);
    
    axum::serve(listener, app).await?;

    Ok(())
}
