# Owami Network Testnet Guide

## Current Testnet Details
- **API Base URL**: http://localhost:8080/testnet
- **Developer Key**: Generated on server startup (check logs)
- **Reset Schedule**: Every 2 weeks

## Getting Started

### 1. Run Testnet Node
```bash
cargo run --release
```

### 2. Get Test Tokens
Via API:
```bash
curl -X POST -H "Authorization: Bearer YOUR_API_KEY" \
  http://localhost:8080/testnet/faucet \
  -d '{"address":"YOUR_WALLET"}'
```

### 3. Make Transactions
Sample transfer:
```bash 
curl -X POST -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  http://localhost:8080/testnet/transfer \
  -d '{"from":"YOUR_WALLET","to":"RECIPIENT_ADDRESS","amount":100}'
```

## Developer Tools

### API Reference
See [API_DOCS.md](./API_DOCS.md) for complete endpoint documentation

### Testing Scripts
Sample scripts available in `/scripts/` directory:
- `distribute_test_tokens.sh` - Bulk token distribution
- `network_test.sh` - Basic network validation

## Reporting Issues
1. Check existing issues at:
   https://github.com/owami-network/testnet/issues
2. Include:
   - API request/response samples
   - Node logs
   - Steps to reproduce