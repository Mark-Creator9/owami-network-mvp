# OWami Network Simplified

A simplified version of the OWami Network blockchain platform, focusing on core token and DApp functionality with a clean REST API.

## 🚀 Quick Start

### Prerequisites
- **Rust** (latest stable)
- **PostgreSQL** (12+)
- **PowerShell** (for Windows scripts)

### One-Command Launch

```powershell
.\launch_testnet.ps1
```

This script will:
- ✅ Check PostgreSQL status
- ✅ Create database `owami_testnet`
- ✅ Run migrations
- ✅ Build the project
- ✅ Start the server on http://localhost:3000

### Manual Setup

1. **Install dependencies:**
```bash
cargo build --release
```

2. **Set up database:**
```bash
createdb owami_testnet
export DATABASE_URL="postgres://postgres:postgres@localhost/owami_testnet"
```

3. **Run migrations:**
```bash
sqlx migrate run
```

4. **Start the server:**
```bash
cargo run --bin owami-network-simplified
```

## 📡 API Endpoints

### Token API
- `GET /api/token/info` - Get token information
- `GET /api/token/balance/:address` - Get balance for address
- `POST /api/token/transfer` - Transfer tokens
- `POST /api/token/mint` - Mint new tokens
- `POST /api/token/approve` - Approve spender
- `GET /api/token/transactions` - Get transaction history

### DApp API
- `POST /api/dapp` - Create new DApp
- `GET /api/dapp/user/:address` - Get user's DApps
- `GET /api/dapp/:id` - Get specific DApp
- `POST /api/dapp/:id/state` - Update DApp state
- `GET /api/dapp/:id/state/:key` - Get DApp state value

### Frontend
- `GET /landing` - Web-based frontend interface

## 🧪 Testing

Run the comprehensive API test suite:

```powershell
.\test_api_endpoints.ps1
```

## 📊 Database Schema

### Tables
- `token_balances` - Token balances by address
- `token_transactions` - All token transfers
- `token_approvals` - Token spending approvals
- `dapps` - Registered DApps
- `dapp_states` - DApp state storage

## 🔧 Configuration

### Environment Variables
- `DATABASE_URL` - PostgreSQL connection string
- `PORT` - Server port (default: 3000)

### Example `.env` file:
```bash
DATABASE_URL=postgres://user:password@localhost/owami_testnet
PORT=3000
```

## 🏗️ Development

### Project Structure
```
owami-network/
├── src/
│   ├── api_simplified/    # REST API endpoints
│   ├── models.rs          # Database models
│   └── main.rs           # Server entry point
├── migrations/           # Database migrations
├── landing/             # Web frontend
├── wasm_examples/       # WebAssembly examples
└── scripts/            # Utility scripts
```

### Adding New Features
1. Add database migration in `migrations/`
2. Create model in `src/models.rs`
3. Add API endpoints in `src/api_simplified/`
4. Update routes in `src/main.rs`

## 🌐 Frontend Usage

The included frontend provides:
- **Token Dashboard** - View balances and transactions
- **DApp Explorer** - Browse and interact with DApps
- **Developer Tools** - Test API endpoints
- **Real-time Updates** - Live balance and transaction updates

Access at: http://localhost:3000/landing

## 🔒 Security Notes

- This is a **testnet** implementation for development
- No real funds or production data
- CORS is enabled for development
- Authentication is simplified for testing

## 📞 Support

For issues or questions:
1. Check the logs in the terminal
2. Verify PostgreSQL is running
3. Ensure all dependencies are installed
4. Run the test suite to verify setup

## 🎯 Next Steps

- [ ] Add WebAssembly smart contract support
- [ ] Implement proper authentication
- [ ] Add WebSocket support for real-time updates
- [ ] Create mobile wallet app
- [ ] Add multi-token support
