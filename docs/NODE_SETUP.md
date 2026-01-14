# Owami Network Node Setup Guide

This guide explains how to set up and run an Owami Network node for the testnet environment. The testnet uses DPoS consensus with mobile-friendly synchronization capabilities.

## Prerequisites

- **Rust 1.90+** (with `cargo`): [Install Rust](https://www.rust-lang.org/tools/install)
- **PostgreSQL 14+**: For local development (Aiven Cloud used in testnet)
- **PowerShell 7+** (for Windows) or **Bash** (for Linux/macOS)
- **Git**: For cloning the repository

## Configuration Setup

### 1. Environment Configuration

Create a `.env.testnet` file in the project root with your testnet configuration:

```env
# Testnet Database Connection (Aiven)
DATABASE_URL=postgres://avnadmin:AVNS_f1dA7_GwLrQY-r0j6zC@owami-testnet-2593.e.aivencloud.com:27345/defaultdb?sslmode=require

# Node Configuration
OWAMI_CONFIG=config/testnet.toml
RUST_LOG=info
```

### 2. Testnet Configuration

The [`config/testnet.toml`](../config/testnet.toml) file contains the consensus parameters:

```toml
[consensus.dpos]
validator_count = 7
block_interval = 3
stake_threshold = 1000
slashing_penalty = 50
```

## Building and Running the Node

### 1. Build the Project

```powershell
# Windows
cargo build --release

# Linux/macOS
cargo build --release
```

### 2. Start the Testnet Node

Use the provided script to launch your node:

```powershell
.\scripts\start_testnet.ps1
```

You should see output indicating successful startup:

```
Starting Owami Testnet Node...
Configuration: config/testnet.toml
Database: postgres://avnadmin:***@owami-testnet-2593.e.aivencloud.com:27345/defaultdb
Access testnet at: http://localhost:8080
Mobile sync endpoint: http://localhost:8080/mobile/sync
```

## Validator Setup

To participate as a validator in the DPoS consensus:

### 1. Register as a Validator

Submit a validator registration transaction with sufficient stake:

```bash
curl -X POST http://localhost:8080/api/register-validator \
  -H "Content-Type: application/json" \
  -d '{
    "address": "YOUR_PUBLIC_KEY_HEX",
    "stake": 1500
  }'
```

> **Note**: Minimum stake required is 1000 OWA tokens as configured in `testnet.toml`

### 2. Verify Validator Status

Check if your node is active in the validator set:

```bash
curl http://localhost:8080/api/validators
```

## Mobile Client Integration

The testnet supports mobile-friendly synchronization through these endpoints:

### 1. Mobile Sync Protocol

```http
POST /mobile/sync
Content-Type: application/json

{
  "last_known_height": 0,
  "max_headers": 100
}
```

Response includes:
- Block headers since specified height
- Current chain height
- Active validator set

### 2. Transaction Verification

```http
POST /mobile/verify
Content-Type: application/json

{
  "transaction_hash": "TX_HASH_HEX",
  "block_height": 12345
}
```

## Troubleshooting

### Common Issues

| Issue | Solution |
|-------|----------|
| Database connection errors | Verify `DATABASE_URL` in `.env.testnet` matches Aiven credentials |
| Validator not elected | Ensure stake meets threshold (1000+) and node is running continuously |
| Mobile sync timeouts | Check network connectivity and server load |

### Log Analysis

View detailed logs with:

```powershell
Get-Content .\logs\owami-network.log -Wait
```

## Next Steps

1. Join the testnet validator set
2. Test mobile synchronization with sample app
3. Contribute to testnet stability monitoring
4. Prepare for mainnet migration

> **Important**: This testnet uses simulated economics. No real value is at stake.