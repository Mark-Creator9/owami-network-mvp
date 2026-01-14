# MVP Deployment Checklist & Verification Guide

Use this checklist to verify your Owami Network MVP testnet is properly deployed and functioning.

## Phase 1: Prerequisites & Setup

- [ ] **Rust Installed**
  ```bash
  rustc --version  # Should show 1.70+
  cargo --version
  ```

- [ ] **Project Directory Accessible**
  ```bash
  cd c:\Users\HP\Desktop\projects\owami-network
  ls  # or dir on Windows
  ```

- [ ] **Configuration Files Present**
  ```bash
  ls config/testnet.toml       # Testnet config
  ls config/production.toml    # Production config
  ```

- [ ] **Example Contracts Available**
  ```bash
  ls examples/simple_counter.rs
  ls examples/SimpleToken.sol
  ```

- [ ] **Dockerfile & Docker Compose Ready**
  ```bash
  ls Dockerfile
  ls docker-compose.yml
  ```

## Phase 2: Code Quality

- [ ] **Build Succeeds**
  ```bash
  cargo build --release
  # Should compile without errors
  ```

- [ ] **No Syntax Errors**
  ```bash
  cargo check  # Quick syntax check
  ```

- [ ] **Tests Pass**
  ```bash
  cargo test --lib
  # Should run without panics
  ```

- [ ] **Key File: contract_registry.rs**
  - [x] Syntax errors fixed
  - [x] Test module compiles
  - [x] WASM bytecode validation present

## Phase 3: Single-Node Launch

### 3.1 Local Development Start

- [ ] **Start Server**
  ```bash
  cargo run --release -- --config config/testnet.toml
  # Wait 5-10 seconds for startup
  ```

- [ ] **Server Listening**
  ```bash
  # Look for: "Server listening on http://0.0.0.0:8080"
  # Or: "listening on port 8080"
  ```

- [ ] **No Critical Errors**
  ```bash
  # No "panic", "fatal", or "error" messages in logs
  ```

### 3.2 Health Verification

- [ ] **Health Check Endpoint**
  ```bash
  curl http://localhost:8080/api/health
  # Expected: {"status": "healthy", ...}
  ```

- [ ] **Blockchain Info**
  ```bash
  curl http://localhost:8080/api/blockchain/info
  # Expected: {"block_height": 0, ...}
  ```

- [ ] **List Contracts (Empty)**
  ```bash
  curl http://localhost:8080/api/contracts/list
  # Expected: {"contracts": []}
  ```

### 3.3 Basic Operations

- [ ] **Mine a Block**
  ```bash
  curl -X POST http://localhost:8080/api/blockchain/mine
  # Expected: {"success": true, ...}
  ```

- [ ] **Verify Block Created**
  ```bash
  curl http://localhost:8080/api/blockchain/info
  # block_height should be 1
  ```

- [ ] **API Response Times**
  ```bash
  time curl http://localhost:8080/api/health
  # Should be < 500ms
  ```

## Phase 4: Smart Contract Operations

- [ ] **Deploy Test Contract**
  ```bash
  curl -X POST http://localhost:8080/api/contracts/deploy \
    -H "Content-Type: application/json" \
    -d '{
      "wasm_bytecode": "0x00asm0100000007606001ff01ff030201000503010010",
      "creator": "test_user",
      "contract_type": "test"
    }'
  # Expected: {"address": "0x...", "success": true}
  ```

- [ ] **Get Contract Address from Response**
  - Save the address returned above

- [ ] **List Contracts (Should Show 1)**
  ```bash
  curl http://localhost:8080/api/contracts/list
  # Should show 1 contract
  ```

- [ ] **Get Contract Details**
  ```bash
  curl http://localhost:8080/api/contracts/{address}
  # Replace {address} with the address from deployment
  # Expected: Full contract details
  ```

- [ ] **Call Contract Function**
  ```bash
  curl -X POST http://localhost:8080/api/contracts/call \
    -H "Content-Type: application/json" \
    -d '{
      "contract_address": "{address}",
      "function_name": "test",
      "args": "0x",
      "caller": "test_user"
    }'
  # Expected: {"success": true, ...}
  ```

- [ ] **Query Contract Storage**
  ```bash
  curl http://localhost:8080/api/contracts/{address}/storage
  # Expected: {"storage": {...}} or {"storage": null}
  ```

## Phase 5: Multi-Node Testing (Docker)

### 5.1 Docker Setup

