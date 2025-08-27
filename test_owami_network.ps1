# Owami Network Comprehensive Test Suite
# This script tests all the major functionality of the Owami Network

param(
    [string]$BaseUrl = "http://localhost:3000",
    [switch]$DetailedOutput = $false
)

# Color functions for output
function Write-Success {
    param([string]$Message)
    Write-Host "✓ $Message" -ForegroundColor Green
}

function Write-Error {
    param([string]$Message)
    Write-Host "✗ $Message" -ForegroundColor Red
}

function Write-Info {
    param([string]$Message)
    Write-Host "ℹ $Message" -ForegroundColor Cyan
}

function Write-Warning {
    param([string]$Message)
    Write-Host "⚠ $Message" -ForegroundColor Yellow
}

# Test API endpoint function
function Test-ApiEndpoint {
    param(
        [string]$Url,
        [string]$Method = "GET",
        [hashtable]$Headers = @{},
        [object]$Body = $null,
        [string]$Description
    )
    
    try {
        Write-Info "Testing: $Description"
        Write-Info "URL: $Method $Url"
        
        $params = @{
            Uri = $Url
            Method = $Method
            Headers = $Headers
            ErrorAction = "Stop"
        }
        
        if ($Body) {
            if ($Body -is [string]) {
                $params.Body = $Body
            } else {
                $params.Body = $Body | ConvertTo-Json -Depth 10
                $params.ContentType = "application/json"
            }
        }
        
        $response = Invoke-RestMethod @params
        
        if ($DetailedOutput) {
            Write-Host "Response:" -ForegroundColor Yellow
            $response | ConvertTo-Json -Depth 10 | Write-Host
        }
        
        Write-Success "$Description - SUCCESS"
        return @{
            Success = $true
            Response = $response
            Error = $null
        }
    }
    catch {
        Write-Error "$Description - FAILED"
        Write-Host "Error: $($_.Exception.Message)" -ForegroundColor Red
        
        if ($_.Exception.Response) {
            try {
                $errorDetails = $_.Exception.Response.GetResponseStream()
                $reader = New-Object System.IO.StreamReader($errorDetails)
                $responseBody = $reader.ReadToEnd()
                Write-Host "Response body: $responseBody" -ForegroundColor Red
            }
            catch {
                Write-Host "Could not read error response body" -ForegroundColor Red
            }
        }
        
        return @{
            Success = $false
            Response = $null
            Error = $_.Exception.Message
        }
    }
}

# Check if server is running
function Test-ServerAvailability {
    Write-Info "Checking if Owami Network server is running..."
    
    try {
        $response = Invoke-RestMethod -Uri "$BaseUrl/api/health" -Method GET -TimeoutSec 5
        Write-Success "Server is running and responsive"
        return $true
    }
    catch {
        Write-Error "Server is not responding at $BaseUrl"
        Write-Warning "Please start the server with: cargo run"
        return $false
    }
}

