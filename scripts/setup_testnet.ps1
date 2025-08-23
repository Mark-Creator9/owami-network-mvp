# Owami Network Testnet Setup Script
# Author: Mark Creator9
# This script sets up the testnet environment

param(
    [string]$DatabaseUrl = "postgres://localhost/owami_testnet",
    [string]$Port = "3000"
)

Write-Host "Setting up Owami Network Testnet..." -ForegroundColor Green

# Check if PostgreSQL is running
try {
    $pgProcess = Get-Process -Name "postgres" -ErrorAction Stop
    Write-Host "PostgreSQL is running" -ForegroundColor Green
} catch {
    Write-Host "PostgreSQL is not running. Please start PostgreSQL first." -ForegroundColor Red
    exit 1
}

# Create database if it doesn't exist
Write-Host "Creating database..." -ForegroundColor Yellow
psql -U postgres -c "CREATE DATABASE owami_testnet;" 2>$null || Write-Host "Database already exists or user lacks permissions"

# Set environment variables
$env:DATABASE_URL = $DatabaseUrl
$env:PORT = $Port

# Run migrations
Write-Host "Running database migrations..." -ForegroundColor Yellow
sqlx migrate run --database-url $DatabaseUrl

# Build the project
Write-Host "Building project..." -ForegroundColor Yellow
cargo build --release --bin owami-network-simplified

if ($LASTEXITCODE -ne 0) {
    Write-Host "Build failed. Please check the error messages above." -ForegroundColor Red
    exit 1
}

Write-Host "Build completed successfully!" -ForegroundColor Green
Write-Host "Starting testnet server..." -ForegroundColor Green
Write-Host "Server will be available at: http://localhost:$Port" -ForegroundColor Cyan
Write-Host "API endpoints:" -ForegroundColor Cyan
Write-Host "  GET  /api/blockchain/info" -ForegroundColor Gray
Write-Host "  GET  /api/blockchain/blocks" -ForegroundColor Gray
Write-Host "  POST /api/blockchain/mine" -ForegroundColor Gray
Write-Host "  GET  /api/token/balance/:address" -ForegroundColor Gray
Write-Host "  POST /api/token/transfer" -ForegroundColor Gray
Write-Host "  POST /api/token/mint/:address" -ForegroundColor Gray
Write-Host "  GET  /api/dapps" -ForegroundColor Gray
Write-Host "  POST /api/dapps" -ForegroundColor Gray
Write-Host "  GET  /api/dapps/:id" -ForegroundColor Gray

# Start the server
cargo run --bin owami-network-simplified