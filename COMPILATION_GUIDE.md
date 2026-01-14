# Owami Network Compilation Guide

## 🎯 Goal: Easy Compilation Without Complications

This guide provides step-by-step instructions to compile the Owami Network project with minimal issues.

## 🏗️ Step 1: Install Prerequisites

### Windows
```powershell
# 1. Install Rust (if not installed)
winget install Rustlang.Rustup

# 2. Install Visual Studio Build Tools
winget install Microsoft.VisualStudio.2022.BuildTools

# 3. Install Git (if not installed)
winget install Git.Git
```

### Linux (Ubuntu/Debian)
```bash
# 1. Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. Install build dependencies
sudo apt-get update
sudo apt-get install -y build-essential pkg-config libssl-dev cmake

# 3. Install Git (if not installed)
sudo apt-get install -y git
```

### Mac
```bash
# 1. Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. Install Xcode Command Line Tools
xcode-select --install

# 3. Install Git (if not installed)
brew install git
```

## 🔧 Step 2: Clone the Repository

```bash
git clone https://github.com/owami/owami-network.git
cd owami-network
```

## 🚀 Step 3: Build the Project

### Fastest Method (Development)
```bash
cargo build
```

### Optimized Method (Release)
```bash
cargo build --release
```

### Windows (Using Build Script)
```cmd
build.bat dev      # Fast development build
build.bat release  # Optimized release build
```

### Linux/Mac (Using Build Script)
```bash
chmod +x build.sh
./build.sh dev      # Fast development build
./build.sh release  # Optimized release build
```

## ⚙️ Step 4: Configuration

### Development Configuration
The project uses `config/testnet.toml` by default for development.

### Production Configuration
Create or edit `config/production.toml` for production settings.

## 🚀 Step 5: Run the Server

### Development Mode
```bash
cargo run
```

### Release Mode
```bash
cargo run --release
```

### With Custom Port
```bash
PORT=8081 cargo run --release
```

## 🧪 Step 6: Test the Server

### Check Health
```bash
curl http://localhost:8081/api/health
```

### Create Wallet
```bash
curl -X POST http://localhost:8081/api/wallet/create \
  -H "Content-Type: application/json" \
  -d '{"username":"testuser","password":"testpass"}'
```

### Get Blockchain Info
```bash
curl http://localhost:8081/api/blockchain/info
```

## 🔍 Troubleshooting

### Build Takes Too Long
**Solution**: Use development build first
```bash
cargo build  # Fast development build
```

### Missing Dependencies
**Solution**: Install required packages
```bash
# Windows
winget install Microsoft.VisualStudio.2022.BuildTools

# Linux
sudo apt-get install build-essential pkg-config libssl-dev
```

### RocksDB Compilation Issues
**Solution**: Use pre-built version or clean build
```bash
cargo clean
cargo build
```

### Port Already in Use
**Solution**: Change port or kill existing process
```bash
# Find process (Linux/Mac)
lsof -i :8081

# Kill process (Linux/Mac)
kill -9 <PID>

# Windows
netstat -ano | findstr 8081
taskkill /PID <PID> /F
```

### Out of Memory
**Solution**: Reduce parallel jobs
```bash
CARGO_BUILD_JOBS=4 cargo build
```

## 📚 Build Optimization Tips

### 1. Use Development Profile
```bash
cargo build --profile dev
```

### 2. Disable LTO for Faster Builds
```bash
cargo build --release --config 'profile.release.lto = false'
```

### 3. Increase Codegen Units
```bash
cargo build --release --config 'profile.release.codegen-units = 16'
```

### 4. Use Thin LTO
```bash
cargo build --release --config 'profile.release.lto = "thin"'
```

### 5. Cache Dependencies
```bash
mkdir -p ~/.cargo/registry
mkdir -p ~/.cargo/git
```

## 🎯 Best Practices

### 1. Development Workflow
```bash
# Fast iteration
cargo build      # Fast build
cargo run        # Run server
# Make changes
# Repeat
```

### 2. Testing Workflow
```bash
cargo test --profile dev  # Fast tests
cargo build --release      # Optimized build
cargo test --release       # Full tests
```

### 3. Production Workflow
```bash
cargo clean                # Clean build
cargo build --release      # Optimized build
./target/release/owami-server # Run production server
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
├── config/             # Configuration files
│   ├── testnet.toml     # Testnet configuration
│   └── production.toml  # Production configuration
├── build.bat           # Windows build script
├── build.sh            # Linux/Mac build script
├── QUICK_START.md      # Quick start guide
├── BUILD_OPTIMIZATION.md # Build optimization tips
└── COMPILATION_GUIDE.md # This guide
```

## 🚀 Deployment

### Local Deployment
```bash
cargo build --release
./target/release/owami-server
```

### Production Deployment
```bash
# Build
cargo build --release

# Set environment
export PORT=8080
export CONFIG_PATH=config/production.toml

# Run
./target/release/owami-server
```

### Systemd Service (Linux)
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

## 🎓 Learning Resources

- **Rust Documentation**: https://doc.rust-lang.org/
- **Cargo Book**: https://doc.rust-lang.org/cargo/
- **Axum Web Framework**: https://docs.rs/axum
- **RocksDB**: https://rocksdb.org/

## 🤝 Community & Support

- **GitHub Issues**: https://github.com/owami/owami-network/issues
- **Documentation**: https://docs.owami.network
- **Community Forum**: https://forum.owami.network

## 🎉 Summary

### Quick Commands
```bash
# Install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone
git clone https://github.com/owami/owami-network.git
cd owami-network

# Build
cargo build

# Run
cargo run

# Test
curl http://localhost:8081/api/health
```

### Build Times
- **Development**: 1-5 minutes (depending on system)
- **Release**: 5-15 minutes (first time)
- **Subsequent builds**: 30-60 seconds (incremental)

### System Requirements
- **RAM**: 4GB minimum, 8GB recommended
- **Disk**: 10GB free space
- **CPU**: 2+ cores
- **OS**: Windows 10+, Linux, Mac

Follow these steps and you'll have the Owami Network project compiled and running smoothly! 🚀