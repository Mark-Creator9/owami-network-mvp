# Owami Network MVP Testnet Launch Guide

This guide provides step-by-step instructions to launch the MVP testnet with smart contract support.

## Overview

The Owami Network MVP features:
- ✅ **DPoS Consensus** - Delegated Proof of Stake with 7 validators
- ✅ **Smart Contracts** - WASM-based contracts with full lifecycle management
- ✅ **REST API** - Complete blockchain and smart contract endpoints
- ✅ **Token System** - Native token with transfer and mint operations
- ✅ **P2P Network** - libp2p-based decentralized networking

## Prerequisites

### System Requirements
- **Rust**: 1.90.0 or later (`rustc --version`)
- **RAM**: 4GB minimum
- **Disk**: 2GB free space
- **OS**: Windows, macOS, or Linux

### Install Rust (if needed)
```bash
# Windows PowerShell
irm https://sh.rustup.rs -outfile rustup-init.exe
./rustup-init.exe

# macOS/Linux
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Verify installation:
```bash
rustc --version
cargo --version
```

## Phase 1: Local Development Testnet

### 1.1 Clone and Setup

```bash
# Navigate to project directory
cd c:\Users\HP\Desktop\projects\owami-network

# Build the project (first time - takes 5-10 minutes)
cargo build --release
```

### 1.2 Configure Testnet Environment

Create or update `.env.testnet`:
```bash
# Server Configuration
PORT=8080
HOST=0.0.0.0
ENVIRONMENT=testnet
NODE_ID=validator-1

# Logging
RUST_LOG=info

# Optional: Database (local by default uses RocksDB)
# DATABASE_URL=your_postgres_url
```

### 1.3 Start Single Node

```bash
# Run with testnet configuration
cargo run --release -- --config config/testnet.toml
```

You should see:
```
[INFO] Server listening on http://0.0.0.0:8080
[INFO] Initializing WASM smart contract engine
[INFO] Consensus: DPoS (7 validators)
```

### 1.4 Verify Node Health

In a new terminal:
```bash
# Health check
curl http://localhost:8080/api/health

# Expected response:
# {
#   "status": "healthy",
#   "block_height": 0,
#   "network": "owami-testnet"
# }
```

### 1.5 Test Smart Contracts

```bash
# Deploy a simple contract
curl -X POST http://localhost:8080/api/contracts/deploy \
  -H "Content-Type: application/json" \
  -d '{
    "wasm_bytecode": "0x00asm010000000...",
    "creator": "addr_1",
    "contract_type": "test"
  }'

# List deployed contracts
curl http://localhost:8080/api/contracts/list
```

## Phase 2: Multi-Node Local Testnet (Docker)

### 2.1 Create Docker Compose File

Create `docker-compose.yml`:
```yaml
version: '3.8'

services:
  owami-validator-1:
    build: .
    ports:
      - "8080:8080"
      - "4001:4001"
    environment:
      NODE_ID: validator-1
      PORT: 8080
      BOOTSTRAP_PEERS: ""
      RUST_LOG: info
    volumes:
      - ./data/node1:/var/data

  owami-validator-2:
    build: .
    ports:
      - "8081:8080"
      - "4002:4001"
    environment:
      NODE_ID: validator-2
      PORT: 8080
      BOOTSTRAP_PEERS: "/dns4/owami-validator-1/tcp/4001"
      RUST_LOG: info
    volumes:
      - ./data/node2:/var/data
    depends_on:
      - owami-validator-1

  owami-validator-3:
    build: .
    ports:
      - "8082:8080"
      - "4003:4001"
    environment:
      NODE_ID: validator-3
      PORT: 8080
      BOOTSTRAP_PEERS: "/dns4/owami-validator-1/tcp/4001"
      RUST_LOG: info
    volumes:
      - ./data/node3:/var/data
    depends_on:
      - owami-validator-1
```

### 2.2 Create Dockerfile

Create `Dockerfile`:
```dockerfile
FROM rust:1.90

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY config ./config

RUN cargo build --release

EXPOSE 8080 4001

CMD ["./target/release/owami-network", "--config", "config/testnet.toml"]
```

### 2.3 Start 3-Node Testnet

```bash
# Build and start all nodes
docker-compose up --build

# Wait for all nodes to show "Server listening"
# Press Ctrl+C to stop when done testing
```

### 2.4 Test Cluster Health

```bash
# Check all nodes
for i in 0 1 2; do
  echo "Node $i:"
  curl http://localhost:$((8080+i))/api/health
  echo ""
done
```

## Phase 3: Deploy Smart Contracts

### 3.1 Compile Example Contract

The project includes example contracts in `examples/`:

**Rust Contract** (`examples/simple_counter.rs`):
```bash
# Compile to WASM
rustup target add wasm32-unknown-unknown
rustc --target wasm32-unknown-unknown -O examples/simple_counter.rs -o counter.wasm
```

**Solidity Contract** (`examples/SimpleToken.sol`):
```bash
# Requires solc compiler
# Install: https://docs.soliditylang.org/en/latest/installing-solidity.html

solc --optimize --bin --abi examples/SimpleToken.sol
```

### 3.2 Deploy Contract

```bash
# Convert WASM to hex
hexdump -ve '1/1 "%.2x"' counter.wasm

