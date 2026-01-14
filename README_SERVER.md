# Owami Network - Fully Functional Blockchain Project

A layer-0 blockchain platform for Africa's digital economy with integrated frontend and backend.

## Quick Start

### Windows (Start Server)
```bash
start-server.bat
```

### Linux/Mac (Start Server)
```bash
cargo run --bin owami-server
```

The server will start at:
- **Frontend**: http://localhost:8081
- **API**: http://localhost:8081/api

## Features

### ✅ Working Features

1. **Wallet Management**
   - Create wallets with Ed25519 cryptography
   - Get wallet addresses and private keys
   - Auto-save wallet to localStorage

2. **Token Operations**
   - Get OWA test tokens from faucet (1000 tokens)
   - Transfer tokens between wallets
   - Check wallet balance

3. **Blockchain Operations**
   - Create/mine new blocks
   - View blockchain information
   - List recent blocks and transactions

4. **Frontend**
   - Professional MVP interface at landing/mvp.html
   - Real-time updates every 10-30 seconds
   - Responsive design with toast notifications

5. **API Endpoints**

```
Health & Status:
  GET  /health              - Server health check
  GET  /api/health          - Detailed health info
  GET  /api/blockchain/info  - Blockchain information

Authentication:
  POST /api/auth/register   - Create wallet (username, password)
  POST /api/auth/login      - Login (demo mode)

Wallet Operations:
  POST /api/wallet/create      - Create new wallet
  GET  /api/wallet/balance/:id  - Get wallet balance
  POST /api/wallet/faucet      - Get test tokens

Token Operations:
  GET  /api/token/info                - Token information
  GET  /api/token/balance/:address   - Get token balance
  POST /api/token/transfer             - Transfer tokens
  GET  /api/token/transactions       - Transaction history

Blockchain:
  GET  /api/blockchain/blocks  - List all blocks
  POST /api/blockchain/mine    - Mine a new block

DApps:
  GET  /api/dapps           - List deployed DApps
  POST /api/dapps/create    - Create new DApp
```

## Project Structure

```
owami-network/
├── src/
│   ├── main_simple_enhanced.rs    # Main server file
│   ├── blockchain.rs               # Core blockchain logic
│   ├── wallet.rs                  # Wallet cryptography
│   ├── crypto_utils.rs            # Crypto utilities
│   └── api/                      # API modules
│       ├── token.rs              # Token endpoints
│       ├── auth.rs               # Auth endpoints
│       └── dapp.rs               # DApp endpoints
├── landing/
│   ├── mvp.html                  # Frontend UI
│   ├── index.html                # Alternative landing
│   └── js/
│       └── app.js                # Frontend JavaScript
├── Cargo.toml                       # Rust dependencies
├── start-server.bat                 # Windows startup script
└── data/                           # Blockchain data
```

## Usage Examples

### Create Wallet and Get Tokens

1. Open http://localhost:8081 in your browser
2. Click "Create Wallet"
3. Wait for wallet to be created
4. Click "Get 1000 OWA" to receive test tokens

### Send Tokens

1. Make sure you have tokens in your wallet
2. Enter recipient wallet address
3. Enter amount to send
4. Click "Send Globally"

### Mine Blocks

1. Create some transactions first (send tokens)
2. Click "Mine Block"
3. Wait for mining to complete
4. View mined blocks in "Recent Blocks" section

### API Testing

```bash
# Health check
curl http://localhost:8081/health

# Create wallet
curl -X POST http://localhost:8081/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username":"testuser","password":"password"}'

# Get blockchain info
curl http://localhost:8081/api/blockchain/info

# Get token info
curl http://localhost:8081/api/token/info

# List blocks
curl http://localhost:8081/api/blockchain/blocks
```

## Technical Details

- **Backend**: Rust with Axum framework
- **Frontend**: Vanilla HTML/JS/CSS
- **Database**: RocksDB for blockchain storage
- **Cryptography**: Ed25519 for signing
- **Hashing**: BLAKE3 for block hashing
- **Port**: 8081 (configurable via PORT env var)

## Troubleshooting

**Port already in use?**
- Change port: `set PORT=8082`
- Kill existing process on Windows: `taskkill /F /IM owami-server.exe`

**Build errors?**
- Clean build: `cargo clean`
- Rebuild: `cargo build --bin owami-server`

**Frontend not loading?**
- Ensure `landing/mvp.html` exists
- Check browser console for errors
- Verify server is running

## Development

```bash
# Run with debug logging
RUST_LOG=debug cargo run --bin owami-server

# Run tests
cargo test

# Build release version
cargo build --release --bin owami-server
```

## License

See LICENSE file for details.
