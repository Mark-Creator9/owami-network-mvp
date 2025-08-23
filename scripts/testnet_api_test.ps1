# Owami Network Testnet API Testing Script
# Author: Mark Creator9
# This script tests all API endpoints

param(
    [string]$BaseUrl = "http://localhost:3000"
)

Write-Host "Testing Owami Network Testnet API..." -ForegroundColor Green

# Function to test API endpoints
function Test-ApiEndpoint {
    param(
        [string]$Method,
        [string]$Endpoint,
        [object]$Body = $null
    )
    
    $url = "$BaseUrl$Endpoint"
    Write-Host "`nTesting $Method $url" -ForegroundColor Yellow
    
    try {
        $headers = @{
            "Content-Type" = "application/json"
            "Accept" = "application/json"
        }
        
        $params = @{
            Method = $Method
            Uri = $url
            Headers = $headers
            UseBasicParsing = $true
        }
        
        if ($Body -ne $null) {
            $params.Body = $Body | ConvertTo-Json -Depth 10
        }
        
        $response = Invoke-RestMethod @params
        
        Write-Host "✅ Success: $($response | ConvertTo-Json -Depth 3)" -ForegroundColor Green
        return $response
    } catch {
        Write-Host "❌ Error: $($_.Exception.Message)" -ForegroundColor Red
        if ($_.Exception.Response) {
            $reader = New-Object System.IO.StreamReader($_.Exception.Response.GetResponseStream())
            $reader.BaseStream.Position = 0
            $reader.DiscardBufferedData()
            $responseBody = $reader.ReadToEnd()
            Write-Host "Response: $responseBody" -ForegroundColor Red
        }
        return $null
    }
}

# Test blockchain endpoints
Write-Host "`n=== Testing Blockchain Endpoints ===" -ForegroundColor Cyan
Test-ApiEndpoint -Method "GET" -Endpoint "/api/blockchain/info"
Test-ApiEndpoint -Method "GET" -Endpoint "/api/blockchain/blocks"

# Test token endpoints
Write-Host "`n=== Testing Token Endpoints ===" -ForegroundColor Cyan
$testAddress = "test_address_123"
Test-ApiEndpoint -Method "GET" -Endpoint "/api/token/balance/$testAddress"

# Mint some tokens
Test-ApiEndpoint -Method "POST" -Endpoint "/api/token/mint/$testAddress" -Body 1000

# Check balance again
Test-ApiEndpoint -Method "GET" -Endpoint "/api/token/balance/$testAddress"

# Test transfer
$transferBody = @{
    from = $testAddress
    to = "recipient_address_456"
    amount = 100
    private_key = "test_private_key"
}
Test-ApiEndpoint -Method "POST" -Endpoint "/api/token/transfer" -Body $transferBody

# Test DApp endpoints
Write-Host "`n=== Testing DApp Endpoints ===" -ForegroundColor Cyan
Test-ApiEndpoint -Method "GET" -Endpoint "/api/dapps"

# Create a DApp
$dappBody = @{
    name = "Test DApp"
    description = "A test decentralized application"
    contract_address = "0x1234567890abcdef"
}
Test-ApiEndpoint -Method "POST" -Endpoint "/api/dapps" -Body $dappBody

# List DApps again
$dapps = Test-ApiEndpoint -Method "GET" -Endpoint "/api/dapps"
if ($dapps -and $dapps.data -and $dapps.data.Count -gt 0) {
    $dappId = $dapps.data[0].id
    Test-ApiEndpoint -Method "GET" -Endpoint "/api/dapps/$dappId"
}

# Test mining
Write-Host "`n=== Testing Mining ===" -ForegroundColor Cyan
Test-ApiEndpoint -Method "POST" -Endpoint "/api/blockchain/mine"

Write-Host "`n=== API Testing Complete ===" -ForegroundColor Green