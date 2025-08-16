# Owami Network API Documentation

## Base URL
The API base URL is `http://localhost:8080/api`. All endpoints are relative to this base URL.

## Authentication
Most endpoints require authentication via API key in the Authorization header:
```
Authorization: Bearer YOUR_API_KEY
```

## Endpoints

### Create Wallet
Creates a new wallet on the network.

```http
POST /wallets/create
Content-Type: application/json
```

**Response**
```json
{
    "address": "generated_address",
    "private_key": "generated_private_key"
}
```

### Get Wallet Balance
Retrieves the current balance for a wallet.

```http
GET /wallets/{address}/balance
```

**Parameters**
- `address`: Wallet address (path parameter)

**Response**
```json
{
    "balance": 1000
}
```

### Request Test Tokens (Faucet)
Request test tokens for a wallet address.

```http
POST /faucet
Content-Type: application/json
```

**Request Body**
```json
{
    "address": "0x123"
}
```

**Response**
```json
{
    "status": "success"
}
```

### Get Transaction History
Retrieves transaction history for a wallet.

```http
GET /wallets/{address}/transactions
```

**Parameters**
- `address`: Wallet address (path parameter)

**Response**
```json
[
    {
        "amount": 100,
        "fee": 1,
        "from": "0xsender",
        "to": "0xreceiver",
        "hash": "0x123",
        "nonce": 0,
        "signature": []
    }
]
```

### Deploy Smart Contract
Deploys a smart contract to the network.

```http
POST /dapp/deploy
Content-Type: application/json
```

**Request Body**
```json
{
    "contract_path": "path/to/contract.sol",
    "network": "testnet"
}
```

**Response**
```json
{
    "contract_address": "0x1234567890abcdef1234567890abcdef12345678"
}
```

### Call Smart Contract Function
Calls a function on a deployed smart contract.

```http
POST /dapp/call
Content-Type: application/json
```

**Request Body**
```json
{
    "contract_address": "0x1234567890abcdef1234567890abcdef12345678",
    "function_name": "transfer",
    "params": {
        "to": "0xabcdef1234567890abcdef1234567890abcdef12",
        "amount": 100
    },
    "network": "testnet"
}
```

**Response**
The response will contain the result of the function call.

## Error Responses

### Database Connection Error
```json
{
    "error": "Database connection failed",
    "details": "error details here"
}
```

## Testing
You can test the API using curl commands:

```bash
# Create a wallet
curl -X POST http://localhost:8080/api/wallets/create -H "Content-Type: application/json"

# Check balance
curl http://localhost:8080/api/wallets/0x123/balance

# Request tokens from faucet
curl -X POST http://localhost:8080/api/faucet -H "Content-Type: application/json" -d '{"address":"0x123"}'

# Get transaction history
curl http://localhost:8080/api/wallets/0x123/transactions

# Deploy a smart contract
curl -X POST http://localhost:8080/api/dapp/deploy -H "Content-Type: application/json" -d '{"contract_path":"path/to/contract.sol","network":"testnet"}'

# Call a smart contract function
curl -X POST http://localhost:8080/api/dapp/call -H "Content-Type: application/json" -d '{"contract_address":"0x1234567890abcdef1234567890abcdef12345678","function_name":"transfer","params":{"to":"0xabcdef1234567890abcdef1234567890abcdef12","amount":100},"network":"testnet"}'
```
# Owami Network API Documentation

## Base URL
`http://localhost:8080`

## Authentication
- Required for protected endpoints
- Include in headers:
```
Authorization: Bearer YOUR_API_KEY
```

## Performance Metrics

### Transaction Processing Speed
Performance measurements based on local testing:

1. **Base Performance (Sequential)**:
   - TPS: 2.7 transactions per second
   - Configuration:
     * 1000 sequential transactions
     * Single client requests
     * No optimizations

2. **Optimized Performance (Concurrent)**:
   - TPS: 26.73 transactions per second (10x improvement)
   - Configuration:
     * 1000 total transactions
     * 10 concurrent requests
     * Batch processing enabled
     * Parallel validation
  * Local network environment
  * Release build configuration

### Performance Test Scripts
Two test scripts are provided to measure performance:

1. **Basic Interaction Test** (`test_alice_bob.ps1`):
   - Tests basic wallet-to-wallet interactions
   - Suitable for functional testing

2. **Performance Benchmark** (`test_tps.ps1`):
   - Measures transaction throughput
   - Sends 1000 sequential transactions
   - Calculates actual TPS
   - Provides detailed progress monitoring

### Running Performance Tests
```powershell
# Full TPS benchmark
pwsh ./test_tps.ps1

# Basic interaction test
pwsh ./test_alice_bob.ps1
```

### Performance Considerations
1. Current TPS is measured in a development environment
2. Sequential transaction processing is used
3. No parallel processing optimization
4. Local network latency only
5. Single client scenario

### Potential Optimizations
- Implement batch transaction processing
- Add parallel transaction handling
- Optimize database operations
- Implement caching mechanisms
- Add load balancing for distributed deployment

## Test Scenarios

The following test scenarios are available to verify the functionality of the Owami Network:

### 1. Unit Tests
Located in the `tests/` directory:

