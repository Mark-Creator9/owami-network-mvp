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
        [int]$ExpectedStatus = 200,
        [hashtable]$Headers = $null
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
        
        if ($Headers) {
            $params.Headers = $Headers
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
# AUTHENTICATION SETUP
# ============================================================================
Write-Host "`n🔐 AUTHENTICATION SETUP" -ForegroundColor Blue

# Register a test user with unique username to avoid conflicts
$timestamp = Get-Date -Format "yyyyMMddHHmmss"
$testUsername = "testuser_$timestamp"
$registerBody = @{
    username = $testUsername
    password = "testpassword123"
}
$authResponse = Test-Endpoint -Method "POST" -Endpoint "/api/auth/register" -Description "Register Test User" -Body $registerBody
$authToken = $null
if ($authResponse -and $authResponse.token) {
    $authToken = $authResponse.token
    Write-Host "   ✅ Authentication token obtained" -ForegroundColor Green
} else {
    # Try login if registration failed (user might already exist)
    $loginBody = @{
        username = "testuser"
        password = "testpassword123"
    }
    $loginResponse = Test-Endpoint -Method "POST" -Endpoint "/api/auth/login" -Description "Login Test User" -Body $loginBody
    if ($loginResponse -and $loginResponse.token) {
        $authToken = $loginResponse.token
        Write-Host "   ✅ Authentication token obtained via login" -ForegroundColor Green
    } else {
        Write-Host "   ⚠️ Authentication failed, continuing without auth" -ForegroundColor Yellow
    }
}

$authHeaders = @{}
if ($authToken) {
    $authHeaders = @{
        Authorization = "Bearer $authToken"
    }
}

# ============================================================================
# TOKEN API TESTS
# ============================================================================
Write-Host "`n📊 TOKEN API TESTS" -ForegroundColor Blue

# Test 1: Get Token Info
Test-Endpoint -Method "GET" -Endpoint "/api/token/info" -Description "Get Token Information"

# Test 2: Get Balance for an address
$testAddress = "20fa6a2566e8022b545347740091e02eed91165d8f9e0413d6a4bb647d8b1e4a"  # Valid ED25519 public key
Test-Endpoint -Method "GET" -Endpoint "/api/token/balance/$testAddress" -Description "Get Token Balance"

# Test 3: Test token transactions endpoint
Test-Endpoint -Method "GET" -Endpoint "/api/token/transactions" -Description "Get Token Transactions"

# Test 4: Test token mint (if enabled)
$mintBody = @{
    to = $testAddress
    amount = 1000  # Changed from string to number
}
Test-Endpoint -Method "POST" -Endpoint "/api/token/mint" -Description "Mint Tokens" -Body $mintBody

# Test 5: Test token transfer
$transferBody = @{
    from = $testAddress
    to = "20fa6a2566e8022b545347740091e02eed91165d8f9e0413d6a4bb647d8b1e4a"  # Same address for testing
    amount = 100   # Changed from string to number
    private_key = "154dc850c10d24b29d9d245e8052610d4f746973c658751bf12ded323fbe90e1"  # New valid ED25519 private key
}
Test-Endpoint -Method "POST" -Endpoint "/api/token/transfer" -Description "Transfer Tokens" -Body $transferBody

# Test 6: Test token approve (if endpoint exists) - Currently not implemented
# $approveBody = @{
#     owner = $testAddress
#     spender = "0x9876543210987654321098765432109876543210"
#     amount = 500   # Changed from string to number
# }
# Test-Endpoint -Method "POST" -Endpoint "/api/token/approve" -Description "Approve Token Spending" -Body $approveBody

# ============================================================================
# DAPP API TESTS
# ============================================================================
Write-Host "`n🏗️ DAPP API TESTS" -ForegroundColor Blue

# Test 7: Create DApp (with authentication)
$dappBody = @{
    name = "Test DApp"
    description = "A test DApp for API testing"
    contract_address = "0xabcdef1234567890abcdef1234567890abcdef12"
}
$createdDapp = Test-Endpoint -Method "POST" -Endpoint "/api/dapps" -Description "Create DApp" -Body $dappBody -Headers $authHeaders

# Test 8: Get user's DApps (if authentication available)
if ($authToken) {
    Test-Endpoint -Method "GET" -Endpoint "/api/dapps" -Description "Get User's DApps" -Headers $authHeaders
} else {
    Write-Host "`n⏭️ Skipping authenticated DApp tests (no auth token)" -ForegroundColor Yellow
}

# Test 9: Get specific DApp (if creation was successful)
if ($createdDapp -and $createdDapp.id) {
    Test-Endpoint -Method "GET" -Endpoint "/api/dapps/$($createdDapp.id)" -Description "Get Specific DApp" -Headers $authHeaders
    
    # Test 10: Update DApp state (if endpoint exists)
    $stateBody = @{
        key = "test_key"
        value = "test_value"
    }
    Test-Endpoint -Method "POST" -Endpoint "/api/dapps/$($createdDapp.id)/state" -Description "Update DApp State" -Body $stateBody -Headers $authHeaders
    
    # Test 11: Get DApp state value (if endpoint exists)
    Test-Endpoint -Method "GET" -Endpoint "/api/dapps/$($createdDapp.id)/state/test_key" -Description "Get DApp State Value" -Headers $authHeaders
}

# ============================================================================
# BLOCKCHAIN API TESTS
# ============================================================================
Write-Host "`n⛓️ BLOCKCHAIN API TESTS" -ForegroundColor Blue

# Test 12: Get blockchain info
Test-Endpoint -Method "GET" -Endpoint "/api/blockchain/info" -Description "Get Blockchain Info"

# Test 13: Get blocks
Test-Endpoint -Method "GET" -Endpoint "/api/blockchain/blocks" -Description "Get Blockchain Blocks"

# Test 14: Mine a block (if endpoint exists) - Only attempt if we have transactions
if ($passedTests -ge 5) {  # Only mine if we've created some transactions
    Test-Endpoint -Method "POST" -Endpoint "/api/blockchain/mine" -Description "Mine New Block"
} else {
    Write-Host "`n⏭️ Skipping mining test (no transactions created)" -ForegroundColor Yellow
}

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