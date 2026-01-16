#!/usr/bin/env pwsh

Write-Host "🚀 Starting Owami Network Blockchain Test" -ForegroundColor Green

# Start the blockchain server in background
Write-Host "📡 Starting blockchain server..." -ForegroundColor Yellow
$serverProcess = Start-Process -FilePath ".\target\debug\owami-network.exe" -WorkingDirectory ".\" -PassThru -WindowStyle Hidden

# Wait for server to start
Start-Sleep -Seconds 3

# Test endpoints
$baseUrl = "http://localhost:8082"

Write-Host "🔍 Testing blockchain endpoints..." -ForegroundColor Yellow

# Test health check
Write-Host "1. Testing health check..." -ForegroundColor Cyan
try {
    $response = Invoke-RestMethod -Uri "$baseUrl/health" -Method GET
    Write-Host "✅ Health check passed:" -ForegroundColor Green
    Write-Host "   Status: $($response.status)" -ForegroundColor White
    Write-Host "   Network: $($response.network)" -ForegroundColor White
    Write-Host "   Database: $($response.database)" -ForegroundColor White
    Write-Host "   WASM Support: $($response.wasm_support)" -ForegroundColor White
} catch {
    Write-Host "❌ Health check failed: $($_.Exception.Message)" -ForegroundColor Red
}

# Test blockchain info
Write-Host "`n2. Testing blockchain info..." -ForegroundColor Cyan
try {
    $response = Invoke-RestMethod -Uri "$baseUrl/api/blockchain/info" -Method GET
    Write-Host "✅ Blockchain info retrieved:" -ForegroundColor Green
    Write-Host "   Network: $($response.network)" -ForegroundColor White
    Write-Host "   Block Height: $($response.block_height)" -ForegroundColor White
    Write-Host "   Total Supply: $($response.total_supply)" -ForegroundColor White
    Write-Host "   Version: $($response.version)" -ForegroundColor White
    Write-Host "   Smart Contracts: $($response.smart_contracts)" -ForegroundColor White
} catch {
    Write-Host "❌ Blockchain info failed: $($_.Exception.Message)" -ForegroundColor Red
}

# Test WASM info
Write-Host "`n3. Testing WASM info..." -ForegroundColor Cyan
try {
    $response = Invoke-RestMethod -Uri "$baseUrl/api/wasm/info" -Method GET
    Write-Host "✅ WASM info retrieved:" -ForegroundColor Green
    Write-Host "   Status: $($response.status)" -ForegroundColor White
    Write-Host "   Runtime: $($response.runtime)" -ForegroundColor White
    Write-Host "   Version: $($response.version)" -ForegroundColor White
} catch {
    Write-Host "❌ WASM info failed: $($_.Exception.Message)" -ForegroundColor Red
}

# Test contract deployment
Write-Host "`n4. Testing contract deployment..." -ForegroundColor Cyan
try {
    $contractData = @{
        contract_code = "contract SimpleStorage { uint256 value; }"
        creator = "test_user"
    }
    $response = Invoke-RestMethod -Uri "$baseUrl/api/contracts/deploy" -Method POST -Body ($contractData | ConvertTo-Json) -ContentType "application/json"
    Write-Host "✅ Contract deployed:" -ForegroundColor Green
    Write-Host "   Address: $($response.contract_address)" -ForegroundColor White
    Write-Host "   Message: $($response.message)" -ForegroundColor White
} catch {
    Write-Host "❌ Contract deployment failed: $($_.Exception.Message)" -ForegroundColor Red
}

# Test contract call
Write-Host "`n5. Testing contract call..." -ForegroundColor Cyan
try {
    $callData = @{
        contract_address = "0x12345678"
        function_name = "getValue"
    }
    $response = Invoke-RestMethod -Uri "$baseUrl/api/contracts/call" -Method POST -Body ($callData | ConvertTo-Json) -ContentType "application/json"
    Write-Host "✅ Contract call executed:" -ForegroundColor Green
    Write-Host "   Success: $($response.success)" -ForegroundColor White
    Write-Host "   Result: $($response.result)" -ForegroundColor White
    Write-Host "   Gas Used: $($response.gas_used)" -ForegroundColor White
} catch {
    Write-Host "❌ Contract call failed: $($_.Exception.Message)" -ForegroundColor Red
}

# Test list contracts
Write-Host "`n6. Testing contract list..." -ForegroundColor Cyan
try {
    $response = Invoke-RestMethod -Uri "$baseUrl/api/contracts" -Method GET
    Write-Host "✅ Contract list retrieved:" -ForegroundColor Green
    Write-Host "   Count: $($response.count)" -ForegroundColor White
    Write-Host "   Contracts: $($response.contracts -join ', ')" -ForegroundColor White
} catch {
    Write-Host "❌ Contract list failed: $($_.Exception.Message)" -ForegroundColor Red
}

# Test mining
Write-Host "`n7. Testing block mining..." -ForegroundColor Cyan
try {
    $response = Invoke-RestMethod -Uri "$baseUrl/api/blockchain/mine" -Method POST
    Write-Host "✅ Block mined:" -ForegroundColor Green
    Write-Host "   Result: $response" -ForegroundColor White
} catch {
    Write-Host "❌ Block mining failed: $($_.Exception.Message)" -ForegroundColor Red
}

# Cleanup
Write-Host "`n🧹 Cleaning up..." -ForegroundColor Yellow
if ($serverProcess -and !$serverProcess.HasExited) {
    Stop-Process -Id $serverProcess.Id -Force
    Write-Host "✅ Server stopped" -ForegroundColor Green
}

Write-Host "`n🎉 Blockchain test completed!" -ForegroundColor Green