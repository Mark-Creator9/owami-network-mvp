# Owami Network Quick Start Guide

## 🚀 Fastest Way to Get Started

### 1. Install Prerequisites

**Windows:**
```powershell
# Install Rust (if not already installed)
winget install Rustlang.Rustup

# Install build tools
winget install Microsoft.VisualStudio.2022.BuildTools
```

**Linux/Mac:**
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install build dependencies
sudo apt-get install build-essential pkg-config libssl-dev cmake
```

### 2. Build the Project

**Fast Development Build:**
```bash
cargo build
```

**Optimized Release Build:**
```bash
cargo build --release
```

**Windows (using build script):**
```cmd
build.bat dev      # Development build
build.bat release  # Release build
```

### 3. Run the Server

```bash
cargo run --release
```

The server will start on:
- **Frontend**: http://localhost:8081
- **API**: http://localhost:8081/api

### 4. Test the API

```bash
# Check health
curl http://localhost:8081/api/health

# Create wallet
curl -X POST http://localhost:8081/api/wallet/create \
  -H "Content-Type: application/json" \
  -d '{"username":"testuser","password":"testpass"}'
```

## 📦 Project Structure

```
owami-network/
├── Cargo.toml          # Rust project configuration
├── src/                # Source code
│   ├── main.rs          # Main entry point
│   ├── lib.rs           # Library code
│   └── ...              # Other modules
├── landing/            # Frontend files
│   ├── index.html       # Main landing page
│   ├── modern-index.html # Modern UI with 3D effects
│   ├── css/             # Stylesheets
│   └── js/              # JavaScript
├── build.bat           # Windows build script
├── build.sh            # Linux/Mac build script
├── QUICK_START.md      # This guide
└── BUILD_OPTIMIZATION.md # Build optimization tips
```

## 🔧 Common Issues & Solutions

### Build takes too long
- **Solution**: Use development build first
  ```bash
  cargo build  # Fast development build
  ```

### Missing dependencies
- **Solution**: Install build tools
  ```bash
  # Windows
  winget install Microsoft.VisualStudio.2022.BuildTools
  
  # Linux
  sudo apt-get install build-essential pkg-config libssl-dev
  ```

### Port already in use
- **Solution**: Change port or kill existing process
  ```bash
  # Find process
  lsof -i :8081
  
  # Kill process (Linux/Mac)
  kill -9 <PID>
  
  # Windows
  taskkill /PID <PID> /F
  ```

### RocksDB compilation slow
- **Solution**: Use pre-built version or disable for development
  ```bash
  # Temporarily disable RocksDB (if available)
  cargo build --no-default-features
  ```

## 🎯 Development Workflow

### 1. Make Changes
Edit files in `src/` or `landing/` directories.

### 2. Test Changes
```bash
# Fast test
cargo build

# Run tests
cargo test
```

### 3. Run Server
```bash
cargo run
```

### 4. Iterate
Make more changes and repeat!

## 🚀 Production Deployment

### Build for Production
```bash
cargo build --release
```

### Run in Production
```bash
# Set environment variables
export PORT=8080
export CONFIG_PATH=config/production.toml

# Run server
./target/release/owami-server
```

### Use Systemd (Linux)
```ini
# /etc/systemd/system/owami.service
[Unit]
Description=Owami Network Server
After=network.target

[Service]
User=owami
WorkingDirectory=/opt/owami-network
ExecStart=/opt/owami-network/target/release/owami-server
Restart=always
Environment=PORT=8080
Environment=CONFIG_PATH=config/production.toml

[Install]
WantedBy=multi-user.target
```

## 📚 Learning Resources

- **Rust Documentation**: https://doc.rust-lang.org/
- **Axum Web Framework**: https://docs.rs/axum
- **RocksDB**: https://rocksdb.org/
- **libp2p**: https://libp2p.io/

## 🤝 Community

- **GitHub Issues**: https://github.com/owami/owami-network/issues
- **Documentation**: https://docs.owami.network
- **Community Forum**: https://forum.owami.network

## 🎉 Next Steps

1. **Explore the code**: Look at `src/main.rs` to understand the server
2. **Try the API**: Test endpoints with `curl` or Postman
3. **Build a DApp**: Create your first decentralized application
4. **Contribute**: Submit pull requests to improve the project

Happy coding! 🚀