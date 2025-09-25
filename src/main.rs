use axum::{
    routing::{get, post},
    Router,
    http::{Method, HeaderValue},
};
use tower_http::cors::{Any as CorsAny, CorsLayer};
use tower_http::services::ServeDir;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, error, warn};
use tracing_subscriber;

use owami_network::blockchain::Blockchain;
use owami_network::api::{blockchain as blockchain_api, token as token_api, dapp as dapp_api, auth};
use owami_network::config::AppConfig;
use owami_network::db::BlockchainRepository;
use owami_network::key_management::KeyManager;
use owami_network::audit_log;
use owami_network::rate_limiting::{RateLimiterState, rate_limiter_middleware, ddos_protection_middleware};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = AppConfig::load()?;
    info!("Loaded configuration: {:?}", config);

    // Initialize tracing with config
    tracing_subscriber::fmt()
        .with_max_level(match config.logging.level.as_str() {
            "debug" => tracing::Level::DEBUG,
            "info" => tracing::Level::INFO,
            "warn" => tracing::Level::WARN,
            "error" => tracing::Level::ERROR,
            _ => tracing::Level::INFO,
        })
        .init();

    info!("Starting Owami Network Production Server...");

    // Database connection - use configured pool size via db module
    info!("Connecting to database with configuration: {:?}", config.database);
    
    let pool = owami_network::db::create_pool(&config).await.unwrap_or_else(|e| {
        error!("Failed to connect to database: {}", e);
        panic!("Database connection required for operation");
    });
    
    // Run migrations
    if let Err(e) = sqlx::migrate!("./migrations").run(&pool).await {
        warn!("Migration failed: {}. This is expected for some database types.", e);
    } else {
        info!("Database migrations completed successfully");
    }

    // Initialize audit logger
    audit_log::initialize_audit_logger(None)?;
    
    // Initialize key manager for validator
    let mut key_manager = KeyManager::new(None)?;
    let validator_key = key_manager.load_or_generate_key()?;
    
    // Create blockchain repository and initialize blockchain with database
    let repository = BlockchainRepository::new(pool.clone());
    let blockchain = Blockchain::new(validator_key, repository).await?;
    let blockchain = Arc::new(Mutex::new(blockchain));
    
    // Log system startup
    audit_log::log_system_event(
        "System startup".to_string(),
        "Owami Network testnet server started".to_string(),
        "success".to_string(),
    )?;

    // Initialize rate limiting with configured settings
    let rate_limiter_config = owami_network::rate_limiting::RateLimitingConfig {
        ip_requests_per_minute: config.security.rate_limiting.requests,
        api_requests_per_second: (config.security.rate_limiting.requests as f64 / config.security.rate_limiting.per_seconds as f64) as u32,
        burst_capacity: config.security.rate_limiting.requests,
    };
    let rate_limiter_state = Arc::new(RateLimiterState::new(rate_limiter_config));
    info!("Rate limiting initialized with production configuration");

    // Setup CORS with configured origins
    let cors = if config.security.cors_origins.is_empty() {
        CorsLayer::new()
            .allow_origin(CorsAny)
            .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
            .allow_headers(CorsAny)
    } else {
        let mut cors_layer = CorsLayer::new()
            .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
            .allow_headers(CorsAny);
        
        for origin in &config.security.cors_origins {
            let header_value = HeaderValue::from_str(origin)
                .map_err(|e| {
                    error!("Invalid CORS origin: {} - {}", origin, e);
                    std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid CORS origin")
                })?;
            cors_layer = cors_layer.allow_origin(header_value);
        }
        cors_layer
    };

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
        .layer(axum::middleware::from_fn_with_state(
            rate_limiter_state.clone(),
            rate_limiter_middleware,
        ))
        .layer(axum::middleware::from_fn(ddos_protection_middleware))
        .with_state(blockchain);
    
    let dapp_routes = Router::new()
        // DApp management routes
        .route("/api/dapps", get(dapp_api::list_dapps))
        .route("/api/dapps", post(dapp_api::create_dapp))
        .route("/api/dapps/:id", get(dapp_api::get_dapp))
        // Legacy deploy/call routes
                        .layer(axum::middleware::from_fn_with_state(
            rate_limiter_state.clone(),
            rate_limiter_middleware,
        ))
        .layer(axum::middleware::from_fn(ddos_protection_middleware))
        .with_state(pool.clone());

    let auth_routes = Router::new()
        .route("/api/auth/register", post(auth::register))
        .route("/api/auth/login", post(auth::login))
        .route("/api/auth/profile", get(auth::profile))
        .layer(axum::middleware::from_fn_with_state(
            rate_limiter_state.clone(),
            rate_limiter_middleware,
        ))
        .layer(axum::middleware::from_fn(ddos_protection_middleware))
        .with_state(pool.clone());

    // Health check endpoints
    let health_routes = Router::new()
        .route("/api/health", get(|| async {
            axum::Json(serde_json::json!({
                "status": "healthy",
                "network": "owami-testnet",
                "token": "0x742d35Cc6634C0532925a3b8D4e6D3b6e8d3e8A0"
            }))
        }))
        .route("/health", get(|| async {
            axum::Json(serde_json::json!({
                "status": "healthy",
                "network": "owami-testnet",
                "token": "0x742d35Cc6634C0532925a3b8D4e6D3b6e8d3e8A0"
            }))
        }))
        .route("/status", get(|| async {
            axum::Json(serde_json::json!({
                "status": "operational",
                "version": "1.0.0",
                "timestamp": chrono::Utc::now().to_rfc3339()
            }))
        }));
    
    // Static file serving with index.html support
    let static_dir = ServeDir::new("landing").append_index_html_on_directories(true);

    let app = blockchain_routes
        .merge(dapp_routes)
        .merge(auth_routes)
        .merge(health_routes)
        // Serve static files from landing directory
        .nest_service("/landing", static_dir.clone())
        .nest_service("/", static_dir)
        .layer(cors);

    // Use configured port
    let port = config.server.port;

    // Run the server
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    info!("Server listening on http://0.0.0.0:{}", port);
    
    axum::serve(listener, app).await?;

    Ok(())
}
