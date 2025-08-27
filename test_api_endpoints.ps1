# Owami Network API Testing Script
# This script tests all API endpoints comprehensively

param(
    [string]$BaseUrl = "http://localhost:3000",
    [switch]$Verbose
)

Write-Host "🚀 Starting Owami Network API Testing..." -ForegroundColor Green
Write-Host "Base URL: $BaseUrl" -ForegroundColor Cyan

# Test Results Storage
$testResults = @()
$passedTests = 0
$failedTests = 0

# Helper function to test API endpoint
function Test-Endpoint {
    param(
        [string]$Method,
        [string]$Endpoint,
        [string]$Description,
        [hashtable]$Body = $null,
        [int]$ExpectedStatus = 200
    )
    
    Write-Host "`n🧪 Testing: $Description" -ForegroundColor Yellow
    Write-Host "   $Method $Endpoint" -ForegroundColor Gray
    
    try {
        $uri = "$BaseUrl$Endpoint"
        $params = @{
            Uri = $uri
            Method = $Method
            ContentType = "application/json"
        }
        
        if ($Body) {
            $params.Body = $Body | ConvertTo-Json -Depth 10
            if ($Verbose) {
                Write-Host "   Request Body: $($params.Body)" -ForegroundColor DarkGray
            }
        }
        
        $response = Invoke-RestMethod @params
        
        if ($Verbose) {
            Write-Host "   Response: $($response | ConvertTo-Json -Depth 3)" -ForegroundColor DarkGray
        }
        
        Write-Host "   ✅ PASSED" -ForegroundColor Green
        $script:passedTests++
        $script:testResults += @{
            Test = $Description
            Status = "PASSED"
            Endpoint = $Endpoint
            Method = $Method
            Response = $response
        }
        
        return $response
    }
    catch {
        Write-Host "   ❌ FAILED: $($_.Exception.Message)" -ForegroundColor Red
        $script:failedTests++
        $script:testResults += @{
            Test = $Description
            Status = "FAILED"
            Endpoint = $Endpoint
            Method = $Method
            Error = $_.Exception.Message
        }
        
        return $null
    }
}

# Wait for server to be ready
Write-Host "`n⏳ Checking server availability..." -ForegroundColor Cyan
$maxRetries = 10
$retryCount = 0

do {
    try {
        $response = Invoke-WebRequest -Uri "$BaseUrl/api/token/info" -TimeoutSec 5
        Write-Host "✅ Server is ready!" -ForegroundColor Green
        break
    }
    catch {
        $retryCount++
        if ($retryCount -ge $maxRetries) {
            Write-Host "❌ Server not responding after $maxRetries attempts" -ForegroundColor Red
            exit 1
        }
        Write-Host "⏳ Attempt $retryCount/$maxRetries - Server not ready, waiting..." -ForegroundColor Yellow
        Start-Sleep -Seconds 2
    }
} while ($retryCount -lt $maxRetries)

Write-Host "`n🔥 Starting API Endpoint Tests..." -ForegroundColor Magenta

# ============================================================================
# TOKEN API TESTS
# ============================================================================
Write-Host "`n📊 TOKEN API TESTS" -ForegroundColor Blue

# Test 1: Get Token Info
Test-Endpoint -Method "GET" -Endpoint "/api/token/info" -Description "Get Token Information"

# Test 2: Get Balance for an address
$testAddress = "0x1234567890123456789012345678901234567890"
Test-Endpoint -Method "GET" -Endpoint "/api/token/balance/$testAddress" -Description "Get Token Balance"

# Test 3: Test token transactions endpoint
Test-Endpoint -Method "GET" -Endpoint "/api/token/transactions" -Description "Get Token Transactions"

# Test 4: Test token mint (if enabled)
$mintBody = @{
    to = $testAddress
    amount = "1000"
}
Test-Endpoint -Method "POST" -Endpoint "/api/token/mint" -Description "Mint Tokens" -Body $mintBody

# Test 5: Test token transfer
$transferBody = @{
    from = $testAddress
    to = "0x9876543210987654321098765432109876543210"
    amount = "100"
}
Test-Endpoint -Method "POST" -Endpoint "/api/token/transfer" -Description "Transfer Tokens" -Body $transferBody

# Test 6: Test token approve
$approveBody = @{
    owner = $testAddress
    spender = "0x9876543210987654321098765432109876543210"
    amount = "500"
}
Test-Endpoint -Method "POST" -Endpoint "/api/token/approve" -Description "Approve Token Spending" -Body $approveBody

# ============================================================================
# DAPP API TESTS
# ============================================================================
Write-Host "`n🏗️ DAPP API TESTS" -ForegroundColor Blue

# Test 7: Create DApp
$dappBody = @{
    name = "Test DApp"
    description = "A test DApp for API testing"
    contract_address = "0xabcdef1234567890abcdef1234567890abcdef12"
    creator_id = "550e8400-e29b-41d4-a716-446655440000"
}
$createdDapp = Test-Endpoint -Method "POST" -Endpoint "/api/dapp" -Description "Create DApp" -Body $dappBody

# Test 8: Get user's DApps
$userId = "550e8400-e29b-41d4-a716-446655440000"
Test-Endpoint -Method "GET" -Endpoint "/api/dapp/user/$userId" -Description "Get User's DApps"