# Deploy contract
curl -X POST http://localhost:8080/api/contracts/deploy \
  -H "Content-Type: application/json" \
  -d '{
    "wasm_bytecode": "0x...",
    "creator": "address_1",
    "contract_type": "counter"
  }'
```

### 3.3 Call Contract Function

```bash
# Call contract function
curl -X POST http://localhost:8080/api/contracts/call \
  -H "Content-Type: application/json" \
  -d '{
    "contract_address": "0x...",
    "function_name": "increment",
    "args": "0x",
    "caller": "address_1"
  }'
```

## Phase 4: Cloud Deployment (Render)

### 4.1 Push to GitHub

```bash
git add -A
git commit -m "MVP Testnet: Smart contracts and multi-node support"
git push origin main
```

### 4.2 Deploy to Render

1. Go to **https://dashboard.render.com**
2. Click **"New +"** → **"Web Service"**
3. Connect your GitHub repository

**Configuration:**
- **Name**: `owami-validator-1`
- **Environment**: Docker
- **Region**: Closest to your location
- **Plan**: Free (or Starter for production)

**Environment Variables:**
```
NODE_ID=validator-1
PORT=8080
RUST_LOG=info
BOOTSTRAP_PEERS=""
```

Wait 5-10 minutes for deployment.

### 4.3 Verify Deployment

```bash
# Test deployed node
curl https://owami-validator-1.onrender.com/api/health

# Deploy additional validators (repeat 4.2 with NODE_ID=validator-2, validator-3)
```

## API Reference

### Smart Contract Endpoints

**Deploy Contract**
```
POST /api/contracts/deploy
Content-Type: application/json

{
  "wasm_bytecode": "0x...",
  "creator": "address",
  "contract_type": "type",
  "constructor_args": "0x..." (optional),
  "gas_limit": 1000000 (optional)
}
```

**List Contracts**
```
GET /api/contracts/list
```

**Get Contract Details**
```
GET /api/contracts/{address}
```

**Call Contract Function**
```
POST /api/contracts/call
Content-Type: application/json

{
  "contract_address": "0x...",
  "function_name": "function_name",
  "args": "0x...",
  "caller": "address",
  "value": 0 (optional),
  "gas_limit": 1000000 (optional)
}
```

**Get Contract Storage**
```
GET /api/contracts/{address}/storage
```

### Blockchain Endpoints

**Get Blockchain Info**
```
GET /api/blockchain/info
```

**Get Block**
```
GET /api/blockchain/blocks/{height}
```

**Mine Block**
```
POST /api/blockchain/mine
```

### Health & Status

**Health Check**
```
GET /api/health
```

**System Status**
```
GET /status
```

## Troubleshooting

### Build Errors

**Error: "unterminated byte string literal"**
- Fixed in `src/contract_registry.rs`
- Run `cargo clean` and rebuild if issue persists

**Error: "dependency not found"**
```bash
cargo update
cargo build --release
```

### Runtime Issues

**Port Already in Use**
```bash
# Windows: Find process using port 8080
netstat -ano | findstr :8080

# Kill process (replace PID)
taskkill /PID <PID> /F

# macOS/Linux
lsof -i :8080
kill -9 <PID>
```

**WASM Bytecode Invalid**
- Ensure bytecode starts with magic number: `00 61 73 6d`
- Verify contract compiled successfully

**Nodes Not Syncing**
- Check bootstrap peer addresses
- Verify P2P port (4001) is accessible
- Review logs: `RUST_LOG=debug cargo run --release`

## Testing Smart Contracts

### Unit Tests
```bash
cargo test --lib
```

### Integration Tests
```bash
cargo test --test '*'
```

### Contract Validation
```bash
# Verify WASM bytecode
wasmparser path/to/contract.wasm
```

## Performance Metrics

Expected performance on MVP testnet:
- **Block Time**: 3 seconds
- **TPS**: 10-50 transactions per second
- **Consensus**: DPoS with 7 validators
- **Finality**: 1 block confirmation

## Next Steps for Production

After successful MVP testnet:

1. **Security Audit**
   - Smart contract audit
   - Network security review
   - Cryptographic validation

2. **Performance Optimization**
   - Increase validator count
   - Optimize gas metering
   - Implement state snapshots

3. **Scaling**
   - Sharding support
   - Layer 2 solutions
   - Cross-chain bridges

4. **Ecosystem**
   - Developer SDK
   - Mobile wallets
   - Block explorer

## Support & Documentation

- **API Docs**: `docs/SMART_CONTRACT_API.md`
- **Quick Start**: `docs/QUICKSTART.md`
- **Troubleshooting**: `docs/TROUBLESHOOTING.md`
- **GitHub**: Your repository

## Success Checklist

- [ ] Local single-node testnet running
- [ ] Single node health check passing
- [ ] Smart contract deployment working
- [ ] Contract function calls executing
- [ ] 3-node Docker testnet syncing
- [ ] All nodes reporting same block height
- [ ] Nodes deployed to Render
- [ ] Cloud nodes syncing correctly
- [ ] MVP testnet public and accessible

## Launch Commands (Quick Reference)

```bash
# Local Development
cargo run --release -- --config config/testnet.toml

# Local Multi-Node
docker-compose up --build

# Check Logs
docker logs owami-validator-1

# Stop All Nodes
docker-compose down
```

---

**Status**: MVP Ready
**Last Updated**: October 2024
**Version**: 1.0.0