- [ ] **Docker Installed**
  ```bash
  docker --version  # Should show version
  docker-compose --version
  ```

- [ ] **Docker Compose File Present**
  ```bash
  cat docker-compose.yml | grep services
  # Should show 3 validators
  ```

### 5.2 Docker Build

- [ ] **Build Docker Image**
  ```bash
  docker-compose build
  # Should complete without errors
  ```

- [ ] **Image Built Successfully**
  ```bash
  docker images | grep owami
  # Should show owami-network image
  ```

### 5.3 Multi-Node Launch

- [ ] **Start 3-Node Network**
  ```bash
  docker-compose up --build
  # Wait for all 3 nodes to start (30-60 seconds)
  ```

- [ ] **All Containers Running**
  ```bash
  docker ps | grep owami
  # Should show 3 containers running
  ```

- [ ] **Node 1 Health**
  ```bash
  curl http://localhost:8080/api/health
  # Status: healthy
  ```

- [ ] **Node 2 Health**
  ```bash
  curl http://localhost:8081/api/health
  # Status: healthy
  ```

- [ ] **Node 3 Health**
  ```bash
  curl http://localhost:8082/api/health
  # Status: healthy
  ```

### 5.4 Network Synchronization

- [ ] **Mine Block on Node 1**
  ```bash
  curl -X POST http://localhost:8080/api/blockchain/mine
  ```

- [ ] **Wait 5 Seconds**
  ```bash
  sleep 5
  ```

- [ ] **Check Block Height on Node 1**
  ```bash
  curl http://localhost:8080/api/blockchain/info
  # block_height should be 1
  ```

- [ ] **Check Block Height on Node 2**
  ```bash
  curl http://localhost:8081/api/blockchain/info
  # block_height should also be 1
  ```

- [ ] **Check Block Height on Node 3**
  ```bash
  curl http://localhost:8082/api/blockchain/info
  # block_height should also be 1
  ```

### 5.5 P2P Network Verification

- [ ] **Check Node 1 Logs**
  ```bash
  docker logs owami-validator-1 | grep -i "peer\|connection"
  # Should see connection messages
  ```

- [ ] **Check Node 2 Logs**
  ```bash
  docker logs owami-validator-2 | grep -i "peer\|connection"
  ```

- [ ] **Check Node 3 Logs**
  ```bash
  docker logs owami-validator-3 | grep -i "peer\|connection"
  ```

### 5.6 Smart Contract on Network

- [ ] **Deploy Contract via Node 1**
  ```bash
  curl -X POST http://localhost:8080/api/contracts/deploy \
    -H "Content-Type: application/json" \
    -d '{
      "wasm_bytecode": "0x00asm0100000007606001ff01ff030201000503010010",
      "creator": "network_test",
      "contract_type": "network"
    }'
  ```

- [ ] **Contract Visible on Node 2**
  ```bash
  curl http://localhost:8081/api/contracts/list
  # Should show the deployed contract
  ```

- [ ] **Contract Visible on Node 3**
  ```bash
  curl http://localhost:8082/api/contracts/list
  # Should show the deployed contract
  ```

## Phase 6: Load & Performance

- [ ] **Rapid Health Checks (10 requests)**
  ```bash
  for i in {1..10}; do
    time curl http://localhost:8080/api/health
  done
  # All should return in <500ms
  ```

- [ ] **Multiple Contract Deployments**
  ```bash
  # Deploy 5 contracts in quick succession
  for i in {1..5}; do
    curl -X POST http://localhost:8080/api/contracts/deploy \
      -H "Content-Type: application/json" \
      -d "{\"wasm_bytecode\": \"0x00asm0100000007606001ff01ff030201000503010010\", \"creator\": \"user_$i\", \"contract_type\": \"load_test\"}"
  done
  ```

- [ ] **Verify All Contracts Deployed**
  ```bash
  curl http://localhost:8080/api/contracts/list | grep -c address
  # Should show 5
  ```

- [ ] **Memory Usage Reasonable**
  ```bash
  docker stats --no-stream owami-validator-1
  # MEMORY usage should be < 500MB
  ```

## Phase 7: Configuration & Security

- [ ] **Rate Limiting Active**
  ```bash
  # Send 201 requests in 60 seconds
  for i in {1..201}; do
    curl http://localhost:8080/api/health &
  done
  # Some should be rate limited (429 errors expected)
  ```

- [ ] **CORS Configured**
  ```bash
  curl -H "Origin: http://localhost:3000" http://localhost:8080/api/health -v
  # Should see CORS headers in response
  ```

