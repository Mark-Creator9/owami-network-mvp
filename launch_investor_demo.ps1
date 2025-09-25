# Owami Network - Investor Demo Launch Script
# This script launches the Owami Network testnet for investor demonstration

param(
    [string]$Port = "3000",
    [switch]$OpenBrowser,
    [switch]$RunTests,
    [switch]$Verbose
)

Write-Host "🚀 OWAMI NETWORK - INVESTOR DEMO LAUNCHER" -ForegroundColor Magenta
Write-Host "==========================================" -ForegroundColor Magenta
Write-Host ""

# Set environment variables
$env:DATABASE_URL = "postgres://postgres:owamitest@localhost:5432/owami_network_test"
$env:PORT = $Port
$env:RUST_LOG = "info"
$env:JWT_SECRET = "owami-network-demo-jwt-secret-2024"

Write-Host "🔧 Configuration:" -ForegroundColor Cyan
Write-Host "   Database: PostgreSQL (owami_network_test)" -ForegroundColor Gray
Write-Host "   Port: $Port" -ForegroundColor Gray
Write-Host "   Environment: Investor Demo" -ForegroundColor Gray
Write-Host ""

# Check if PostgreSQL is running
Write-Host "🔍 Checking PostgreSQL connection..." -ForegroundColor Yellow
try {
    $testConnection = & psql $env:DATABASE_URL -c "SELECT 1;" 2>$null
    Write-Host "✅ PostgreSQL is running and accessible" -ForegroundColor Green
} catch {
    Write-Host "❌ PostgreSQL connection failed. Please ensure PostgreSQL is running." -ForegroundColor Red
    Write-Host "   Connection string: $env:DATABASE_URL" -ForegroundColor Gray
    exit 1
}

# Ensure database exists and run migrations
Write-Host "🗄️ Setting up database..." -ForegroundColor Yellow
try {
    & sqlx database create 2>$null
    & sqlx migrate run
    Write-Host "✅ Database setup complete" -ForegroundColor Green
} catch {
    Write-Host "⚠️ Database setup encountered issues (this may be normal)" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "🌟 LAUNCHING OWAMI NETWORK TESTNET..." -ForegroundColor Green
Write-Host "=====================================" -ForegroundColor Green
Write-Host ""
Write-Host "🔗 Access URLs:" -ForegroundColor Cyan
Write-Host "   Landing Page: http://localhost:$Port" -ForegroundColor White
Write-Host "   API Health:   http://localhost:$Port/api/health" -ForegroundColor White
Write-Host "   Blockchain:   http://localhost:$Port/api/blockchain/info" -ForegroundColor White
Write-Host "   Token Info:   http://localhost:$Port/api/token/info" -ForegroundColor White
Write-Host ""
Write-Host "📊 Key Features for Investors:" -ForegroundColor Cyan
Write-Host "   ✅ Layer-0 Blockchain Architecture" -ForegroundColor Green
Write-Host "   ✅ Native OWA Token System" -ForegroundColor Green
Write-Host "   ✅ DApp Development Platform" -ForegroundColor Green
Write-Host "   ✅ REST API for Integration" -ForegroundColor Green
Write-Host "   ✅ Enterprise Security (ED25519)" -ForegroundColor Green
Write-Host "   ✅ African Market Focus" -ForegroundColor Green
Write-Host ""

# Open browser if requested
if ($OpenBrowser) {
    Write-Host "🌐 Opening browser..." -ForegroundColor Yellow
    Start-Process "http://localhost:$Port"
}

Write-Host "🚀 Starting Owami Network Server..." -ForegroundColor Green
Write-Host "   Press Ctrl+C to stop the server" -ForegroundColor Gray
Write-Host ""

# Start the server
try {
    if ($Verbose) {
        $env:RUST_LOG = "debug"
    }
    
    & cargo run --release --bin owami-network
} catch {
    Write-Host "❌ Server startup failed: $($_.Exception.Message)" -ForegroundColor Red
    exit 1
}

# If we get here, the server was stopped
Write-Host ""
Write-Host "🛑 Owami Network Server Stopped" -ForegroundColor Yellow

# Run tests if requested
if ($RunTests) {
    Write-Host ""
    Write-Host "🧪 Running API Tests..." -ForegroundColor Cyan
    & .\test_api_endpoints.ps1 -BaseUrl "http://localhost:$Port" -Verbose:$Verbose
}

Write-Host ""
Write-Host "📋 Next Steps for Investors:" -ForegroundColor Magenta
Write-Host "   1. Review the landing page at http://localhost:$Port" -ForegroundColor White
Write-Host "   2. Check API documentation at /api-docs.html" -ForegroundColor White
Write-Host "   3. Test the blockchain functionality" -ForegroundColor White
Write-Host "   4. Review the investor presentation in /landing/INVESTOR_DEMO.md" -ForegroundColor White
Write-Host ""
Write-Host "💰 Investment Opportunity: $1M for African Blockchain Revolution" -ForegroundColor Green
Write-Host "🌍 Target: 100M+ Africans through USSD accessibility" -ForegroundColor Green
Write-Host ""