# Smart Contract API Documentation

This document describes the WASM smart contract functionality implemented in the Owami Network.

## Overview

The Owami Network now supports WebAssembly (WASM) smart contracts with the following features:

- **WASM Runtime**: Built on Wasmtime for secure execution
- **Solidity Compilation**: Solidity source code compilation to WASM
- **Rust WASM**: Native Rust smart contract support
- **Contract Registry**: Complete contract lifecycle management
- **Gas Metering**: Resource usage tracking and limits
- **State Management**: Persistent contract storage
- **Security**: Sandboxed execution with proper isolation

## API Endpoints

### Smart Contract Deployment

#### Deploy Contract from Source Code
```http
POST /api/contracts/deploy
Content-Type: application/json

{
  "contract_type": "solidity",
  "contract_code": "base64-encoded-source-code",
  "network": "testnet"
}
```

**Response:**
```json
{
  "success": true,
  "contract_address": "0x123...abc",
  "message": "Contract deployed successfully",
  "contract": {
    "address": "0x123...abc",
    "creator": "0xdef...456",
    "contract_type": "solidity",
    "deployment_height": 1,
    "deployment_timestamp": "2024-01-01T00:00:00Z",
    "version": "1.0.0"
  }
}
```

#### Deploy WASM Contract from File
```http
POST /api/contracts/deploy_wasm
Content-Type: multipart/form-data

file: [wasm-binary-file]
creator: [optional-creator-address]
```

#### Deploy Contract from File
```http
POST /api/contracts/deploy_file
Content-Type: application/json

{
  "contract_path": "/path/to/contract.sol",
  "contract_type": "solidity",
  "network": "testnet"
}
```

### Contract Interaction

#### Call Contract Function
```http
POST /api/contracts/call
Content-Type: application/json

{
  "contract_address": "0x123...abc",
  "function_name": "transfer",
  "args": {
    "to": "0xdef...456",
    "amount": "1000000000000000000"
  },
  "caller": "0x789...def",
  "gas_limit": 1000000
}
```

**Response:**
```json
{
  "success": true,
  "result": "0x...",
  "gas_used": 21000,
  "events": [
    {
      "contract_address": "0x123...abc",
      "event_name": "Transfer",
      "data": {
        "from": "0x789...def",
        "to": "0xdef...456",
        "value": "1000000000000000000"
      },
      "block_number": 1,
      "transaction_hash": "0x..."
    }
  ],
  "error": null
}
```

### Contract Management

#### Get Contract Information
```http
GET /api/contracts/{address}
```

**Response:**
```json
{
  "address": "0x123...abc",
  "creator": "0xdef...456",
  "wasm_bytecode": "0x...",
  "abi": [...],
  "deployment_height": 1,
  "deployment_timestamp": "2024-01-01T00:00:00Z",
  "contract_type": "solidity",
  "version": "1.0.0",
  "metadata": {
    "name": "SimpleToken",
    "description": "A simple ERC20-like token",
    "version": "1.0.0",
    "license": "MIT",
    "authors": ["Developer"],
    "links": {}
  }
}
```

#### List All Contracts
```http
GET /api/contracts
```

**Response:**
```json
[
  {
    "address": "0x123...abc",
    "creator": "0xdef...456",
    "contract_type": "solidity",
    "deployment_height": 1,
    "deployment_timestamp": "2024-01-01T00:00:00Z",
    "version": "1.0.0"
  },
  {
    "address": "0x456...def",
    "creator": "0x789...abc",
    "contract_type": "wasm",
    "deployment_height": 2,
    "deployment_timestamp": "2024-01-01T01:00:00Z",
    "version": "1.0.0"
  }
]
```

#### Get Contract Storage
```http
GET /api/contracts/{address}/storage
```

**Response:**
```json
{
  "contract_address": "0x123...abc",
  "storage": {
    "count": "42",
    "owner": "0xdef...456"
  },
  "message": "Storage retrieved successfully"
}
```

### Compilation

#### Compile Source Code
```http
POST /api/contracts/compile
Content-Type: application/json

{
  "source": "pragma solidity ^0.8.0; contract Simple { uint256 public value; }",
  "language": "solidity"
}
```