# Test 9: Get specific DApp (if creation was successful)
if ($createdDapp -and $createdDapp.id) {
    Test-Endpoint -Method "GET" -Endpoint "/api/dapp/$($createdDapp.id)" -Description "Get Specific DApp"
    
    # Test 10: Update DApp state
    $stateBody = @{
        key = "test_key"
        value = "test_value"
    }
    Test-Endpoint -Method "POST" -Endpoint "/api/dapp/$($createdDapp.id)/state" -Description "Update DApp State" -Body $stateBody
    
    # Test 11: Get DApp state value
    Test-Endpoint -Method "GET" -Endpoint "/api/dapp/$($createdDapp.id)/state/test_key" -Description "Get DApp State Value"
}

# ============================================================================
# BLOCKCHAIN API TESTS
# ============================================================================
Write-Host "`n⛓️ BLOCKCHAIN API TESTS" -ForegroundColor Blue

# Test 12: Get blockchain info
Test-Endpoint -Method "GET" -Endpoint "/api/blockchain/info" -Description "Get Blockchain Info"

# Test 13: Get blocks
Test-Endpoint -Method "GET" -Endpoint "/api/blockchain/blocks" -Description "Get Blockchain Blocks"

# Test 14: Mine a block (if endpoint exists)
Test-Endpoint -Method "POST" -Endpoint "/api/blockchain/mine" -Description "Mine New Block"

# ============================================================================
# HEALTH CHECK TESTS
# ============================================================================
Write-Host "`n🏥 HEALTH CHECK TESTS" -ForegroundColor Blue

# Test 15: Basic health check (try common health endpoints)
Test-Endpoint -Method "GET" -Endpoint "/health" -Description "Health Check Endpoint"
Test-Endpoint -Method "GET" -Endpoint "/api/health" -Description "API Health Check"
Test-Endpoint -Method "GET" -Endpoint "/status" -Description "Status Endpoint"

# ============================================================================
# FRONTEND TESTS
# ============================================================================
Write-Host "`n🌐 FRONTEND TESTS" -ForegroundColor Blue

# Test 16: Landing page accessibility
Test-Endpoint -Method "GET" -Endpoint "/landing" -Description "Landing Page Access"

# Test 17: Static file serving (CSS)
Test-Endpoint -Method "GET" -Endpoint "/landing/css/style.css" -Description "Static CSS File"

# Test 18: Static file serving (JavaScript)
Test-Endpoint -Method "GET" -Endpoint "/landing/js/app.js" -Description "Static JavaScript File"

# ============================================================================
# ERROR HANDLING TESTS
# ============================================================================
Write-Host "`n⚠️ ERROR HANDLING TESTS" -ForegroundColor Blue

# Test 19: Invalid endpoint
try {
    Invoke-RestMethod -Uri "$BaseUrl/api/invalid/endpoint" -Method GET
    Write-Host "   ❌ FAILED: Should have returned 404" -ForegroundColor Red
    $failedTests++
}
catch {
    if ($_.Exception.Response.StatusCode -eq 404) {
        Write-Host "   ✅ PASSED: Correctly returned 404 for invalid endpoint" -ForegroundColor Green
        $passedTests++
    } else {
        Write-Host "   ❌ FAILED: Unexpected error: $($_.Exception.Message)" -ForegroundColor Red
        $failedTests++
    }
}

# Test 20: Invalid JSON body
try {
    $invalidBody = "invalid json"
    Invoke-RestMethod -Uri "$BaseUrl/api/token/transfer" -Method POST -Body $invalidBody -ContentType "application/json"
    Write-Host "   ❌ FAILED: Should have rejected invalid JSON" -ForegroundColor Red
    $failedTests++
}
catch {
    Write-Host "   ✅ PASSED: Correctly rejected invalid JSON" -ForegroundColor Green
    $passedTests++
}

# ============================================================================
# TEST SUMMARY
# ============================================================================
Write-Host "`n📊 TEST SUMMARY" -ForegroundColor Magenta
Write-Host "===========================================" -ForegroundColor Magenta

$totalTests = $passedTests + $failedTests
$successRate = if ($totalTests -gt 0) { [math]::Round(($passedTests / $totalTests) * 100, 2) } else { 0 }

Write-Host "Total Tests: $totalTests" -ForegroundColor Cyan
Write-Host "Passed: $passedTests" -ForegroundColor Green
Write-Host "Failed: $failedTests" -ForegroundColor Red
Write-Host "Success Rate: $successRate%" -ForegroundColor $(if ($successRate -ge 90) { "Green" } elseif ($successRate -ge 70) { "Yellow" } else { "Red" })

if ($failedTests -eq 0) {
    Write-Host "`n🎉 ALL TESTS PASSED! Owami Network API is fully operational!" -ForegroundColor Green
} else {
    Write-Host "`n⚠️ Some tests failed. Please check the detailed results below." -ForegroundColor Yellow
}

# Detailed Results
if ($Verbose -or $failedTests -gt 0) {
    Write-Host "`n📋 DETAILED RESULTS:" -ForegroundColor Cyan
    foreach ($result in $testResults) {
        $color = if ($result.Status -eq "PASSED") { "Green" } else { "Red" }
        Write-Host "[$($result.Status)] $($result.Test)" -ForegroundColor $color
        if ($result.Error) {
            Write-Host "   Error: $($result.Error)" -ForegroundColor DarkRed
        }
    }
}

# Export results to JSON
$resultsFile = "api_test_results.json"
$testResults | ConvertTo-Json -Depth 10 | Out-File -FilePath $resultsFile -Encoding UTF8
Write-Host "`n💾 Test results saved to: $resultsFile" -ForegroundColor Cyan

Write-Host "`n🏁 API Testing Complete!" -ForegroundColor Green