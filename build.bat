@echo off
setlocal enabledelayedexpansion

:: Owami Network Build Script for Windows
:: Optimized for faster compilation

echo 🚀 Starting Owami Network Build Process
echo ========================================

:: Set environment variables for optimized build
set RUSTFLAGS=-C target-cpu=native
set CARGO_INCREMENTAL=1

:: Check command line arguments
if "%1"=="dev" (
    echo 🔨 Building in development mode...
    echo.
    cargo build --incremental
    if !errorlevel! equ 0 (
        echo ✅ Development build completed successfully!
        echo ========================================
        echo 📁 Binary location: target\debug\owami-server.exe
        echo 🚀 To run: cargo run
    ) else (
        echo ❌ Development build failed!
        goto :error
    )
) else if "%1"=="release" (
    echo 🔨 Building in release mode (optimized)...
    echo.
    cargo build --release --config 'profile.release.lto = "thin"' --config 'profile.release.codegen-units = 16'
    if !errorlevel! equ 0 (
        echo ✅ Release build completed successfully!
        echo ========================================
        echo 📁 Binary location: target\release\owami-server.exe
        echo 🚀 To run: cargo run --release
    ) else (
        echo ❌ Release build failed!
        goto :error
    )
) else if "%1"=="clean" (
    echo 🧹 Cleaning build artifacts...
    echo.
    cargo clean
    if exist target rd /s /q target
    echo ✅ Clean completed!
    goto :end
) else (
    echo Usage: build.bat [dev|release|clean]
    echo.
    echo Examples:
    echo   build.bat dev      - Fast development build
    echo   build.bat release  - Optimized production build
    echo   build.bat clean    - Clean all build artifacts
    goto :end
)

goto :end

:error
echo.
echo 🔍 Troubleshooting tips:
if "%1"=="dev" (
    echo 1. Try: cargo clean
    echo 2. Try: cargo update
    echo 3. Check for missing dependencies
) else (
    echo 1. Try: build.bat clean
    echo 2. Try: cargo build (development mode first)
    echo 3. Check system resources (RAM, disk space)
)

:end
endlocal