# Owami Network Testnet Launch Guide
**Author: Mark Creator9**

## Overview
The Owami Network Testnet is now ready for deployment. This guide provides step-by-step instructions for launching and testing the testnet environment.

## Quick Start

### Prerequisites
- Rust 1.70+ installed
- PostgreSQL 12+ running locally
- PowerShell (Windows) or Bash (Linux/Mac)

### 1. Database Setup
```bash
# Create database
createdb owami_testnet

# Set environment variable
export DATABASE_URL=postgres://localhost/owami_testnet
```

### 2. Launch Testnet (Windows PowerShell)
```powershell
# Navigate to project directory
cd c:\Users\HP\Desktop\owami-network

# Run setup script
.\scripts\setup_testnet.ps1
```

### 3. Launch Testnet (Linux/Mac)
```bash
# Navigate to project directory
cd /path/to/owami-network

# Set environment variables
export DATABASE_URL=postgres://localhost/owami_testnet
export PORT=3000

# Run migrations
sqlx migrate run

# Build and run
cargo build --release --bin owami-network-simplified
cargo run --bin owami-network-simplified
```

## API Endpoints

### Blockchain
- `GET /api/blockchain/info` - Get blockchain statistics
- `GET /api/blockchain/blocks` - Get all blocks
- `POST /api/blockchain/mine` - Mine a new block

### Token Management
- `GET /api/token/balance/:address` - Get token balance for address
- `POST /api/token/transfer` - Transfer tokens between addresses
- `POST /api/token/mint/:address` - Mint new tokens to address

### DApp Management
- `GET /api/dapps` - List all DApps
- `POST /api/dapps` - Create new DApp
- `GET /api/dapps/:id` - Get specific DApp details

## Testing

### Automated Testing (Windows)
```powershell
# Run API tests
.\scripts\testnet_api_test.ps1

# Or test specific endpoints
.\scripts\testnet_api_test.ps1 -BaseUrl "http://localhost:3000"
```

### Manual Testing
```bash
# Test blockchain info
curl http://localhost:3000/api/blockchain/info

# Test DApps listing
curl http://localhost:3000/api/dapps

# Create a DApp
curl -X POST http://localhost:3000/api/dapps \
  -H "Content-Type: application/json" \
  -d '{"name":"Test DApp","description":"Test description","contract_address":"0x123"}'
```

## Development Features

### WASM Smart Contracts
- Deploy custom WASM contracts
- Execute contract functions via API
- Gas metering and validation

### Token System
- Native OWA token implementation
- Transfer, mint, and balance tracking
- Integration with blockchain state

### Database Integration
- PostgreSQL for persistent storage
- SQLx for type-safe database operations
- Migration system for schema updates

## Configuration

### Environment Variables
- `DATABASE_URL`: PostgreSQL connection string
- `PORT`: Server port (default: 3000)
- `RUST_LOG`: Logging level (debug, info, warn, error)

### Database Schema
- `dapps` table: Stores DApp metadata
- Automatic migrations via SQLx

## Troubleshooting

### Common Issues

1. **Database Connection Failed**
   - Ensure PostgreSQL is running
   - Check DATABASE_URL format
   - Verify database exists

2. **Port Already in Use**
   - Change PORT environment variable
   - Kill existing process on port 3000

3. **Build Errors**
   - Run `cargo clean`
   - Update Rust: `rustup update`
   - Check dependencies in Cargo.toml

### Debug Mode
```bash
# Enable debug logging
export RUST_LOG=debug
cargo run --bin owami-network-simplified
```

## Production Deployment

### Docker Deployment
```bash
# Build Docker image
docker build -t owami-testnet .

# Run with Docker Compose
docker-compose up -d
```

### Cloud Deployment (Render)
- Automatic deployment via `render.yaml`
- PostgreSQL addon configured
- Environment variables set automatically

## Monitoring

### Health Check
```bash
curl http://localhost:3000/api/health
```

### Logs
- Application logs via tracing
- Database query logs (debug mode)
- Request/response logging

## Next Steps

1. **Smart Contract Development**
   - Create custom WASM contracts
   - Deploy via API endpoints
   - Test contract interactions

2. **Frontend Integration**
   - Connect web3 applications
   - Build DApp interfaces
   - Integrate with existing tools

3. **Network Expansion**
   - Add peer-to-peer networking
   - Implement consensus mechanisms
   - Scale to multiple validators

## Support

For issues or questions:
- Check the troubleshooting section
- Review API logs
- Test with provided scripts
- Submit issues to GitHub repository

---
**Created by Mark Creator9**  
**Owami Network Testnet v0.1.0**