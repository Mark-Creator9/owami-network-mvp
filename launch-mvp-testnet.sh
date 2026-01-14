#!/bin/bash

# Owami Network MVP Testnet Launcher
# Bash script for macOS and Linux

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Default parameters
MODE="local"
PORT=8080
NODE_ID="validator-1"
OPEN_BROWSER=false

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -m|--mode)
            MODE="$2"
            shift 2
            ;;
        -p|--port)
            PORT="$2"
            shift 2
            ;;
        -n|--node-id)
            NODE_ID="$2"
            shift 2
            ;;
        -b|--open-browser)
            OPEN_BROWSER=true
            shift
            ;;
        -h|--help)
            show_help
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            show_help
            exit 1
            ;;
    esac
done

# Functions
show_help() {
    echo -e "${CYAN}Owami Network MVP Testnet Launcher${NC}"
    echo ""
    echo "Usage: ./launch-mvp-testnet.sh [options]"
    echo ""
    echo "Options:"
    echo "  -m, --mode <local|docker>    Launch mode (default: local)"
    echo "  -p, --port <number>          Port for local node (default: 8080)"
    echo "  -n, --node-id <string>       Node identifier (default: validator-1)"
    echo "  -b, --open-browser           Open health check in browser"
    echo "  -h, --help                   Show this help message"
    echo ""
    echo "Examples:"
    echo "  ./launch-mvp-testnet.sh --mode local --open-browser"
    echo "  ./launch-mvp-testnet.sh --mode docker"
    echo "  ./launch-mvp-testnet.sh --mode local --port 8082 --node-id validator-2"
}

check_prerequisites() {
    echo -e "${YELLOW}Checking prerequisites...${NC}"
    
    # Check Rust
    if command -v cargo &> /dev/null; then
        RUST_VERSION=$(cargo --version)
        echo -e "${GREEN}✓ $RUST_VERSION${NC}"
    else
        echo -e "${RED}✗ Rust not found. Install from https://rustup.rs/${NC}"
        exit 1
    fi
    
    # Check Docker (for docker mode)
    if [ "$MODE" = "docker" ]; then
        if command -v docker &> /dev/null; then
            DOCKER_VERSION=$(docker --version)
            echo -e "${GREEN}✓ $DOCKER_VERSION${NC}"
        else
            echo -e "${RED}✗ Docker not found. Install from https://www.docker.com/${NC}"
            exit 1
        fi
    fi
    
    echo ""
}

build_project() {
    echo -e "${YELLOW}Building Owami Network...${NC}"
    echo -e "${CYAN}(This may take 5-10 minutes on first build)${NC}"
    echo ""
    
    cargo build --release
    
    if [ $? -ne 0 ]; then
        echo -e "${RED}Build failed!${NC}"
        exit 1
    fi
    
    echo -e "${GREEN}✓ Build successful${NC}"
    echo ""
}

launch_local_node() {
    echo -e "${YELLOW}Launching local testnet node...${NC}"
    echo -e "${CYAN}Node ID: $NODE_ID${NC}"
    echo -e "${CYAN}Port: $PORT${NC}"
    echo -e "${CYAN}Mode: Single Node (local development)${NC}"
    echo ""
    
    # Export environment variables
    export RUST_LOG=info
    export PORT=$PORT
    export NODE_ID=$NODE_ID
    
    echo -e "${YELLOW}Starting server...${NC}"
    echo -e "${CYAN}Endpoint: http://localhost:$PORT${NC}"
    echo ""
    
    if [ "$OPEN_BROWSER" = true ]; then
        if command -v open &> /dev/null; then
            # macOS
            open "http://localhost:$PORT/api/health"
        elif command -v xdg-open &> /dev/null; then
            # Linux
            xdg-open "http://localhost:$PORT/api/health"
        fi
    fi
    
    # Run the server
    cargo run --release -- --config config/testnet.toml
}

launch_docker_nodes() {
    echo -e "${YELLOW}Launching multi-node Docker testnet...${NC}"
    echo -e "${CYAN}Nodes: validator-1, validator-2, validator-3${NC}"
    echo ""
    
    # Check if docker-compose.yml exists
    if [ ! -f "docker-compose.yml" ]; then
        echo -e "${YELLOW}Creating docker-compose.yml...${NC}"
        create_docker_compose
    fi
    
    echo -e "${YELLOW}Building Docker images...${NC}"
    docker-compose build
    
    if [ $? -ne 0 ]; then
        echo -e "${RED}Docker build failed!${NC}"
        exit 1
    fi
    
    echo -e "${YELLOW}Starting 3-node testnet...${NC}"
    docker-compose up
}

create_docker_compose() {
    cat > docker-compose.yml << 'EOF'
version: '3.8'

services:
  owami-validator-1:
    build: .
    ports:
      - "8080:8080"
      - "4001:4001"
    environment:
      NODE_ID: validator-1
      PORT: 8080
      BOOTSTRAP_PEERS: ""
      RUST_LOG: info
    volumes:
      - ./data/node1:/var/data

  owami-validator-2:
    build: .
    ports:
      - "8081:8080"
      - "4002:4001"
    environment:
      NODE_ID: validator-2
      PORT: 8080
      BOOTSTRAP_PEERS: "/dns4/owami-validator-1/tcp/4001"
      RUST_LOG: info
    volumes:
      - ./data/node2:/var/data
    depends_on:
      - owami-validator-1

  owami-validator-3:
    build: .
    ports:
      - "8082:8080"
      - "4003:4001"
    environment:
      NODE_ID: validator-3
      PORT: 8080
      BOOTSTRAP_PEERS: "/dns4/owami-validator-1/tcp/4001"
      RUST_LOG: info
    volumes:
      - ./data/node3:/var/data
    depends_on:
      - owami-validator-1
EOF
    
    echo -e "${GREEN}✓ docker-compose.yml created${NC}"
}

# Banner
echo -e "${CYAN}╔════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║  Owami Network MVP Testnet Launcher   ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════╝${NC}"
echo ""

# Main execution
check_prerequisites

if [ "$MODE" = "local" ]; then
    build_project
    launch_local_node
elif [ "$MODE" = "docker" ]; then
    launch_docker_nodes
else
    echo -e "${RED}Invalid mode: $MODE${NC}"
    echo -e "${YELLOW}Use --mode local or --mode docker${NC}"
    exit 1
fi
