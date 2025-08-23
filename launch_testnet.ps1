param(
    [switch]$SkipDatabase,
    [switch]$SkipBuild,
    [switch]$SkipFrontend,
    [int]$Port = 3000
)

# Colors for output
$Green = "`e[32m"
$Yellow = "`e[33m"
$Red = "`e[31m"
$Reset = "`e[0m"

Write-Host "${Green}🚀 OWami Network Simplified Testnet Launcher${Reset}" -ForegroundColor Green
Write-Host "=========================================="

# Check if PostgreSQL is running
if (-not $SkipDatabase) {
    Write-Host "${Yellow}🔍 Checking PostgreSQL...${Reset}" -ForegroundColor Yellow
    try {
        $pgService = Get-Service -Name "postgresql*" -ErrorAction SilentlyContinue
        if ($pgService -and $pgService.Status -eq "Running") {
            Write-Host "${Green}✅ PostgreSQL is running${Reset}" -ForegroundColor Green
        } else {
            Write-Host "${Yellow}🔄 Starting PostgreSQL...${Reset}" -ForegroundColor Yellow
            Start-Service -Name "postgresql*" -ErrorAction SilentlyContinue
            Start-Sleep -Seconds 3
        }
    } catch {
        Write-Host "${Yellow}⚠️  Could not check PostgreSQL service. Make sure PostgreSQL is running.${Reset}" -ForegroundColor Yellow
    }
}

# Set environment variables
$env:DATABASE_URL = "postgres://postgres:postgres@localhost/owami_testnet"
$env:PORT = $Port.ToString()

# Create database if it doesn't exist
if (-not $SkipDatabase) {
    Write-Host "${Yellow}🗄️  Setting up database...${Reset}" -ForegroundColor Yellow
    try {
        # Check if database exists
        $dbExists = psql -U postgres -d postgres -tAc "SELECT 1 FROM pg_database WHERE datname='owami_testnet'" 2>$null
        if ($dbExists -ne "1") {
            Write-Host "${Yellow}📦 Creating database 'owami_testnet'...${Reset}" -ForegroundColor Yellow
            createdb -U postgres owami_testnet
            if ($LASTEXITCODE -ne 0) {
                Write-Host "${Red}❌ Failed to create database. Make sure PostgreSQL is running and you have permissions.${Reset}" -ForegroundColor Red
                exit 1
            }
        } else {
            Write-Host "${Green}✅ Database 'owami_testnet' already exists${Reset}" -ForegroundColor Green
        }
    } catch {
        Write-Host "${Red}❌ Database setup failed. Make sure PostgreSQL is installed and running.${Reset}" -ForegroundColor Red
        Write-Host "${Yellow}💡 Try running: pg_ctl -D `"C:\Program Files\PostgreSQL\*\data`" start${Reset}" -ForegroundColor Yellow
        exit 1
    }
}

# Build the project
if (-not $SkipBuild) {
    Write-Host "${Yellow}🔨 Building project...${Reset}" -ForegroundColor Yellow
    cargo build --release
    if ($LASTEXITCODE -ne 0) {
        Write-Host "${Red}❌ Build failed${Reset}" -ForegroundColor Red
        exit 1
    }
    Write-Host "${Green}✅ Build completed${Reset}" -ForegroundColor Green
}

# Start the server
Write-Host "${Green}🌐 Starting OWami Network Testnet...${Reset}" -ForegroundColor Green
Write-Host "${Green}📡 Server will be available at: http://localhost:$Port${Reset}" -ForegroundColor Green
Write-Host "${Green}🖥️  Frontend available at: http://localhost:$Port/landing${Reset}" -ForegroundColor Green
Write-Host "${Green}🔧 API endpoints:${Reset}" -ForegroundColor Green
Write-Host "   - GET  http://localhost:$Port/api/token/info"
Write-Host "   - GET  http://localhost:$Port/api/token/balance/:address"
Write-Host "   - POST http://localhost:$Port/api/token/transfer"
Write-Host "   - POST http://localhost:$Port/api/token/mint"
Write-Host "   - POST http://localhost:$Port/api/token/approve"
Write-Host "   - GET  http://localhost:$Port/api/token/transactions"
Write-Host "   - POST http://localhost:$Port/api/dapp"
Write-Host "   - GET  http://localhost:$Port/api/dapp/user/:address"
Write-Host "   - GET  http://localhost:$Port/api/dapp/:id"
Write-Host "   - POST http://localhost:$Port/api/dapp/:id/state"
Write-Host "   - GET  http://localhost:$Port/api/dapp/:id/state/:key"
Write-Host ""
Write-Host "${Yellow}Press Ctrl+C to stop the server${Reset}" -ForegroundColor Yellow
Write-Host "=========================================="

# Run the server
cargo run --bin owami-network-simplified