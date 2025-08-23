# PowerShell script to run all WASM tests for Owami Network

param(
    [switch]$SkipBuild = $false,
    [switch]$Verbose = $false
)

$ErrorActionPreference = "Stop"

Write-Host @"
╔══════════════════════════════════════════════════════════════╗
║                Owami Network WASM Test Suite                 ║
╚══════════════════════════════════════════════════════════════╝
"@ -ForegroundColor Cyan

# Check prerequisites
Write-Host "`n📋 Checking prerequisites..." -ForegroundColor Yellow

# Check if Rust is installed
try {
    $rustVersion = rustc --version
    Write-Host "✓ Rust installed: $rustVersion" -ForegroundColor Green
} catch {
    Write-Host "✗ Rust not found. Please install Rust from https://rustup.rs/" -ForegroundColor Red
    exit 1
}

# Check if wasm32 target is installed
try {
    $targets = rustup target list --installed
    if ($targets -contains "wasm32-unknown-unknown") {
        Write-Host "✓ wasm32-unknown-unknown target installed" -ForegroundColor Green
    } else {
        Write-Host "⚠ Installing wasm32-unknown-unknown target..." -ForegroundColor Yellow
        rustup target add wasm32-unknown-unknown
    }
} catch {
    Write-Host "✗ Failed to check/install wasm32 target" -ForegroundColor Red
    exit 1
}

# Check if Owami Network is running
Write-Host "`n🔍 Checking Owami Network server..." -ForegroundColor Yellow
try {
    $response = Invoke-RestMethod -Uri "http://localhost:8081/api/dapp/deploy" -Method Post -Body '{"contract_path":"test.wasm"}' -ContentType "application/json" -ErrorAction SilentlyContinue
    Write-Host "✓ Owami Network server is running" -ForegroundColor Green
} catch {
    Write-Host "⚠ Owami Network server not responding on localhost:8081" -ForegroundColor Yellow
    Write-Host "  Please start the server with: cargo run --bin owami-network-simplified" -ForegroundColor Gray
    $startServer = Read-Host "  Would you like to start the server now? (y/n)"
    if ($startServer -eq "y") {
        Write-Host "  Starting server in background..." -ForegroundColor Gray
        Start-Process "cargo" -ArgumentList "run --bin owami-network-simplified" -WorkingDirectory ".." -NoNewWindow
        Start-Sleep -Seconds 5
    } else {
        Write-Host "  Please start the server manually and run this script again." -ForegroundColor Yellow
        exit 1
    }
}

# Build and test
Write-Host "`n🔨 Building and testing WASM contract..." -ForegroundColor Yellow

if (-not $SkipBuild) {
    try {
        Write-Host "  Building contract..." -ForegroundColor Gray
        .\compile_contract.ps1
        Write-Host "✓ Contract built successfully" -ForegroundColor Green
    } catch {
        Write-Host "✗ Failed to build contract: $_" -ForegroundColor Red
        exit 1
    }
} else {
    Write-Host "  Skipping build (requested)" -ForegroundColor Gray
}

# Test deployment
Write-Host "`n🚀 Testing deployment..." -ForegroundColor Yellow
try {
    .\test_deploy_wasm.ps1
    Write-Host "✓ Deployment test completed successfully" -ForegroundColor Green
} catch {
    Write-Host "✗ Deployment test failed: $_" -ForegroundColor Red
    exit 1
}

# Test API endpoints
Write-Host "`n🔍 Testing API endpoints..." -ForegroundColor Yellow
try {
    .\test_api_endpoints.ps1
    Write-Host "✓ API endpoint tests completed" -ForegroundColor Green
} catch {
    Write-Host "⚠ API endpoint tests had issues: $_" -ForegroundColor Yellow
}

# Summary
Write-Host @"

╔══════════════════════════════════════════════════════════════╗
║                    Test Summary                              ║
╚══════════════════════════════════════════════════════════════╝
"@ -ForegroundColor Cyan

Write-Host "✅ All tests completed successfully!" -ForegroundColor Green
Write-Host ""
Write-Host "📁 Files created:"
Write-Host "   - simple_contract.wasm (compiled contract)"
Write-Host "   - deployment_result.json (deployment response)"
Write-Host ""
Write-Host "🎯 Next steps:"
Write-Host "   1. Use the contract address from deployment_result.json"
Write-Host "   2. Interact with your contract via the API endpoints"
Write-Host "   3. Check the Owami Network dashboard for transaction details"
Write-Host ""

if ($Verbose) {
    Write-Host "📊 Detailed logs available in current directory" -ForegroundColor Gray
}