- **Blockchain Tests** (`blockchain_tests.rs`):
  - Genesis block creation
  - Block validation
  - Shared blockchain state
  
- **Token Tests** (`token_tests.rs`):
  - Token transfer validation
  - Transaction signing
  - Vesting operations

### 2. API Integration Tests

#### Basic User Flow (Alice and Bob)
Run with `test_alice_bob.ps1`:
```powershell
pwsh ./test_alice_bob.ps1
```
Tests basic wallet-to-wallet interactions.

#### Comprehensive Test Suite
Run with `test_all_scenarios.ps1`:
```powershell
pwsh ./test_all_scenarios.ps1
```

This suite covers:

1. **Basic API Tests**
   - Health endpoint verification
   ```bash
   curl http://localhost:8080/
   # Response: "Owami Network API v0.1"
   ```

2. **Wallet Operations**
   - Wallet creation
   - Balance checking
   ```bash
   # Example Response:
   {"balance":1000}
   ```

3. **Faucet Operations**
   - Token request
   - Balance verification
   ```bash
   # Example Response:
   {"status":"success"}
   ```

4. **Transaction Tests**
   - Token transfers
   - Balance updates
   ```bash
   # Example transaction:
   {
     "from": "0x123",
     "to": "0x456",
     "amount": 100,
     "privateKey": "private"
   }
   ```

5. **Transaction History**
   - Historical transaction lookup
   ```bash
   # Example Response:
   [{
     "amount": 100,
     "fee": 1,
     "from": "0xsender",
     "hash": "0x123",
     "nonce": 0,
     "signature": [],
     "to": "0xreceiver"
   }]
   ```

6. **Error Handling**
   - Invalid wallet addresses
   - Invalid transaction parameters
   - Database connection failures

### Test Environment Setup
```bash
# 1. Start the server
cargo run --release

# 2. Run comprehensive tests
pwsh ./test_all_scenarios.ps1

# 3. Monitor test output for:
- Successful API responses
- Correct balance updates
- Transaction processing
- Error handling
```

### Expected Test Results
- All API endpoints return appropriate status codes
- Wallet balances update correctly after transactions
- Invalid operations are properly rejected
- Transaction history is accurately maintained
- Error responses include helpful details

## Example Test Scenario: Alice and Bob

This example demonstrates a complete interaction between two users (Alice and Bob) using the API.

### Test Flow

1. Create wallets for both users
```bash
# Create Alice's wallet
curl -X POST http://localhost:8080/api/wallets/create
# Response: {"address":"0x123","private_key":"private"}

# Create Bob's wallet
curl -X POST http://localhost:8080/api/wallets/create
# Response: {"address":"0x123","private_key":"private"}
```

2. Check initial balances
```bash
# Check Alice's balance
curl http://localhost:8080/api/wallets/0x123/balance
# Response: {"balance":1000}

# Check Bob's balance
curl http://localhost:8080/api/wallets/0x123/balance
# Response: {"balance":1000}
```

3. Request test tokens for Alice
```bash
curl -X POST http://localhost:8080/api/faucet -H "Content-Type: application/json" \
     -d '{"address":"0x123"}'
# Response: {"status":"success"}
```

4. Make a transfer from Alice to Bob
```bash
curl -X POST http://localhost:8080/api/transactions \
     -H "Content-Type: application/json" \
     -d '{
       "from": "0x123",
       "to": "0x123",
       "amount": 100,
       "privateKey": "private"
     }'
```

5. View transaction history
```bash
# Get Alice's transactions
curl http://localhost:8080/api/wallets/0x123/transactions
# Response: [{"amount":100,"fee":1,"from":"0xsender","hash":"0x123","nonce":0,"signature":[],"to":"0xreceiver"}]
```

### Automated Testing
A PowerShell script `test_alice_bob.ps1` is provided to run through this complete scenario automatically:

```powershell
pwsh ./test_alice_bob.ps1
```

This script:
- Creates wallets for Alice and Bob
- Gets their initial balances
- Requests test tokens for Alice
- Performs a transfer from Alice to Bob
- Shows final balances
- Displays transaction history for both users
```
- Current developer key: `058000ef-b9d0-4f35-9c5c-eff48ad61869`

## Endpoints

### Health Check
`GET /`
- Public endpoint
- Example:
```bash
curl http://localhost:8080
```
- Response: 
```
Owami Network API v0.1
```

### Sandbox
`POST /testnet/sandbox`
- Requires authentication
- Example:
```bash
curl -X POST -H "Authorization: Bearer 058000ef-b9d0-4f35-9c5c-eff48ad61869" \
  http://localhost:8080/testnet/sandbox
```
- Response:
```json
{
  "status": "success",
  "message": "Sandbox environment initialized"
}
```

### Network Stats
`GET /testnet/stats`
- Requires authentication
- Example:
```bash
curl -H "Authorization: Bearer 058000ef-b9d0-4f35-9c5c-eff48ad61869" \
  http://localhost:8080/testnet/stats
```
- Response:
```json
{
  "nodes": 15,
  "transactions": 42, 
  "blocks": 103
}
```

## Error Responses
- `401 Unauthorized` - Missing/invalid API key
- `500 Server Error` - Internal server error