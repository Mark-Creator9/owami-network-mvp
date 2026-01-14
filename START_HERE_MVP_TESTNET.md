# 🚀 START HERE: MVP Testnet Launch

Welcome to the Owami Network MVP Testnet! This document will get you started in 5 minutes.

## ⚡ 5-Minute Quick Start

### Step 1: Prerequisites Check (1 min)
```bash
# Check if Rust is installed
rustc --version
cargo --version

# If not installed, go to: https://rustup.rs/
```

### Step 2: Launch Testnet (4 min)
Choose your preferred option:

#### 🔵 Option A: Single Node (Easiest)
**Windows (PowerShell):**
```powershell
cd c:\Users\HP\Desktop\projects\owami-network
.\launch-mvp-testnet.ps1 -Mode local -OpenBrowser
```

**macOS/Linux:**
```bash
cd ~/projects/owami-network
chmod +x launch-mvp-testnet.sh
./launch-mvp-testnet.sh --mode local --open-browser
```

**Manual Build:**
```bash
cargo run --release -- --config config/testnet.toml
```

#### 🟦 Option B: 3-Node Testnet (Requires Docker)
**Windows (PowerShell):**
```powershell
.\launch-mvp-testnet.ps1 -Mode docker
```

**macOS/Linux:**
```bash
./launch-mvp-testnet.sh --mode docker
```

**Manual:**
```bash
docker-compose up --build
```

### Step 3: Verify It Works (Instant)
```bash
# Test the health endpoint
curl http://localhost:8080/api/health

# Expected response:
# {"status": "healthy", ...}
```

**🎉 Success!** Your testnet is running!

## 📖 Next: Learn What You Can Do

### Test 1: Check Blockchain Info
```bash
curl http://localhost:8080/api/blockchain/info
```

### Test 2: Mine a Block
```bash
curl -X POST http://localhost:8080/api/blockchain/mine
```

### Test 3: Deploy a Smart Contract
```bash
curl -X POST http://localhost:8080/api/contracts/deploy \
  -H "Content-Type: application/json" \
  -d '{
    "wasm_bytecode": "0x00asm0100000007606001ff01ff03020100050301000a",
    "creator": "demo_address",
    "contract_type": "test"
  }'
```

### Test 4: List Contracts
```bash
curl http://localhost:8080/api/contracts/list
```

## 🗺️ Documentation Roadmap

Once you've verified the testnet is running, explore:

| Document | Purpose | Time |
|----------|---------|------|
| `MVP_TESTNET_SUMMARY.md` | Overview of features and checklist | 5 min |
| `MVP_TESTNET_LAUNCH.md` | Detailed launch guide | 15 min |
| `docs/SMART_CONTRACT_API.md` | Smart contract API reference | 20 min |
| `docs/QUICKSTART.md` | Quick start guide | 10 min |
| `docs/TESTNET_GUIDE.md` | Testnet configuration | 5 min |

## 🎯 Common Tasks

### I want to deploy a real smart contract
1. See: `examples/simple_counter.rs` and `examples/SimpleToken.sol`
2. Compile to WASM bytecode
3. Convert to hex
4. Deploy using the API

### I want to run multiple nodes
1. Use Docker Compose: `docker-compose up --build`
2. Or follow: `MVP_TESTNET_LAUNCH.md` → Phase 2

### I want to deploy to the cloud
1. Follow: `MVP_TESTNET_LAUNCH.md` → Phase 4
2. Uses Render.com or any Docker-compatible platform

### I have questions about the API
1. Check: `docs/SMART_CONTRACT_API.md`
2. Try: Postman collection in `postman/`

## 🐛 Troubleshooting

### "Port already in use"
```bash
# Windows: Kill process on port 8080
netstat -ano | findstr :8080
taskkill /PID <PID> /F

# macOS/Linux:
lsof -i :8080
kill -9 <PID>
```

### "Build failed"
```bash
# Clean and rebuild
cargo clean
cargo build --release
```

### "Connection refused"
- Make sure the server is running (check previous terminal)
- Wait 5-10 seconds for startup
- Try again

### Docker not found
- Install Docker Desktop: https://www.docker.com/products/docker-desktop
- Restart your terminal after installation

## 📊 What's Running

After launch, you have:

| Component | Port | Purpose |
|-----------|------|---------|
| REST API | 8080 | Deploy contracts, call functions |
| P2P Network | 4001 | Node communication |
| Health Check | 8080 | System status |

## 🔄 Multi-Node Network (Docker Only)

If using Docker Compose with 3 nodes:
- **Validator 1**: http://localhost:8080
- **Validator 2**: http://localhost:8081
- **Validator 3**: http://localhost:8082

All nodes sync automatically and have the same blockchain state.

## 🚀 Next Steps

### Immediate
- [ ] Verify testnet is running
- [ ] Test health endpoint
- [ ] Read `MVP_TESTNET_SUMMARY.md`

### Short Term (Today)
- [ ] Deploy a test contract
- [ ] Call contract functions
- [ ] Test with 3-node Docker setup

### Medium Term (This Week)
- [ ] Deploy to cloud
- [ ] Create example contracts
- [ ] Load test the network

### Long Term (Production)
- [ ] Security audit
- [ ] Performance optimization
- [ ] Mainnet preparation

## 💡 Pro Tips

1. **Use Postman**: Import `postman/Owami.postman_collection.json`
2. **Check Logs**: `RUST_LOG=debug cargo run --release` for verbose output
3. **Save API Calls**: Create a bash script with your frequent curl commands
4. **Monitor Nodes**: Keep a terminal with `watch curl http://localhost:8080/api/health`
5. **Docker Logs**: `docker logs owami-validator-1 --follow`

## 🎓 Learning Resources

- **Blockchain Basics**: See `docs/OVERVIEW.md`
- **Smart Contracts**: See `docs/SMART_CONTRACT_API.md`
- **Network Architecture**: See `NEXT_STEPS.md`
- **Examples**: See `examples/` directory

## ❓ Quick Questions

**Q: How do I stop the testnet?**
A: Press `Ctrl+C` in the terminal (single node) or `Ctrl+C` then `docker-compose down` (Docker)

**Q: Can I run multiple testnets?**
A: Yes, change the port: `cargo run --release -- --config config/testnet.toml --port 9000`

**Q: How do I deploy to production?**
A: Follow `MVP_TESTNET_LAUNCH.md` → Phase 4 (Cloud Deployment)

**Q: Where are my contract bytecodes stored?**
A: In RocksDB (local) or Docker volumes if using Docker Compose

**Q: Can I modify testnet config?**
A: Yes, edit `config/testnet.toml` before launching

## 📞 Need Help?

1. Check `docs/TROUBLESHOOTING.md`
2. Review relevant documentation from the Roadmap above
3. Check console logs for error messages
4. Try a clean rebuild: `cargo clean && cargo build --release`

## 🎉 You're All Set!

You now have a fully functional MVP testnet with:
- ✅ Smart contract support
- ✅ Multi-node consensus
- ✅ REST API
- ✅ Token system
- ✅ P2P networking

**Happy testing!** 🚀

---

**Still here?** 

Follow the steps above to launch. Come back to this file if you need help.

Once running, open: `http://localhost:8080/api/health`

Good luck! 🌟
