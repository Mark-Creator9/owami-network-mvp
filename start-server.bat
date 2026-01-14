@echo off
REM Owami Network - Server Startup Script

echo.
echo ==========================================
echo   Owami Network
echo ==========================================
echo.
echo  Building and starting server...
echo.

REM Build the project
cargo build --bin owami-server

if %ERRORLEVEL% NEQ 0 (
    echo.
    echo ERROR: Build failed
    pause
    exit /b 1
)

echo.
echo  Starting Owami Network server...
echo  Frontend: http://localhost:8081
echo  API:      http://localhost:8081/api
echo.
echo  Press Ctrl+C to stop the server
echo.
echo ==========================================
echo.

REM Run the server
cargo run --bin owami-server
