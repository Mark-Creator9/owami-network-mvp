@echo off
setlocal

echo Starting Owami Network Server...

REM Ensure required environment variables are set
if "%DATABASE_URL%"=="" (
  echo ERROR: DATABASE_URL is not set.
  echo Please set DATABASE_URL or use launch_aiven_demo.ps1 to configure Aiven.
  echo Example: set DATABASE_URL=postgres://avnadmin:***@host:port/defaultdb?sslmode=require
  exit /b 1
)

REM Ensure logs directory exists
if not exist logs (
  mkdir logs
)

set CONFIG_PATH=config/production.toml
if "%RUST_LOG%"=="" set RUST_LOG=info
if "%JWT_SECRET%"=="" set JWT_SECRET=owami_dev_jwt_secret_please_change

REM Redirect stdout and stderr to log files
cargo run --release --bin owami-network 1> logs\server.out.log 2> logs\server.err.log

endlocal
