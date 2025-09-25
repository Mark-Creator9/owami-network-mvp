<#
Bootstrap local environment for Owami Network
- Ensures .env exists
- Applies migrations
- Starts the server via start_server.bat
#>

param(
    [switch]$SkipMigrations
)

Write-Host "[Owami] Bootstrap starting..." -ForegroundColor Cyan

# Ensure .env exists
if (-not (Test-Path -Path ".env")) {
    if (Test-Path -Path ".env.example") {
        Copy-Item ".env.example" ".env" -Force
        Write-Host "[Owami] Created .env from .env.example. Please edit DATABASE_URL and JWT_SECRET." -ForegroundColor Yellow
    } else {
        Write-Host "[Owami] Missing .env and .env.example. Please create .env before proceeding." -ForegroundColor Red
        exit 1
    }
}

# Apply migrations (unless skipped)
if (-not $SkipMigrations) {
    try {
        Write-Host "[Owami] Applying database migrations..." -ForegroundColor Cyan
        sqlx migrate run
    } catch {
        Write-Host "[Owami] Failed to run migrations. Ensure sqlx is installed and DATABASE_URL is valid." -ForegroundColor Red
        exit 1
    }
}

# Start server
if (Test-Path -Path "start_server.bat") {
    Write-Host "[Owami] Starting server (production config)..." -ForegroundColor Cyan
    ./start_server.bat
} else {
    Write-Host "[Owami] start_server.bat not found. Starting with cargo run --release" -ForegroundColor Yellow
    cargo run --release
}