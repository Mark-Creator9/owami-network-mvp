# Owami Network - Production Ready MVP

A high-performance blockchain platform built in Rust, featuring a complete REST API for token operations, DApp management, and blockchain functionality. Designed for global accessibility and enterprise-grade deployment.

## 📚 Documentation Portal
- Start here: `docs/OVERVIEW.md`
- Quickstart: `docs/QUICKSTART.md`
- API Guide: `docs/API_GUIDE.md`
- Testnet Guide: `docs/TESTNET_GUIDE.md`
- Troubleshooting: `docs/TROUBLESHOOTING.md`
- Postman: `postman/Owami.postman_collection.json` + `postman/Owami.local_environment.json`

## 🚀 Quick Start

### Prerequisites
- **Rust** (1.70+)
- **PostgreSQL** (Aiven cloud recommended)
- **PowerShell 7+** (for Windows scripts)

### Recommended: Aiven Cloud Launch
```powershell
$env:AIVEN_PASSWORD = "YOUR_AIVEN_PASSWORD"
./launch_aiven_demo.ps1 -AivenPassword $env:AIVEN_PASSWORD -OpenBrowser
```

### Alternative: Manual Setup
1. **Install dependencies:**
```bash
cargo build --release
```

2. **Set up database:**
```bash
# Provide a valid DATABASE_URL (local or external)
export DATABASE_URL="postgres://username:password@host:port/db?sslmode=require"
```

3. **Run migrations:**
```bash
sqlx migrate run
```

4. **Start the production server:**
```bash
./start_server.bat
```

## 📡 API Endpoints

### Authentication API
- `POST /api/auth/register` - User registration
- `POST /api/auth/login` - User login
- `GET /api/auth/profile` - Get user profile

### Token API
- `GET /api/token/info` - Get token information
- `GET /api/token/balance/:address` - Get balance for address
- `POST /api/token/transfer` - Transfer tokens
- `POST /api/token/mint/:address` - Mint tokens to address
- `POST /api/token/mint` - Mint tokens (JSON payload)
- `GET /api/token/transactions` - Get transaction history

### Blockchain API
- `GET /api/blockchain/info` - Get blockchain information
- `GET /api/blockchain/blocks` - Get block list
- `POST /api/blockchain/mine` - Mine new block

### DApp API
- `GET /api/dapps` - List all DApps
- `POST /api/dapps` - Create new DApp
- `GET /api/dapps/:id` - Get specific DApp

### Health & Status
- `GET /api/health` - Health check endpoint
- `GET /status` - System status
- `GET /landing` - Web frontend interface

## 🏗️ Project Structure

```
owami-network/
├── src/
│   ├── api/              # REST API endpoints
│   │   ├── auth.rs       # Authentication handlers
│   │   ├── blockchain.rs # Blockchain operations
│   │   ├── dapp.rs       # DApp management
│   │   ├── token.rs      # Token operations
│   │   └── mod.rs        # API module exports
│   ├── blockchain.rs     # Core blockchain logic
│   ├── block.rs          # Block structure and operations
│   ├── transaction.rs    # Transaction handling
│   ├── db.rs            # Database repository
│   ├── models.rs        # Data models
│   ├── config.rs        # Configuration management
│   ├── key_management.rs # Cryptographic key handling
│   ├── audit_log.rs     # Audit logging system
│   ├── rate_limiting.rs # Rate limiting middleware
│   ├── wallet.rs        # Wallet functionality
│   └── main.rs          # Server entry point
├── migrations/          # Database migrations
├── landing/            # Web frontend (HTML/CSS/JS)
├── config/             # Configuration files
├── keys/              # Cryptographic keys
├── logs/              # Application logs
└── tests/             # Test suites
```

## 🔧 Configuration

### Environment Variables
- `DATABASE_URL` - PostgreSQL connection string
- `PORT` - Server port (default: 3000)
- `RUST_LOG` - Logging level (default: info)

### Example `.env` file:
```bash
DATABASE_URL=postgres://user:password@localhost/owami_production
PORT=3000
RUST_LOG=info
```

### Production Configuration
The system includes production-ready configuration in `config/production.toml` with:
- Optimized database connection pooling
- Enhanced security settings
- Rate limiting configurations
- CORS origins for production deployment

## 🛡️ Security Features

- **ED25519 Cryptography** for all transactions
- **JWT Authentication** with secure token management
- **Rate Limiting** per endpoint and IP address
- **DDoS Protection** middleware
- **Audit Logging** for all operations
- **CORS Configuration** for secure cross-origin requests

## 📊 Database Schema

### Core Tables
- `blocks` - Blockchain blocks storage
- `transactions` - Transaction records
- `token_balances` - Token balances by address
- `dapps` - DApp registrations
- `users` - User accounts and authentication

### Migration Management
Database migrations are managed through SQLx with versioned SQL files in the `migrations/` directory.

## 🧪 Testing

### Windows Command Prompt (cmd.exe)
```bat
cd C:\Users\HP\Desktop\projects\owami-network
cargo test
```

- Run a specific test file:
```bat
cargo test --test rate_limiting_tests
```

- Verbose output:
```bat
cargo test -v
```

- Notes:
  - Warnings about `cfg(FALSE)` are expected and safe; they indicate disabled internal test modules.
  - Unit/integration tests do not require a running database. The server runtime does require a valid `DATABASE_URL` and migrations.

### PowerShell
```powershell
cd C:\Users\HP\Desktop\projects\owami-network
cargo test
```

### Test Modules
- `tests/rate_limiting_tests.rs` — Rate limiting and DDoS middleware behavior
- `tests/token_tests.rs` — Token and vesting basic operations

## 🚀 Deployment

### Production Deployment
The project includes `render.yaml` for easy deployment to Render.com, but can be deployed to any cloud platform supporting Rust and PostgreSQL.

### Kubernetes (Planned)
- Kubernetes manifests for orchestration (TBD)
- Cloud formation templates for AWS/Azure (TBD)

## 🔍 Monitoring & Logging

- **Structured Logging** with tracing and tracing-subscriber
- **Audit Trail** for all financial operations
- **Health Endpoints** for monitoring
- **Performance Metrics** via Prometheus (planned)

## 🎯 Current Features

✅ **Core Blockchain** - Functional blockchain with mining and transactions
✅ **Token System** - Complete token operations (transfer, mint, balance)
✅ **REST API** - Comprehensive API with authentication
✅ **Database Integration** - PostgreSQL with proper migrations
✅ **Security** - Rate limiting, audit logging, cryptographic security
✅ **Frontend** - Web-based interface for testing and demonstration

## 📈 Next Steps

- [ ] **Multi-node Deployment** - Distributed validator network
- [ ] **Performance Optimization** - Caching and query optimization
- [ ] **Enhanced Monitoring** - Prometheus/Grafana integration
- [ ] **API Documentation** - Swagger/OpenAPI specification
- [ ] **Mobile SDK** - Client libraries for mobile applications
- [ ] **Enterprise Features** - Advanced compliance and reporting

## 📞 Support

For issues or questions:
1. Check the terminal logs for error messages
2. Verify PostgreSQL is running and accessible
3. Ensure all environment variables are set correctly
4. Run the test suite to validate setup: `cargo test`

## 🌐 Vision

Owami Network is building a globally accessible blockchain ecosystem with initial focus on core functionality and scalability. The platform is designed to support various use cases including financial services, supply chain tracking, and digital identity, with future plans for specialized African market integrations.

**Current Status**: Production-ready MVP with all core functionality operational and tested.