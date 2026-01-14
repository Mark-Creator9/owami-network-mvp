# Owami Network MVP Testnet Launcher
# PowerShell script for Windows

param(
    [string]$Mode = "local",  # "local" or "docker"
    [int]$Port = 8080,
    [string]$NodeId = "validator-1",
    [bool]$OpenBrowser = $false
)

Write-Host "╔════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║  Owami Network MVP Testnet Launcher   ║" -ForegroundColor Cyan
Write-Host "╚════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""

# Functions
function Check-Prerequisites {
    Write-Host "Checking prerequisites..." -ForegroundColor Yellow
    
    # Check Rust
    try {
        $rustVersion = cargo --version
        Write-Host "✓ $rustVersion" -ForegroundColor Green
    }
    catch {
        Write-Host "✗ Rust not found. Install from https://rustup.rs/" -ForegroundColor Red
        exit 1
    }
    
    # Check Docker (for docker mode)
    if ($Mode -eq "docker") {
        try {
            $dockerVersion = docker --version
            Write-Host "✓ $dockerVersion" -ForegroundColor Green
        }
        catch {
            Write-Host "✗ Docker not found. Install from https://www.docker.com/" -ForegroundColor Red
            exit 1
        }
    }
    
    Write-Host ""
}

function Build-Project {
    Write-Host "Building Owami Network..." -ForegroundColor Yellow
    Write-Host "(This may take 5-10 minutes on first build)" -ForegroundColor Gray
    
    cargo build --release
    
    if ($LASTEXITCODE -ne 0) {
        Write-Host "Build failed!" -ForegroundColor Red
        exit 1
    }
    
    Write-Host "✓ Build successful" -ForegroundColor Green
    Write-Host ""
}

function Launch-LocalNode {
    Write-Host "Launching local testnet node..." -ForegroundColor Yellow
    Write-Host "Node ID: $NodeId" -ForegroundColor Cyan
    Write-Host "Port: $Port" -ForegroundColor Cyan
    Write-Host "Mode: Single Node (local development)" -ForegroundColor Cyan
    Write-Host ""
    
    # Set environment variables
    $env:RUST_LOG = "info"
    $env:PORT = $Port
    $env:NODE_ID = $NodeId
    
    Write-Host "Starting server..." -ForegroundColor Yellow
    Write-Host "Endpoint: http://localhost:$Port" -ForegroundColor Cyan
    Write-Host ""
    
    if ($OpenBrowser) {
        Start-Process "http://localhost:$Port/api/health"
    }
    
    # Run the server
    cargo run --release -- --config config/testnet.toml
}

function Launch-DockerNodes {
    Write-Host "Launching multi-node Docker testnet..." -ForegroundColor Yellow
    Write-Host "Nodes: validator-1, validator-2, validator-3" -ForegroundColor Cyan
    Write-Host ""
    
    # Check if docker-compose.yml exists
    if (-not (Test-Path "docker-compose.yml")) {
        Write-Host "docker-compose.yml not found!" -ForegroundColor Red
        Write-Host "Creating docker-compose.yml..." -ForegroundColor Yellow
        Create-DockerCompose
    }
    
    Write-Host "Building Docker images..." -ForegroundColor Yellow
    docker-compose build
    
    if ($LASTEXITCODE -ne 0) {
        Write-Host "Docker build failed!" -ForegroundColor Red
        exit 1
    }
    
    Write-Host "Starting 3-node testnet..." -ForegroundColor Yellow
    docker-compose up
}

function Create-DockerCompose {
@"
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
"@ | Out-File -FilePath "docker-compose.yml" -Encoding UTF8
    
    Write-Host "✓ docker-compose.yml created" -ForegroundColor Green
}

function Show-Help {
    Write-Host "Owami MVP Testnet Launcher" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "Usage: .\launch-mvp-testnet.ps1 [options]" -ForegroundColor White
    Write-Host ""
    Write-Host "Options:" -ForegroundColor White
    Write-Host "  -Mode <local|docker>    Launch mode: local single node or Docker multi-node" -ForegroundColor Gray
    Write-Host "  -Port <number>          Port for local node (default: 8080)" -ForegroundColor Gray
    Write-Host "  -NodeId <string>        Node identifier (default: validator-1)" -ForegroundColor Gray
    Write-Host "  -OpenBrowser            Open health check in browser" -ForegroundColor Gray
    Write-Host ""
    Write-Host "Examples:" -ForegroundColor White
    Write-Host "  .\launch-mvp-testnet.ps1 -Mode local -OpenBrowser" -ForegroundColor Cyan
    Write-Host "  .\launch-mvp-testnet.ps1 -Mode docker" -ForegroundColor Cyan
    Write-Host "  .\launch-mvp-testnet.ps1 -Mode local -Port 8082 -NodeId validator-2" -ForegroundColor Cyan
    Write-Host ""
}

# Main execution
if ($args -contains "-help" -or $args -contains "help") {
    Show-Help
    exit 0
}

try {
    Check-Prerequisites
    
    # Only build for local mode (Docker build happens in docker-compose)
    if ($Mode -eq "local") {
        Build-Project
        Launch-LocalNode
    }
    elseif ($Mode -eq "docker") {
        Launch-DockerNodes
    }
    else {
        Write-Host "Invalid mode: $Mode" -ForegroundColor Red
        Write-Host "Use -Mode local or -Mode docker" -ForegroundColor Yellow
        exit 1
    }
}
catch {
    Write-Host "Error: $_" -ForegroundColor Red
    exit 1
}
finally {
    Write-Host ""
    Write-Host "Launcher finished." -ForegroundColor Gray
}
