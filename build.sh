#!/bin/bash

# Owami Network Build Script
# This script optimizes the build process for faster compilation

echo "🚀 Starting Owami Network Build Process"
echo "========================================"

# Set environment variables for optimized build
export RUSTFLAGS="-C target-cpu=native"

# Check if we're on Windows (for Git Bash)
if [[ "$OSTYPE" == "msys" ]]; then
    echo "📋 Windows detected - using optimized settings"
    export CARGO_INCREMENTAL=1
    export CARGO_PROFILE_DEV_OPT_LEVEL=1
fi

# Build with optimized settings
echo "🔨 Building project with optimized settings..."
cargo build --release --features "" \
    --config 'profile.release.lto = "thin"' \
    --config 'profile.release.codegen-units = 16' \
    --config 'profile.release.strip = false'

# Check if build succeeded
if [ $? -eq 0 ]; then
    echo "✅ Build completed successfully!"
    echo "========================================"
    echo "📁 Binary location: target/release/owami-server"
    echo "🚀 To run: cargo run --release"
else
    echo "❌ Build failed!"
    echo "========================================"
    echo "🔍 Try these troubleshooting steps:"
    echo "1. cargo clean"
    echo "2. cargo update"
    echo "3. Check for missing dependencies"
    exit 1
fi