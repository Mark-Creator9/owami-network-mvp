# Owami Network DApp API Documentation

This document describes the API endpoints for deploying and interacting with smart contracts on the Owami Network.

## Deploy Smart Contract

Deploys a smart contract to the Owami Network.

### Endpoint
`POST /api/dapp/deploy`

### Request Body
```json
{
  "contract_path": "path/to/contract.sol",
  "network": "testnet" // Optional, defaults to "testnet"
}
```

### Response
```json
{
  "contract_address": "0x1234567890abcdef1234567890abcdef12345678"
}
```

## Call Smart Contract Function

Calls a function on a deployed smart contract.

### Endpoint
`POST /api/dapp/call`

### Request Body
```json
{
  "contract_address": "0x1234567890abcdef1234567890abcdef12345678",
  "function_name": "transfer",
  "params": {
    "to": "0xabcdef1234567890abcdef1234567890abcdef12",
    "amount": 100
  },
  "network": "testnet" // Optional, defaults to "testnet"
}
```

### Response
The response will contain the result of the function call.