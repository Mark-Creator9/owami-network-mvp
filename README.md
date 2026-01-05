# Owami Network

[![Rust Version](https://img.shields.io/badge/Rust-1.90.0-blue.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)]()

## Project Summary

Owami Network is a cutting-edge blockchain platform built in Rust, designed to provide high-performance decentralized applications with enterprise-grade security and scalability. The platform utilizes a Delegated Proof of Stake (DPoS) consensus mechanism and supports WebAssembly (WASM) smart contracts, making it ideal for both developers and enterprises.

### Key Features

- **DPoS Consensus**: Efficient and scalable consensus mechanism with validator delegation
- **WASM Smart Contracts**: High-performance smart contract execution using WebAssembly
- **Multi-layer Architecture**: Separated consensus, execution, and networking layers
- **High Performance**: Optimized for throughput and low latency
- **Enterprise Ready**: Production-grade security and monitoring capabilities

## Language & Runtime

### Rust Configuration

- **Edition**: Rust 2021 edition
- **Version**: Rust 1.90.0
- **Build System**: Cargo

### Core Dependencies

#### Core Blockchain
- `ed25519-dalek` - Ed25519 digital signature scheme for identity and transaction signing
- `blake3` - Fast cryptographic hash function for proof-of-work and data integrity
- `secp256k1` - Elliptic curve cryptography for Bitcoin-style addresses

#### Database
- `rocksdb` v0.21 - High-performance key-value store for blockchain state management

#### Networking
- `libp2p` v0.53 - Modular peer-to-peer networking stack
  - `tcp` - TCP transport for network communication
  - `gossipsub` - Pub/Sub messaging for network gossip
  - `noise` - Secure communication protocol
  - `yamux` - Stream multiplexing for efficient connection usage

#### Web Framework
- `axum` v0.7 - Web framework for HTTP API endpoints
- `tokio` v1.0 - Asynchronous runtime for high-performance networking
- `tower` - Modular building blocks for network services

#### WebAssembly Runtime
- `wasmtime` v20.0 - High-performance WASM runtime for smart contract execution
- `wasmparser` v0.201 - Parser for WebAssembly binary format

#### Security
- `jsonwebtoken` - JWT token handling for authentication
- `bcrypt` - Password hashing and verification
- `governor` - Rate limiting for API protection

#### Serialization
- `serde` - Framework for serializing and deserializing Rust data structures
- `serde_json` - JSON serialization support
- `bincode` - Binary serialization for efficient data storage

## Build & Installation

### Prerequisites

- Rust 1.90.0 or higher
- Cargo package manager
- Git for version control

### Building from Source

```bash
# Clone the repository
git clone https://github.com/owami/owami-network.git
cd owami-network

# Build the project
cargo build --release

# Run tests
cargo test

# Generate documentation
cargo doc --no-deps --open

# Build specific features
cargo build --release --features debug-logging
```

### Feature Flags

- `debug-logging`: Enable detailed debug logging
- `metrics`: Enable Prometheus metrics collection
- `json-rpc`: Enable JSON-RPC API endpoints
- `websocket`: Enable WebSocket connections

### Development Commands

```bash
# Run in development mode
cargo run

# Run with custom config
cargo run -- --config config/development.toml

# Run benchmarks
cargo bench

# Check for clippy warnings
cargo clippy

# Format code
cargo fmt
```

## Docker Configuration

### Dockerfile Structure

```dockerfile
# Multi-stage build for production optimization
FROM debian:bullseye-slim as builder
RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Build stage
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim
COPY --from=builder /app/target/release/owami-node /usr/local/bin/
COPY config/ /app/config/
EXPOSE 8080 8081 8082
```

### Docker Compose Setup

```yaml
# docker-compose.yml
version: '3.8'

services:
  validator-1:
    build: .
    ports:
      - "8080:8080"
      - "8081:8081"
      - "8082:8082"
    volumes:
      - validator1_data:/app/data
    environment:
      - OWAMI_NODE_ID=validator-1
      - OWAMI_CONFIG_PATH=/app/config/validator1.toml
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/health"]
      interval: 30s
      timeout: 10s
      retries: 3

  validator-2:
    build: .
    ports:
      - "8083:8080"
      - "8084:8081"
      - "8085:8082"
    volumes:
      - validator2_data:/app/data
    environment:
      - OWAMI_NODE_ID=validator-2
      - OWAMI_CONFIG_PATH=/app/config/validator2.toml
    depends_on:
      - validator-1

  validator-3:
    build: .
    ports:
      - "8086:8080"
      - "8087:8081"
      - "8088:8082"
    volumes:
      - validator3_data:/app/data
    environment:
      - OWAMI_NODE_ID=validator-3
      - OWAMI_CONFIG_PATH=/app/config/validator3.toml
    depends_on:
      - validator-1

volumes:
  validator1_data:
  validator2_data:
  validator3_data:
```

### Running with Docker

```bash
# Build and start all validators
docker-compose up --build

# Start in detached mode
docker-compose up -d

# View logs
docker-compose logs -f validator-1

# Scale validators
docker-compose up --scale validator-1=2

# Stop and remove
docker-compose down -v
```

## Testing

### Test Framework

The project uses multiple testing frameworks:

- **Cargo test**: Built-in Rust testing framework
- **tokio-test**: Async testing utilities
- **criterion**: Benchmark testing and performance measurement

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test module
cargo test rate_limiting

# Run integration tests
cargo test --test integration

# Run benchmarks
cargo bench

# Run tests with coverage
cargo install cargo-tarpaulin
cargo tarpaulin --out xml
```

### Test Files Structure

```
tests/
├── unit/
│   ├── rate_limiting_tests.rs
│   ├── token_tests.rs
│   ├── consensus_tests.rs
│   └── network_tests.rs
├── integration/
│   ├── api_tests.rs
│   ├── dpos_tests.rs
│   └── wasm_tests.rs
├── benchmarks/
│   ├── throughput_bench.rs
│   └── latency_bench.rs
└── fixtures/
    ├── test_blocks.json
    └── test_transactions.json
```

### Test Examples

```rust
// rate_limiting_tests.rs
use owami_network::network::rate_limiting::RateLimiter;

#[tokio::test]
async fn test_rate_limiter_allows_requests_within_limit() {
    let rate_limiter = RateLimiter::new(100, Duration::from_secs(1));
    
    for i in 0..100 {
        assert!(rate_limiter.check_rate_limit().await.is_ok());
    }
    
    // 101st request should be denied
    assert!(rate_limiter.check_rate_limit().await.is_err());
}

#[tokio::test]
async fn test_rate_limiter_resets_after_window() {
    let rate_limiter = RateLimiter::new(10, Duration::from_millis(100));
    
    // Exhaust the limit
    for _ in 0..10 {
        assert!(rate_limiter.check_rate_limit().await.is_ok());
    }
    
    // Should be rate limited
    assert!(rate_limiter.check_rate_limit().await.is_err());
    
    // Wait for window to reset
    tokio::time::sleep(Duration::from_millis(150)).await;
    
    // Should be allowed again
    assert!(rate_limiter.check_rate_limit().await.is_ok());
}
```

## API Endpoints

### Authentication Endpoints

#### POST /api/v1/auth/login
Authenticate user and receive JWT token.

**Request:**
```json
{
  "username": "user@example.com",
  "password": "secure_password"
}
```

**Response:**
```json
{
  "access_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
  "token_type": "Bearer",
  "expires_in": 3600,
  "user": {
    "id": "user_123",
    "username": "user@example.com",
    "roles": ["user"]
  }
}
```

#### POST /api/v1/auth/register
Register new user account.

**Request:**
```json
{
  "username": "newuser@example.com",
  "password": "secure_password",
  "email": "newuser@example.com"
}
```

### Token Management

#### GET /api/v1/tokens
Retrieve user's token balance.

**Response:**
```json
{
  "balances": {
    "OWM": "1000.50",
    "USDT": "500.00"
  },
  "staked": {
    "OWM": "200.00"
  }
}
```

#### POST /api/v1/tokens/transfer
Transfer tokens to another address.

**Request:**
```json
{
  "to": "owm1qxygfkny8gmynx5m6vd80rz5oy4nLx949qgf5u",
  "amount": "100.00",
  "token": "OWM",
  "memo": "Payment for services"
}
```

#### POST /api/v1/tokens/stake
Stake tokens for validator rewards.

**Request:**
```json
{
  "validator_address": "owm1qxygfkny8gmynx5m6vd80rz5oy4nLx949qgf5u",
  "amount": "500.00"
}
```

### Blockchain Operations

#### GET /api/v1/blockchain/status
Get current blockchain status.

**Response:**
```json
{
  "height": 1234567,
  "hash": "0x1a2b3c4d5e6f7890abcdef1234567890abcdef12",
  "timestamp": "2024-01-15T10:30:00Z",
  "validators": 21,
  "dpos_consensus": "active"
}
```

#### GET /api/v1/blockchain/blocks/{height}
Retrieve block information.

#### GET /api/v1/blockchain/transactions/{hash}
Retrieve transaction details.

#### POST /api/v1/blockchain/broadcast
Broadcast transaction to network.

### DApp Management

#### POST /api/v1/dapps/deploy
Deploy WASM smart contract.

#### GET /api/v1/dapps/{address}
Get DApp information and metadata.

#### POST /api/v1/dapps/{address}/call
Execute smart contract function.

#### GET /api/v1/dapps/{address}/state
Query smart contract state.

### Health Checks

#### GET /health
Basic health check endpoint.

**Response:**
```json
{
  "status": "healthy",
  "timestamp": "2024-01-15T10:30:00Z",
  "uptime": 86400,
  "version": "1.0.0"
}
```

#### GET /health/detailed
Detailed health check with system metrics.

**Response:**
```json
{
  "status": "healthy",
  "components": {
    "database": "healthy",
    "network": "healthy",
    "consensus": "healthy",
    "wasm_runtime": "healthy"
  },
  "metrics": {
    "cpu_usage": 45.2,
    "memory_usage": 1024.5,
    "disk_usage": 5120.0,
    "network_connections": 42
  }
}
```

## Configuration

### Configuration Files

The project uses TOML-based configuration files for different environments.

#### Development Configuration (config/development.toml)

```toml
# Server configuration
[server]
host = "127.0.0.1"
port = 8080
workers = 4
max_connections = 1000

# Database configuration
[database]
backend = "rocksdb"
path = "./data/dev"
cache_size = 100
compression = "lz4"

# Network configuration
[network]
p2p_port = 8081
public_address = "127.0.0.1:8081"
bootstrap_nodes = [
  "/ip4/127.0.0.1/tcp/8081/p2p/QmYyQSo1c1Ym7orWxLYvCrM2EmxFTANf8wXmmE7DWjhx5N"
]
discovery_enabled = true

# Consensus configuration
[consensus]
consensus_type = "dpos"
min_validators = 3
max_validators = 21
epoch_length = 100
unbonding_period = 21

# WASM configuration
[wasm]
runtime = "wasmtime"
cache_size = "100MB"
gas_limit = 1000000

# Security configuration
[security]
jwt_secret = "your-jwt-secret-here"
bcrypt_rounds = 12
rate_limit_requests = 100
rate_limit_window = 60

# Logging configuration
[logging]
level = "debug"
format = "json"
output = "stdout"
```

#### Production Configuration (config/production.toml)

```toml
# Server configuration
[server]
host = "0.0.0.0"
port = 8080
workers = 16
max_connections = 10000
keep_alive_timeout = 65

# Database configuration
[database]
backend = "rocksdb"
path = "/app/data/prod"
cache_size = 1024
compression = "zstd"
write_buffer_size = 64
max_background_jobs = 8

# Network configuration
[network]
p2p_port = 8081
public_address = "your-public-ip:8081"
bootstrap_nodes = [
  "/ip4/bootstrap1.owami.network/tcp/8081/p2p/..."
]
discovery_enabled = true
max_peers = 50
connection_timeout = 30

# Consensus configuration
[consensus]
consensus_type = "dpos"
min_validators = 7
max_validators = 21
epoch_length = 100
unbonding_period = 21
proposer_selection = "round_robin"

# WASM configuration
[wasm]
runtime = "wasmtime"
cache_size = "1GB"
gas_limit = 10000000
optimization_level = "speed"

# Security configuration
[security]
jwt_secret = "${JWT_SECRET}"
bcrypt_rounds = 12
rate_limit_requests = 1000
rate_limit_window = 60
max_request_size = "10MB"

# Monitoring configuration
[monitoring]
metrics_enabled = true
metrics_port = 9090
tracing_enabled = true
health_check_enabled = true

# Logging configuration
[logging]
level = "info"
format = "json"
output = "file"
log_file = "/var/log/owami/node.log"
rotation = "daily"
retention = "30d"
```

### Environment Variables

| Variable | Description | Required | Default |
|----------|-------------|----------|---------|
| `OWAMI_NODE_ID` | Unique node identifier | Yes | - |
| `OWAMI_CONFIG_PATH` | Path to configuration file | Yes | `config/development.toml` |
| `JWT_SECRET` | JWT signing secret | Yes | - |
| `OWAMI_PRIVATE_KEY` | Node private key for consensus | Yes | - |
| `OWAMI_PUBLIC_KEY` | Node public key for discovery | Yes | - |
| `RUST_LOG` | Logging level override | No | `info` |
| `OWAMI_DATA_DIR` | Data directory path | No | `./data` |
| `OWAMI_METRICS_PORT` | Metrics collection port | No | `9090` |

## Main Entry Points & Key Modules

### Project Structure

```
owami-network/
├── Cargo.toml
├── README.md
├── CHANGELOG.md
├── LICENSE
├── src/
│   ├── main.rs                 # Main application entry point
│   ├── lib.rs                  # Library root with shared types
│   ├── config/                 # Configuration management
│   │   ├── mod.rs
│   │   ├── server.rs
│   │   ├── database.rs
│   │   ├── network.rs
│   │   └── consensus.rs
│   ├── blockchain/             # Core blockchain logic
│   │   ├── mod.rs
│   │   ├── block.rs
│   │   ├── transaction.rs
│   │   ├── merkle_tree.rs
│   │   └── state.rs
│   ├── consensus/              # DPoS consensus mechanism
│   │   ├── mod.rs
│   │   ├── dpos.rs
│   │   ├── validator.rs
│   │   ├── proposer.rs
│   │   └── voting.rs
│   ├── network/                # P2P networking layer
│   │   ├── mod.rs
│   │   ├── p2p.rs
│   │   ├── discovery.rs
│   │   ├── gossipsub.rs
│   │   └── rate_limiting.rs
│   ├── database/               # Database abstraction
│   │   ├── mod.rs
│   │   ├── rocksdb.rs
│   │   ├── batch.rs
│   │   └── migration.rs
│   ├── api/                    # HTTP API layer
│   │   ├── mod.rs
│   │   ├── auth.rs
│   │   ├── tokens.rs
│   │   ├── blockchain.rs
│   │   └── dapps.rs
│   ├── wasm/                   # WASM runtime integration
│   │   ├── mod.rs
│   │   ├── runtime.rs
│   │   ├── gas.rs
│   │   └── host_functions.rs
│   ├── crypto/                 # Cryptographic operations
│   │   ├── mod.rs
│   │   ├── signature.rs
│   │   ├── hash.rs
│   │   └── address.rs
│   ├── security/               # Security and authentication
│   │   ├── mod.rs
│   │   ├── jwt.rs
│   │   ├── password.rs
│   │   └── rate_limiter.rs
│   ├── monitoring/             # Metrics and health checks
│   │   ├── mod.rs
│   │   ├── metrics.rs
│   │   ├── health.rs
│   │   └── tracing.rs
│   └── utils/                  # Utility functions
│       ├── mod.rs
│       ├── logger.rs
│       ├── time.rs
│       └── encoding.rs
├── tests/                      # Test suite
├── config/                     # Configuration files
├── docs/                       # Additional documentation
├── scripts/                    # Build and deployment scripts
└── docker/                     # Docker configuration
```

### Main Entry Points

#### main.rs - Application Entry Point

```rust
use owami_network::{Config, Node, Result};
use tokio::main;
use tracing::{info, error};

#[main]
async fn main() -> Result<()> {
    // Initialize configuration
    let config = Config::from_env()?;
    
    // Initialize logging
    owami_network::utils::logger::init(&config.logging)?;
    
    info!("Starting Owami Network node...");
    info!("Node ID: {}", config.node_id);
    info!("Version: {}", env!("CARGO_PKG_VERSION"));
    
    // Create and start node
    let node = Node::new(config).await?;
    node.start().await?;
    
    Ok(())
}
```

### Key Module Descriptions

#### blockchain Module
- **block.rs**: Block structure and validation logic
- **transaction.rs**: Transaction types and serialization
- **merkle_tree.rs**: Merkle tree implementation for state commitments
- **state.rs**: Global state management and updates

#### consensus Module
- **dpos.rs**: Delegated Proof of Stake consensus implementation
- **validator.rs**: Validator management and delegation handling
- **proposer.rs**: Block proposer selection and rotation
- **voting.rs**: Finalization and voting mechanisms

#### network Module
- **p2p.rs**: Peer-to-peer networking using libp2p
- **discovery.rs**: Peer discovery and connection management
- **gossipsub.rs**: Message propagation and subscription
- **rate_limiting.rs**: Request rate limiting for API protection

#### wasm Module
- **runtime.rs**: WASM execution environment management
- **gas.rs**: Gas accounting and metering for smart contracts
- **host_functions.rs**: Native functions available to WASM contracts

#### api Module
- **auth.rs**: Authentication and authorization endpoints
- **tokens.rs**: Token management and transfer operations
- **blockchain.rs**: Blockchain data access endpoints
- **dapps.rs**: DApp deployment and interaction endpoints

## Release Profile & Optimization

### Cargo Release Profile

```toml
# Cargo.toml excerpt
[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
panic = "abort"
strip = true
debug = false

[profile.release.package."*"]
opt-level = 3

# Additional optimization for WASM
[profile.release-wasm]
inherits = "release"
opt-level = "z"
lto = true
panic = "abort"
codegen-units = 1
```

### Performance Optimizations

#### Link-Time Optimization (LTO)
- Full LTO enabled for maximum performance
- Single codegen unit for better optimization
- Dead code elimination reduces binary size

#### Binary Size Optimization
- Symbols stripped in release builds
- Panic strategy set to `abort` for smaller binaries
- Optional features compiled out in production

#### Runtime Optimizations
- RocksDB tuned for production workloads
- Async runtime optimized for I/O bound operations
- Memory allocation strategies optimized
- Cache-friendly data structures

### Production Deployment Settings

```toml
# Release optimization settings
[profile.production]
opt-level = 3
lto = "fat"
codegen-units = 1
panic = "abort"
strip = true
debug = false

# LLVM optimizations
rustflags = [
    "-C", "target-cpu=native",
    "-C", "target-feature=+sse4.2,+avx2",
    "-C", "force-frame-pointers"
]
```

### Monitoring & Observability

- Prometheus metrics enabled by default
- Structured logging with configurable levels
- Health check endpoints for service monitoring
- Distributed tracing for performance analysis

### Security Hardening
- Memory-safe Rust implementation
- Constant-time cryptographic operations
- Rate limiting and DDoS protection
- Secure JWT token handling
- Input validation and sanitization

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

## Support

- Documentation: [docs.owami.network](https://docs.owami.network)
- Community Forum: [forum.owami.network](https://forum.owami.network)
- Bug Reports: [GitHub Issues](https://github.com/owami/owami-network/issues)
- Security Issues: security@owami.network

---

**Owami Network** - Building the future of decentralized applications 🌍