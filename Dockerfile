# Owami Network Testnet MVP - Production Deployment

# ========= Build stage =========
FROM rust:1.77-slim as builder

# Install build dependencies including libclang for bindgen
RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    clang \
    libclang-dev \
    llvm-dev \
    ca-certificates \
    curl \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source
COPY src ./src
COPY config ./config

# Copy landing directory for frontend
COPY landing ./landing

# Build release binary for owami-server
RUN cargo build --release --bin owami-server

# ========= Runtime stage =========
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
 && rm -rf /var/lib/apt/lists/*

# Create data directory for RocksDB
RUN mkdir -p /app/data

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/owami-server /usr/local/bin/owami-server

# Copy config from builder
COPY --from=builder /app/config ./config

# Copy landing directory for frontend
COPY --from=builder /app/landing ./landing

# Environment:
# - CONFIG_PATH: which config file to use (testnet/production)
# - PORT: HTTP API port
ENV CONFIG_PATH=/app/config/testnet.toml
ENV PORT=8081
ENV HOST=0.0.0.0
ENV RUST_LOG=info

EXPOSE 8081

# Create data directory on runtime
RUN mkdir -p /app/data/rocksdb

# Healthcheck against Owami health endpoint
HEALTHCHECK --interval=30s --timeout=5s --retries=3 \
    CMD curl -f http://localhost:${PORT}/health || exit 1

CMD ["/usr/local/bin/owami-server"]
