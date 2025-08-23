# Owami Network WASM Testing Scripts

This directory contains scripts for testing WASM smart contract deployment and execution on the Owami Network.

## Prerequisites

1. **Rust and Cargo**: Ensure you have Rust installed with the wasm32-unknown-unknown target
2. **PowerShell**: These scripts are designed for PowerShell (Windows)
3. **Owami Network**: The Owami Network server must be running on `localhost:8081`

## Quick Start

### 1. Build and Deploy a WASM Contract

```powershell
# Navigate to the wasm_examples directory
cd wasm_examples

# Run the complete build and deploy process
.\test_deploy_wasm.ps1
```

This script will:
- Build the simple_contract to WASM
- Deploy it to the Owami Network
- Save the deployment result to `deployment_result.json`

### 2. Test API Endpoints

```powershell
# Test all API endpoints
.\test_api_endpoints.ps1
```

This will test:
- Basic deploy endpoint
- Contract call endpoint
- WASM file upload endpoint

### 3. Manual Steps

If you prefer to run each step manually:

#### Build the Contract
```powershell
# Compile the contract to WASM
.\compile_contract.ps1
```

#### Deploy via API
```powershell
# Using curl (alternative)
curl -X POST http://localhost:8081/api/dapp/deploy_wasm \
  -F "file=@simple_contract.wasm"
```

## File Structure

```
wasm_examples/
├── simple_contract/          # Simple WASM contract example
│   ├── src/
│   │   └── lib.rs           # Contract source code
│   ├── Cargo.toml           # Contract dependencies
│   └── Cargo.lock          # Lock file
├── compile_contract.ps1     # Build script
├── test_deploy_wasm.ps1     # Complete test script
├── test_api_endpoints.ps1   # API testing script
└── README.md               # This file
```

## Troubleshooting

### Common Issues

1. **"WASM file not found"**
   - Ensure `compile_contract.ps1` ran successfully
   - Check that `simple_contract.wasm` exists in the wasm_examples directory

2. **"Connection refused"**
   - Ensure the Owami Network server is running on localhost:8081
   - Check if the port is correct in the scripts

3. **"wasm32-unknown-unknown target not found"**
   - Run: `rustup target add wasm32-unknown-unknown`

4. **"wasm-bindgen not found"**
   - The contract should build without wasm-bindgen for basic WASM
   - For advanced features, ensure dependencies are installed

### Debug Mode

To see detailed error messages, run the scripts with verbose output:

```powershell
# Run with detailed error information
.\test_deploy_wasm.ps1 -Verbose
```

## API Endpoints

The scripts test these endpoints:

- `POST /api/dapp/deploy` - Deploy contract with JSON payload
- `POST /api/dapp/deploy_wasm` - Deploy WASM file via multipart upload
- `POST /api/dapp/call` - Call contract functions

## Expected Output

When successful, you should see:

```
Building WASM contract...
Contract built successfully!
Deploying WASM contract...
Deployment Result:
{
    "contract_address": "0x123abc...",
    "transaction_hash": "0xdef456...",
    "gas_used": 12345,
    "status": "success"
}
Response saved to deployment_result.json
```

## Next Steps

After successful deployment:
1. Use the returned contract address to interact with the contract
2. Check the `deployment_result.json` file for transaction details
3. Use the contract call endpoint to execute contract functions