# Quickstart

This guide gets you from zero to a running Owami Network instance quickly.

## Prerequisites
- Rust 1.70+
- PostgreSQL 14+
- PowerShell 7+ (on Windows)

## 1) Clone and Configure
```powershell
# clone your fork or the main repo
# git clone https://github.com/your-org/owami-network.git
# cd owami-network

# Option A: Aiven Cloud (Recommended)
# Use the launch script to set DATABASE_URL and run with production config
$env:AIVEN_PASSWORD = "YOUR_AIVEN_PASSWORD"
./launch_aiven_demo.ps1 -AivenPassword $env:AIVEN_PASSWORD -OpenBrowser

# Option B: Local/Custom DB
Copy-Item .env.example .env -Force
# edit .env and set DATABASE_URL, JWT_SECRET, PORT (optional)
```

## 2) Database Setup
```powershell
# For Aiven: migrations will be run automatically if sqlx-cli is installed
# Otherwise, you can run manually:
sqlx migrate run
```

## 3) Build and Run
```powershell
# Build
cargo build --release

# Start (Windows)
# Preferred (Aiven): uses DATABASE_URL from the environment
./start_server.bat
# or run directly
$env:JWT_SECRET = "change-me"; cargo run --release
```

## 4) Verify
- Open: http://localhost:3002/landing (or the port you configured)
- Health: GET http://localhost:3002/api/health

## 5) Try the API
- Import `postman/Owami.postman_collection.json` into Postman
- Use the provided environment `postman/Owami.local_environment.json`

## 6) Logs
- Logs directory: `logs/` (server.out.log, server.err.log, audit.log)

## 7) Troubleshooting
See `docs/TROUBLESHOOTING.md`.