**Response:**
```json
{
  "success": true,
  "message": "Contract compiled successfully",
  "result": {
    "contracts": {
      "Simple.sol:Simple": {
        "abi": [...],
        "bin": "0x608060405234801561001057600080fd5b50...",
        "function_hashes": {...},
        "metadata": {...}
      }
    },
    "sources": {...},
    "version": "0.8.0"
  }
}
```

### Statistics

#### Get Deployment Statistics
```http
GET /api/deploy/stats
```

**Response:**
```json
{
  "total_contracts": 5,
  "contract_types": {
    "wasm": 3,
    "solidity": 2
  },
  "total_deployments": 5
}
```

## Smart Contract Development

### Solidity Contracts

The network supports Solidity contracts with the following features:

- **Version**: Solidity ^0.8.0
- **Compilation**: Automatic compilation to WASM
- **Standard Functions**: Common ERC20 and ERC721 patterns
- **Events**: Event emission and logging
- **Storage**: Persistent state management

Example Solidity contract:
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract SimpleToken {
    string public name;
    string public symbol;
    uint8 public decimals;
    uint256 public totalSupply;
    
    mapping(address => uint256) public balances;
    
    event Transfer(address indexed from, address indexed to, uint256 value);
    
    constructor(string memory _name, string memory _symbol, uint256 _initialSupply) {
        name = _name;
        symbol = _symbol;
        decimals = 18;
        totalSupply = _initialSupply * (10 ** uint256(decimals));
        balances[msg.sender] = totalSupply;
        emit Transfer(address(0), msg.sender, totalSupply);
    }
    
    function transfer(address _to, uint256 _value) public returns (bool) {
        require(_to != address(0), "Transfer to the zero address");
        require(_value <= balances[msg.sender], "Insufficient balance");
        
        balances[msg.sender] -= _value;
        balances[_to] += _value;
        emit Transfer(msg.sender, _to, _value);
        return true;
    }
}
```

### Rust WASM Contracts

The network also supports native Rust smart contracts:

```rust
use std::collections::HashMap;

struct Counter {
    count: u64,
    owner: String,
}

impl Counter {
    fn new() -> Self {
        Self {
            count: 0,
            owner: "default".to_string(),
        }
    }

    fn increment(&mut self) -> u64 {
        self.count += 1;
        self.count
    }

    fn get_count(&self) -> u64 {
        self.count
    }
}

// Contract interface functions
#[no_mangle]
pub extern "C" fn increment() -> u64 {
    let mut counter = Counter::new();
    counter.increment()
}

#[no_mangle]
pub extern "C" fn get_count() -> u64 {
    let counter = Counter::new();
    counter.get_count()
}
```

## Security Features

### Sandboxing
- All contracts run in isolated Wasmtime instances
- No direct access to host system resources
- Memory limits and gas metering prevent abuse

### Gas Metering
- Each operation consumes gas
- Gas limits prevent infinite loops
- Failed operations don't consume gas

### State Management
- Persistent storage per contract
- Atomic state updates
- Versioned state history

## Error Handling

### Common Error Codes

| Code | Description |
|------|-------------|
| `CONTRACT_NOT_FOUND` | Contract address doesn't exist |
| `INVALID_FUNCTION` | Function doesn't exist in contract |
| `INSUFFICIENT_GAS` | Gas limit exceeded |
| `INVALID_ARGUMENTS` | Function arguments are invalid |
| `EXECUTION_FAILED` | Contract execution failed |
| `COMPILATION_ERROR` | Source code compilation failed |

### Error Response Format
```json
{
  "success": false,
  "error": "Contract not found",
  "code": "CONTRACT_NOT_FOUND"
}
```

## Testing

### Contract Testing Framework

The network includes a testing framework for smart contracts:

```bash
# Run contract tests
cargo test --package owami-network --lib wasm_tests

# Test specific contract
cargo test --package owami-network --lib counter_tests