# Main testing function
function Start-OwamisNetworkTests {
    Write-Host "🚀 Starting Owami Network Comprehensive Test Suite" -ForegroundColor Magenta
    Write-Host "=" * 60 -ForegroundColor Magenta
    
    # Check server availability
    if (-not (Test-ServerAvailability)) {
        Write-Error "Cannot proceed without a running server"
        exit 1
    }
    
    $results = @()
    
    # Test 1: Health Check
    Write-Host "`n📋 HEALTH CHECK TESTS" -ForegroundColor Yellow
    $result = Test-ApiEndpoint -Url "$BaseUrl/api/health" -Description "Health Check Endpoint"
    $results += $result
    
    # Test 2: Blockchain Info
    Write-Host "`n⛓️ BLOCKCHAIN TESTS" -ForegroundColor Yellow
    $result = Test-ApiEndpoint -Url "$BaseUrl/api/blockchain/info" -Description "Get Blockchain Info"
    $results += $result
    
    # Test 3: Get Blocks
    $result = Test-ApiEndpoint -Url "$BaseUrl/api/blockchain/blocks" -Description "Get All Blocks"
    $results += $result
    
    # Test 4: Mine Block
    $result = Test-ApiEndpoint -Url "$BaseUrl/api/blockchain/mine" -Method "POST" -Description "Mine New Block"
    $results += $result
    
    # Test 5: Token Info
    Write-Host "`n🪙 TOKEN TESTS" -ForegroundColor Yellow
    $result = Test-ApiEndpoint -Url "$BaseUrl/api/token/info" -Description "Get Token Info"
    $results += $result
    
    # Test 6: Check balance for a test address
    $testAddress = "0x742d35Cc6634C0532925a3b8D4e6D3b6e8d3e8A0"
    $result = Test-ApiEndpoint -Url "$BaseUrl/api/token/balance/$testAddress" -Description "Get Token Balance"
    $results += $result
    
    # Test 7: Mint tokens (if endpoint supports it)
    $mintBody = @{
        address = $testAddress
        amount = 1000
    }
    $result = Test-ApiEndpoint -Url "$BaseUrl/api/token/mint" -Method "POST" -Body $mintBody -Description "Mint Tokens"
    $results += $result
    
    # Test 8: Get transactions
    $result = Test-ApiEndpoint -Url "$BaseUrl/api/token/transactions" -Description "Get Token Transactions"
    $results += $result
    
    # Test 9: DApp Management
    Write-Host "`n📱 DAPP TESTS" -ForegroundColor Yellow
    $result = Test-ApiEndpoint -Url "$BaseUrl/api/dapps" -Description "List DApps"
    $results += $result
    
    # Test 10: Create DApp
    $dappBody = @{
        name = "Test DApp"
        description = "A test decentralized application"
        contract_address = "0x" + (Get-Random -Maximum 999999999999).ToString("X12")
        creator_id = "test-user-" + (Get-Random -Maximum 1000)
    }
    $result = Test-ApiEndpoint -Url "$BaseUrl/api/dapps" -Method "POST" -Body $dappBody -Description "Create Test DApp"
    $results += $result
    
    # Test 11: Static file serving
    Write-Host "`n📄 STATIC FILE TESTS" -ForegroundColor Yellow
    $result = Test-ApiEndpoint -Url "$BaseUrl/index.html" -Description "Serve Landing Page"
    $results += $result
    
    # Generate Summary
    Write-Host "`n📊 TEST SUMMARY" -ForegroundColor Magenta
    Write-Host "=" * 60 -ForegroundColor Magenta
    
    $successCount = ($results | Where-Object { $_.Success }).Count
    $totalCount = $results.Count
    $failureCount = $totalCount - $successCount
    
    Write-Host "Total Tests: $totalCount" -ForegroundColor White
    Write-Success "Passed: $successCount"
    
    if ($failureCount -gt 0) {
        Write-Error "Failed: $failureCount"
        
        Write-Host "`nFailed Tests:" -ForegroundColor Red
        for ($i = 0; $i -lt $results.Count; $i++) {
            if (-not $results[$i].Success) {
                Write-Host "  - Test #$($i + 1): $($results[$i].Error)" -ForegroundColor Red
            }
        }
    } else {
        Write-Success "All tests passed! 🎉"
    }
    
    # Performance summary
    Write-Host "`n⚡ PERFORMANCE NOTES" -ForegroundColor Yellow
    Write-Info "• All API endpoints should respond within 1 second"
    Write-Info "• Blockchain operations (mining) may take longer"
    Write-Info "• Database operations should be optimized for production"
    
    # Security notes
    Write-Host "`n🔒 SECURITY REMINDERS" -ForegroundColor Yellow
    Write-Warning "• This is a testnet - not for production use"
    Write-Warning "• API endpoints are open for testing purposes"
    Write-Warning "• Implement proper authentication for production"
    
    return @{
        TotalTests = $totalCount
        PassedTests = $successCount
        FailedTests = $failureCount
        Results = $results
    }
}

# Performance testing function
function Start-PerformanceTests {
    param([int]$Iterations = 10)
    
    Write-Host "`n🏃‍♂️ PERFORMANCE TESTING" -ForegroundColor Magenta
    Write-Info "Running $Iterations iterations of key endpoints..."
    
    $endpoints = @(
        "$BaseUrl/api/health",
        "$BaseUrl/api/blockchain/info",
        "$BaseUrl/api/token/info"
    )
    
    foreach ($endpoint in $endpoints) {
        $times = @()
        Write-Info "Testing $endpoint..."
        
        for ($i = 1; $i -le $Iterations; $i++) {
            $stopwatch = [System.Diagnostics.Stopwatch]::StartNew()
            try {
                Invoke-RestMethod -Uri $endpoint -Method GET -TimeoutSec 10 | Out-Null
                $stopwatch.Stop()
                $times += $stopwatch.ElapsedMilliseconds
            }
            catch {
                Write-Warning "Request $i failed"
            }
        }
        
        if ($times.Count -gt 0) {
            $avgTime = ($times | Measure-Object -Average).Average
            $minTime = ($times | Measure-Object -Minimum).Minimum
            $maxTime = ($times | Measure-Object -Maximum).Maximum
            
            Write-Host "  Average: $($avgTime.ToString('F2'))ms | Min: ${minTime}ms | Max: ${maxTime}ms" -ForegroundColor Green
        }
    }
}

# Run the tests
if ($args.Count -eq 0 -or $args[0] -ne "performance-only") {
    $testResults = Start-OwamisNetworkTests
    
    if ($args -contains "performance" -or $args -contains "perf") {
        Start-PerformanceTests
    }
} else {
    Start-PerformanceTests -Iterations 20
}

Write-Host "`n🏁 Testing Complete!" -ForegroundColor Magenta