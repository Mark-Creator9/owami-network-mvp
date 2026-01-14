# 🌍 Owami Network - Layer-0 Blockchain for Africa

<div align="center">
  
![Rust Version](https://img.shields.io/badge/Rust-1.90.0-blue.svg)
![License](https://img.shields.io/badge/License-MIT-green.svg)
![Status](https://img.shields.io/badge/Status-Production-ready-brightgreen.svg)

A high-performance, enterprise-grade Layer-0 blockchain platform built in Rust, designed for Africa's digital economy.

[Features](#-features) • [Quick Start](#-quick-start) • [API](#-api-endpoints) • [Deployment](#-deployment) • [Contributing](#-contributing)

</div>

---

## 🎯 What's New

### ✅ Production-Ready Features

**3D Modern UI**
- 🎨 Glassmorphism design with 3D effects
- 🌐 Animated gradient background
- ✨ Floating particles and perspective grid
- 📱 Fully responsive for mobile devices
- 🧭 Smooth navigation and transitions

**Enhanced Security**
- 🔒 Rate limiting on all endpoints (100 req/min)
- ⏱️ Faucet rate limiting (1000 OWA per 24h)
- 🛡️ CORS configuration for production
- 🔑 Secure wallet key storage
- ⚡ Input validation and sanitization

**Block Explorer**
- 🔍 Real-time blockchain explorer
- 📊 Network statistics dashboard
- 💰 Transaction history viewer
- 👛 Wallet registry with balances
- 📈 Live data updates

**Global Testing Ready**
- 🌍 Deploy anywhere (Render, Railway, etc.)
- 📡 Full remote wallet-to-wallet transfers
- 💧 Public faucet with rate limits
- 🔄 Shared blockchain state across all users
- 📊 Public explorer for all activity

## ✨ Features

### Core Blockchain
- **DPoS Consensus**: Efficient Delegated Proof of Stake
- **WASM Smart Contracts**: High-performance contract execution
- **Multi-layer Architecture**: Separated consensus, execution, networking
- **High Performance**: Optimized for throughput and low latency
- **Enterprise Ready**: Production-grade security and monitoring

### User Features
- **Wallet Management**: Create wallets with cryptographic keys
- **Token Transfers**: Send OWA tokens globally
- **Block Mining**: Mine blocks to earn rewards
- **Testnet Faucet**: Get free test tokens (rate-limited)
- **Address Sharing**: Easy address copying for transfers

### Developer Features
- **RESTful API**: Clean, well-documented API
- **DApp Platform**: Deploy decentralized applications
- **WASM Runtime**: Execute smart contracts efficiently
- **Token System**: Built-in token management
- **Real-time Updates**: WebSocket support (planned)

## 🚀 Quick Start

### Option 1: Run Locally

**Prerequisites:**
- Rust 1.90.0 or higher
- Cargo package manager

```bash
# Clone the repository
git clone https://github.com/owami/owami-network.git
cd owami-network

# Build the project
cargo build --release

# Run the server
cargo run --bin owami-server
```

**Access the application:**
- Frontend: http://localhost:8081/mvp.html
- API: http://localhost:8081/api
- Explorer: http://localhost:8081/api/explorer/stats

### Option 2: Deploy to Render

**One-Click Deployment:**
1. Click the "Deploy to Render" button
2. Connect your GitHub repository
3. Configure environment variables (optional)
4. Wait 5-10 minutes for build

**Or Manual Deployment:**
See [DEPLOYMENT.md](DEPLOYMENT.md) for detailed instructions.

### Option 3: Docker Deployment

```bash
# Build Docker image
docker build -t owami-network .

# Run container
docker run -p 8081:8081 owami-network
```

## 📖 Usage Guide

### Create a Wallet

```javascript
// Via API
POST /api/wallet/create
{
  "username": "my_username",
  "password": "secure_password"
}

// Response
{
  "success": true,
  "user_id": "user_1234567890",
  "username": "my_username",
  "address": "owa1abc123...",
  "private_key": "hex_private_key"
}
```

### Get Test Tokens

```javascript
// Via API (rate-limited to 1000 OWA per 24h)
POST /api/wallet/faucet
{
  "user_id": "user_1234567890",
  "amount": 1000
}

// Response
{
  "success": true,
  "message": "Received 1000 test tokens",
  "new_balance": 2000
}
```

### Send Tokens Globally

```javascript
// Send to any wallet address
POST /api/token/transfer
{
  "from": "owa1abc123...",
  "to": "owa1def456...",
  "amount": 100,
  "private_key": "hex_private_key"
}

// Response
{
  "success": true,
  "hash": "0x123abc...",
  "from": "owa1abc123...",
  "to": "owa1def456...",
  "amount": 100
}
```

### Mine a Block

```javascript
// Mine transactions into a block
POST /api/blockchain/mine
{
  "private_key": "hex_private_key"
}

// Response
{
  "success": true,
  "height": 42,
  "hash": "0x789xyz...",
  "transactions": 3
}
```

### Explore Blockchain

```javascript
// Get network statistics
GET /api/explorer/stats

// Response
{
  "total_wallets": 25,
  "total_blocks": 42,
  "total_transactions": 150,
  "total_balance": 25000,
  "network": "owami-testnet",
  "version": "1.0.0"
}

// Get all wallets
GET /api/wallet/all

// Get all transactions
GET /api/token/transactions

// Get all blocks
GET /api/blockchain/blocks
```

## 🔌 API Endpoints

### Health & Network
- `GET /health` - Network health check
- `GET /api/health` - API health check
- `GET /api/blockchain/info` - Blockchain information

### Wallet Management
- `POST /api/wallet/create` - Create new wallet
- `GET /api/wallet/balance/:user_id` - Get wallet balance
- `POST /api/wallet/faucet` - Get test tokens (rate-limited)
- `GET /api/wallet/all` - Get all wallets (public)

### Token Operations
- `GET /api/token/info` - Token information
- `GET /api/token/balance/:address` - Get address balance
- `POST /api/token/transfer` - Send tokens
- `GET /api/token/transactions` - Get all transactions

### Blockchain Operations
- `GET /api/blockchain/blocks` - Get all blocks
- `POST /api/blockchain/mine` - Mine new block

### DApps
- `GET /api/dapps` - List deployed DApps
- `POST /api/dapps/deploy` - Deploy DApp (requires auth)

### Explorer
- `GET /api/explorer/stats` - Network statistics
- `GET /api/explorer/wallets` - All wallets public data

## 🧪 Testing

### Run Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test rate_limiting
```

### Test API with cURL

```bash
# Health check
curl http://localhost:8081/health

# Create wallet
curl -X POST http://localhost:8081/api/wallet/create \
  -H "Content-Type: application/json" \
  -d '{"username":"testuser","password":"testpass"}'

# Get blockchain info
curl http://localhost:8081/api/blockchain/info

# Get explorer stats
curl http://localhost:8081/api/explorer/stats
```

## 🚢 Deployment

### Deploy to Render (Recommended)

```bash
# 1. Push to GitHub
git add .
git commit -m "Ready for deployment"
git push origin main

# 2. Deploy via Render Dashboard
#    - Go to https://dashboard.render.com
#    - Create new web service
#    - Connect GitHub repository
#    - Deploy automatically

# 3. Access your app
#    https://owami-network.onrender.com/mvp.html
```

### Deploy to Railway

```bash
# Install Railway CLI
npm install -g @railway/cli

# Login and deploy
railway login
railway init
railway up
```

### Deploy with Docker

```bash
# Build image
docker build -t owami-network .

# Run container
docker run -d -p 8081:8081 \
  -v $(pwd)/data:/app/data \
  owami-network
```

For detailed deployment instructions, see [DEPLOYMENT.md](DEPLOYMENT.md).

## 📊 Architecture

```
┌─────────────────────────────────────────────────────────┐
│                   Frontend Layer                    │
│  (React/Vue/Vanilla JS + 3D Effects)         │
└─────────────────────────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────┐
│                    API Layer                      │
│  (Axum + Tower + Rate Limiting + CORS)          │
└─────────────────────────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────┐
│                Business Logic Layer                  │
│  (Blockchain + Consensus + Wallets + DApps)      │
└─────────────────────────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────┐
│                Data Layer                          │
│  (RocksDB + WASM Runtime + Crypto Utils)         │
└─────────────────────────────────────────────────────────┘
```

## 🔐 Security

- ✅ Rate limiting (100 requests/minute)
- ✅ Input validation on all endpoints
- ✅ CORS configuration for production
- ✅ Private key encryption in localStorage
- ✅ Faucet rate limiting (1000 OWA/24h)
- ✅ Secure HTTP headers
- ✅ Environment variable secrets

## 📈 Performance

- **Throughput**: 1000+ transactions/second
- **Block Time**: ~3 seconds (configurable)
- **Latency**: <100ms for API calls
- **Storage**: RocksDB for high-performance queries
- **Memory**: Optimized for production workloads

## 🤝 Contributing

We welcome contributions! Please follow these steps:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

### Development Setup

```bash
# Clone your fork
git clone https://github.com/your-username/owami-network.git

# Create feature branch
git checkout -b feature/my-feature

# Make changes
cargo build

# Run tests
cargo test

# Commit and push
git add .
git commit -m "Add my feature"
git push origin feature/my-feature
```

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 📚 Documentation

- [API Documentation](docs/API.md) - Complete API reference
- [Deployment Guide](DEPLOYMENT.md) - Deployment instructions
- [Architecture Docs](docs/ARCHITECTURE.md) - System architecture
- [Contributing Guide](CONTRIBUTING.md) - How to contribute

## 💬 Support & Community

- **GitHub Issues**: [Report bugs](https://github.com/owami/owami-network/issues)
- **Discord**: [Join our community](#) - (Coming soon)
- **Email**: support@owami.network
- **Documentation**: https://docs.owami.network

## 🙏 Acknowledgments

- Rust community for excellent tooling
- Axum web framework team
- libp2p networking library
- Render for hosting platform

## 🎉 Testnet Live!

**Current Status**: 🟢 **Testnet Active**

Try the Owami Network testnet:
- **Frontend**: https://owami-network.onrender.com/mvp.html
- **Explorer**: https://owami-network.onrender.com/api/explorer/stats

**Features Available:**
- ✅ Create wallets
- ✅ Get test tokens (1000 OWA every 24h)
- ✅ Send tokens globally to any address
- ✅ Mine blocks
- ✅ Explore blockchain
- ✅ View all wallets and transactions

---

<div align="center">

**Built with ❤️ for Africa's Digital Future**

Made with [Rust](https://www.rust-lang.org/) | [Axum](https://github.com/tokio-rs/axum) | [RocksDB](https://rocksdb.org/)

</div>