# Run integration tests
cargo test --package owami-network --test integration_tests
```

### Test Examples

```rust
#[tokio::test]
async fn test_contract_deployment() -> Result<()> {
    let config = AppConfig::load().unwrap();
    let mut service = DeploymentService::new(config);
    
    let wasm_bytes = include_bytes!("../examples/simple_counter.wasm");
    let response = service.deploy_wasm_contract(wasm_bytes, "test_creator").await?;
    
    assert!(response.success);
    assert!(!response.contract_address.is_empty());
    
    Ok(())
}

#[tokio::test]
async fn test_contract_function_call() -> Result<()> {
    let config = AppConfig::load().unwrap();
    let mut service = DeploymentService::new(config);
    
    // Deploy contract
    let wasm_bytes = include_bytes!("../examples/simple_counter.wasm");
    let deploy_response = service.deploy_wasm_contract(wasm_bytes, "test_creator").await?;
    assert!(deploy_response.success);
    
    // Call function
    let call_response = service.call_contract(
        &deploy_response.contract_address,
        "increment",
        &[],
        "test_caller"
    ).await?;
    
    assert!(call_response.success);
    assert_eq!(call_response.result, vec![1]); // Should return 1 after first increment
    
    Ok(())
}
```

## Deployment Examples

### Using curl

```bash
# Deploy a Solidity contract
curl -X POST http://localhost:5000/api/contracts/deploy \
  -H "Content-Type: application/json" \
  -d '{
    "contract_type": "solidity",
    "contract_code": "c2NlbmRpbmc6IE1JVCBsaWNlbnNlIGZyb20gY29udHJvbGxlciBhbmQgcmVxdWVzdCB0byB0aGUgc3RhdGUgY29udHJvbA==",
    "network": "testnet"
  }'

# Call a contract function
curl -X POST http://localhost:5000/api/contracts/call \
  -H "Content-Type: application/json" \
  -d '{
    "contract_address": "0x123...abc",
    "function_name": "transfer",
    "args": {"to": "0xdef...456", "amount": "1000000000000000000"},
    "caller": "0x789...def",
    "gas_limit": 1000000
  }'
```

### Using JavaScript

```javascript
// Deploy contract
async function deployContract() {
    const response = await fetch('/api/contracts/deploy', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({
            contract_type: 'solidity',
            contract_code: btoa('pragma solidity ^0.8.0; contract Simple { uint256 public value; }'),
            network: 'testnet'
        })
    });
    
    const result = await response.json();
    console.log('Deployment result:', result);
    return result.contract_address;
}

// Call contract function
async function callContract(contractAddress, functionName, args) {
    const response = await fetch('/api/contracts/call', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({
            contract_address: contractAddress,
            function_name: functionName,
            args: args,
            caller: '0x789...def',
            gas_limit: 1000000
        })
    });
    
    const result = await response.json();
    console.log('Function call result:', result);
    return result;
}
```

## Performance Considerations

### Gas Optimization
- Use efficient algorithms in contracts
- Minimize storage operations
- Use events instead of return values for logging

### Best Practices
1. **State Management**: Keep state minimal and only store necessary data
2. **Function Design**: Keep functions simple and focused
3. **Error Handling**: Implement proper error checking and revert conditions
4. **Testing**: Thoroughly test all contract functions before deployment
5. **Security**: Validate all inputs and prevent reentrancy attacks

### Monitoring
- Monitor gas usage for each contract
- Track contract execution times
- Monitor storage growth
- Set up alerts for unusual contract behavior

## Future Enhancements

Planned features for future releases:

1. **Upgradability**: Contract upgrade mechanisms
2. **Cross-Contract Calls**: Inter-contract communication
3. **Oracle Integration**: External data feeds
4. **Advanced Security**: Formal verification tools
5. **Performance Optimization**: Caching and indexing
6. **Developer Tools**: IDE plugins and debugging tools

## Support

For issues and questions:
1. Check the API documentation above
2. Review the example contracts in `/examples/`
3. Run the test suite to validate functionality
4. Check server logs for detailed error information
5. Contact the development team for specific issues

---

**Note**: This is a comprehensive implementation of WASM smart contract functionality for the Owami Network. All features are designed to be secure, efficient, and developer-friendly while maintaining the high standards expected from a blockchain platform.