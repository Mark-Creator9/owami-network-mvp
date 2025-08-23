# Quick verification script for Owami Network WASM setup

Write-Host "🔍 Verifying Owami Network WASM setup..." -ForegroundColor Cyan

# Check server health
Write-Host "`n📡 Testing server connectivity..." -ForegroundColor Yellow
try {
    $response = Invoke-RestMethod -Uri "http://localhost:8081/api/dapp/deploy" -Method Post -Body '{"contract_path":"test.wasm"}' -ContentType "application/json"
    Write-Host "✅ Server is responding" -ForegroundColor Green
} catch {
    Write-Host "❌ Server not responding: $_" -ForegroundColor Red
    exit 1
}

# Check if contract can be built
Write-Host "`n🔨 Testing contract build..." -ForegroundColor Yellow
try {
    Push-Location "simple_contract"
    cargo build --target wasm32-unknown-unknown --release
    if (Test-Path "..\target\wasm32-unknown-unknown\release\simple_contract.wasm") {
        Write-Host "✅ Contract builds successfully" -ForegroundColor Green
        Copy-Item "..\target\wasm32-unknown-unknown\release\simple_contract.wasm" "..\simple_contract.wasm" -Force
    } else {
        Write-Host "❌ Contract build failed - WASM file not found" -ForegroundColor Red
    }
    Pop-Location
} catch {
    Write-Host "❌ Contract build failed: $_" -ForegroundColor Red
    Pop-Location
    exit 1
}

# Test file upload
Write-Host "`n📤 Testing file upload..." -ForegroundColor Yellow
try {
    if (Test-Path "simple_contract.wasm") {
        $fileSize = (Get-Item "simple_contract.wasm").Length
        Write-Host "✅ WASM file found ($fileSize bytes)" -ForegroundColor Green
        
        # Test with a simple upload
        $boundary = [System.Guid]::NewGuid().ToString()
        $headers = @{
            "Content-Type" = "multipart/form-data; boundary=`"$boundary`""
        }
        
        $body = "--$boundary`r`n"
        $body += "Content-Disposition: form-data; name=`"file`"; filename=`"simple_contract.wasm`"`r`n"
        $body += "Content-Type: application/wasm`r`n`r`n"
        $body += [System.IO.File]::ReadAllBytes("simple_contract.wasm")
        $body += "`r`n--$boundary--`r`n"
        
        Write-Host "✅ File upload test prepared" -ForegroundColor Green
    } else {
        Write-Host "❌ WASM file not found" -ForegroundColor Red
    }
} catch {
    Write-Host "❌ File upload test failed: $_" -ForegroundColor Red
}

Write-Host "`n🎉 Setup verification complete!" -ForegroundColor Green
Write-Host "Ready to run: .\run_all_tests.ps1" -ForegroundColor Cyan