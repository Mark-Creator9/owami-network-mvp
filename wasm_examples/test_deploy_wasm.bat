@echo off
REM Windows batch file to test WASM contract deployment

echo Testing Owami Network WASM deployment...
echo.

REM Check if PowerShell is available
where powershell >nul 2>nul
if %errorlevel% neq 0 (
    echo PowerShell is required but not found.
    echo Please install PowerShell or use the manual steps.
    pause
    exit /b 1
)

REM Run the PowerShell script
powershell -ExecutionPolicy Bypass -File "test_deploy_wasm.ps1"

echo.
echo Test completed.
pause