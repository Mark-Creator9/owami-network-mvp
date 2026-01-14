# Owami Network Testnet MVP - Single Node + API

# ========= Build stage =========
FROM rust:1.77-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    clang \
    ca-certificates \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source
COPY src ./src
COPY config ./config

# Build release binary for the main entrypoint
# Ensure Cargo.toml [[bin]] points "owami-network" to src/main_simple.rs or your chosen main.
RUN cargo build --release --bin owami-network

# ========= Runtime stage =========
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y ca-certificates curl \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary and config from builder
COPY --from=builder /app/target/release/owami-network /usr/local/bin/owami-network
COPY --from=builder /app/config ./config

# Environment:
# - CONFIG_PATH: which config file to use (testnet/production)
# - PORT: HTTP API port
ENV CONFIG_PATH=/app/config/production.toml
ENV PORT=8080

EXPOSE 8080

# Healthcheck against Owami health endpoint
HEALTHCHECK --interval=30s --timeout=5s --retries=3 CMD curl -f http://localhost:${PORT}/api/health || exit 1

CMD ["/usr/local/bin/owami-network"]
