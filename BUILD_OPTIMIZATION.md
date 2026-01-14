# Owami Network Build Optimization Guide

## Quick Start

### Fastest Build Method
```bash
# For development (fastest)
cargo build

# For production (optimized)
cargo build --release
```

## Build Optimization Strategies

### 1. Development Builds (Fastest)

```bash
# Basic development build
cargo build

# With incremental compilation (default)
cargo build --incremental

# Minimal optimization for faster builds
cargo build --profile dev
```

### 2. Production Builds (Optimized)

```bash
# Standard release build
cargo build --release

# Faster release build (recommended)
cargo build --release --profile release-fast

# Smallest binary size
cargo build --release --profile release-small
```

## Cargo Profile Configuration

Add these profiles to your `Cargo.toml`:

```toml
[profile.dev]
opt-level = 1      # Basic optimization
incremental = true  # Incremental compilation
debug = true       # Debug symbols

[profile.release]
opt-level = 3      # Full optimization
lto = "thin"       # Thin LTO for faster builds
codegen-units = 16 # Parallel code generation
panic = "abort"    # Smaller binaries
strip = false      # Keep symbols for debugging

[profile.release-fast]
inherits = "release"
opt-level = 2      # Slightly less optimization
lto = false        # No LTO for faster builds
codegen-units = 16 # Maximum parallelism

[profile.release-small]
inherits = "release"
opt-level = "z"    # Optimize for size
lto = true         # Full LTO for size
codegen-units = 1  # Single unit for best optimization
strip = true       # Remove symbols for smallest size
```

## Dependency Management

### Reduce Build Times

1. **Use pre-compiled dependencies**:
```bash
cargo update -p librocksdb-sys --precise 0.11.0+8.1.1
```

2. **Cache dependencies**:
```bash
mkdir -p ~/.cargo/registry
mkdir -p ~/.cargo/git
```

3. **Use cargo-cache**:
```bash
cargo install cargo-cache
cargo cache
```

## Common Build Issues & Solutions

### Issue: RocksDB Compilation Takes Too Long

**Solution 1**: Use pre-built RocksDB
```bash
# Add to Cargo.toml
[dependencies]
rocksdb = { version = "0.21", features = ["lz4"] }
```

**Solution 2**: Use a different database for development
```bash
# Create a feature flag in Cargo.toml
[features]
default = ["rocksdb"]
rocksdb = ["dep:rocksdb"]
sled = ["dep:sled"]

# Build without RocksDB
cargo build --no-default-features --features sled
```

### Issue: WASM Dependencies Slow Down Build

**Solution**: Disable WASM for development
```bash
# Build without WASM features
cargo build --no-default-features
```

### Issue: Linker Errors

**Solution**: Install required build tools
```bash
# On Ubuntu/Debian
sudo apt-get install build-essential pkg-config libssl-dev

# On Windows (using MSVC)
# Install Visual Studio with C++ tools
```

## Build Scripts

### Windows Build Script (build.bat)
```bat
@echo off
setlocal

:: Fast development build
if "%1"=="dev" (
    echo Building in development mode...
    cargo build --incremental
) else if "%1"=="release" (
    echo Building in release mode...
    cargo build --release --profile release-fast
) else (
    echo Usage: build.bat [dev|release]
)

endlocal
```

### Linux/Mac Build Script (build.sh)
```bash
#!/bin/bash

# Fast development build
if [ "$1" = "dev" ]; then
    echo "Building in development mode..."
    cargo build --incremental
    
# Optimized release build
elif [ "$1" = "release" ]; then
    echo "Building in release mode..."
    cargo build --release --profile release-fast
    
# Help
else
    echo "Usage: $0 [dev|release]"
fi
```

## Clean Builds

When you encounter strange build issues:

```bash
# Clean everything
cargo clean

# Remove target directory
rm -rf target/

# Clear cargo cache
cargo cache -a

# Rebuild from scratch
cargo build
```

## Cross-Compilation

### Build for Windows from Linux
```bash
rustup target add x86_64-pc-windows-gnu
cargo build --target x86_64-pc-windows-gnu
```

### Build for Linux from Windows
```bash
rustup target add x86_64-unknown-linux-gnu
cargo build --target x86_64-unknown-linux-gnu
```

## Continuous Integration Tips

### GitHub Actions Example
```yaml
name: Build

on: [push, pull_request]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v2
    - uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        override: true
    - name: Cache cargo
      uses: actions/cache@v2
      with:
        path: |
          ~/.cargo/registry
          ~/.cargo/git
          target
        key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
    - name: Build
      run: cargo build --release --profile release-fast
```

## Troubleshooting

### Build is too slow
1. Use `--profile release-fast` instead of `--release`
2. Disable LTO: `--config 'profile.release.lto = false'`
3. Increase codegen units: `--config 'profile.release.codegen-units = 16'`

### Out of memory
1. Reduce parallel jobs: `CARGO_BUILD_JOBS=4 cargo build`
2. Use less optimization: `--profile release-fast`
3. Build in debug mode first: `cargo build`

### Missing dependencies
1. Install system dependencies: `sudo apt-get install build-essential pkg-config libssl-dev`
2. Update Rust: `rustup update`
3. Clean and rebuild: `cargo clean && cargo build`

## Best Practices

1. **Development**: Use `cargo build` for fast iteration
2. **Testing**: Use `cargo test --profile dev` for faster tests
3. **Production**: Use `cargo build --release` for final builds
4. **CI/CD**: Use caching and `--profile release-fast` for faster builds
5. **Debugging**: Use `cargo build --profile dev` with debug symbols

## Environment Variables

```bash
# Faster builds
export CARGO_INCREMENTAL=1
export CARGO_PROFILE_DEV_OPT_LEVEL=1

# Parallel builds
export CARGO_BUILD_JOBS=8

# Build with environment variables
CARGO_INCREMENTAL=1 cargo build
```

## Summary

| Build Type | Command | Use Case |
|------------|---------|----------|
| Development | `cargo build` | Fast iteration during development |
| Release Fast | `cargo build --release --profile release-fast` | Faster production builds |
| Release Small | `cargo build --release --profile release-small` | Smallest binary size |
| Debug | `cargo build --profile dev` | Development with debug symbols |

Choose the appropriate build method based on your needs:
- **Development**: Fastest builds with `cargo build`
- **Testing**: Balanced speed with `cargo build --profile release-fast`
- **Production**: Optimized builds with `cargo build --release`