- [ ] **Testnet Configuration Active**
  ```bash
  curl http://localhost:8080/api/blockchain/info | grep -i "consensus"
  # Should show DPoS
  ```

## Phase 8: Cleanup & Stop

- [ ] **Stop Single Node Server**
  ```bash
  # Press Ctrl+C in the running terminal
  ```

- [ ] **Stop Docker Network**
  ```bash
  docker-compose down
  # All containers should stop
  ```

- [ ] **Verify Containers Stopped**
  ```bash
  docker ps | grep owami
  # Should return empty
  ```

- [ ] **Remove Docker Volumes (Optional)**
  ```bash
  docker-compose down -v
  # Removes all data volumes for clean restart
  ```

## Phase 9: Final Verification

### Document Check
- [ ] `MVP_TESTNET_SUMMARY.md` - Overview present
- [ ] `MVP_TESTNET_LAUNCH.md` - Detailed guide present
- [ ] `START_HERE_MVP_TESTNET.md` - Quick start present
- [ ] `docs/SMART_CONTRACT_API.md` - API docs present
- [ ] `Dockerfile` - Containerization ready
- [ ] `docker-compose.yml` - Multi-node config ready
- [ ] `config/testnet.toml` - Configuration present

### Code Check
- [ ] All source files compile
- [ ] No syntax errors
- [ ] Tests pass
- [ ] Build artifacts present in `target/release/`

### Functionality Check
- [ ] Single-node testnet works
- [ ] 3-node testnet syncs
- [ ] Smart contracts deploy
- [ ] Smart contracts execute
- [ ] API endpoints responsive

## Success Criteria

✅ **MVP Testnet is Ready for Deployment When:**

1. All checks in **Phase 1-3** pass (Single Node)
2. All checks in **Phase 4** pass (Smart Contracts)
3. All checks in **Phase 5** pass (Multi-Node Sync)
4. All checks in **Phase 6** pass (Performance)
5. All checks in **Phase 7** pass (Security)

## Troubleshooting by Phase

### Phase 1-2 Issues
- **Error**: "Rust not installed"
  - **Fix**: Install from https://rustup.rs/
  
- **Error**: "Configuration files missing"
  - **Fix**: Ensure you're in the correct directory

### Phase 3 Issues
- **Error**: "Port 8080 already in use"
  - **Fix**: See START_HERE_MVP_TESTNET.md → Troubleshooting

- **Error**: "Connection refused"
  - **Fix**: Wait 10 seconds after starting, server is initializing

### Phase 4 Issues
- **Error**: "Invalid WASM bytecode"
  - **Fix**: Ensure bytecode starts with `0x00asm`
  - **Fix**: Use proper hex format

### Phase 5 Issues
- **Error**: "Nodes not syncing"
  - **Fix**: Check bootstrap peer configuration
  - **Fix**: Ensure all containers have network connectivity
  - **Fix**: Check P2P port 4001 is accessible

### Phase 6-7 Issues
- **Error**: "High response times"
  - **Fix**: Check system resources (RAM, CPU)
  - **Fix**: Reduce load test intensity

## Performance Baseline

Expected results after passing all phases:

| Metric | Expected |
|--------|----------|
| Health Check | < 100ms |
| Contract Deploy | < 500ms |
| Contract Call | < 200ms |
| Block Mine | < 3 seconds |
| 3-Node Sync | < 5 seconds |
| Block Propagation | < 1 second |
| Memory Usage (Node) | < 500MB |
| CPU Usage (Idle) | < 5% |

## Next Steps After Verification

1. **Cloud Deployment**
   - Follow: `MVP_TESTNET_LAUNCH.md` → Phase 4
   - Deploy to Render, AWS, or other cloud provider

2. **Production Hardening**
   - Security audit
   - Load testing (1000+ TPS)
   - Performance optimization

3. **Monitoring & Observability**
   - Setup logging aggregation
   - Add metrics collection
   - Create dashboards

4. **Documentation**
   - Create API specification
   - Write integration guides
   - Publish example contracts

---

**Completion Status**: `[ ] All phases complete`

**Date Completed**: _______________

**Completed By**: _______________

**Notes**: 

```
[Space for additional notes]
```

---

Once you've completed this checklist, your MVP testnet is production-ready! 🎉

For questions or issues, refer to the documentation in the `docs/` directory or the troubleshooting guide in `START_HERE_MVP_TESTNET.md`.
