param(
    [string]$BaseUrl = "http://localhost:3000"
)

# Colors for output
$Green = "`e[32m"
$Yellow = "`e[33m"
$Red = "`e[31m"
$Reset = "`e[0m"

Write-Host "${Green}🧪 Testing OWami Network API Endpoints${Reset}" -ForegroundColor Green
Write-Host "======================================"

# Test Health Check
Write-Host "`n${Yellow}🔍 Testing Health Check...${Reset}" -ForegroundColor Yellow

Write-Host "`n1. GET $BaseUrl/api/health"
try {
    $response = Invoke-RestMethod -Uri "$BaseUrl/api/health" -Method Get
    Write-Host "${Green}✅ Success:${Reset} $($response | ConvertTo-Json -Compress)"
} catch {
    Write-Host "${Red}❌ Failed:${Reset} $($_.Exception.Message)"
}

# Test Token API endpoints
Write-Host "`n${Yellow}🔍 Testing Token API...${Reset}" -ForegroundColor Yellow

# 1. Get token info
Write-Host "`n2. GET $BaseUrl/api/token/info"
try {
    $response = Invoke-RestMethod -Uri "$BaseUrl/api/token/info" -Method Get
    Write-Host "${Green}✅ Success:${Reset} $($response | ConvertTo-Json -Compress)"
} catch {
    Write-Host "${Red}❌ Failed:${Reset} $($_.Exception.Message)"
}

# 2. Get balance (test address)
Write-Host "`n3. GET $BaseUrl/api/token/balance/0x1234567890123456789012345678901234567890"
try {
    $response = Invoke-RestMethod -Uri "$BaseUrl/api/token/balance/0x1234567890123456789012345678901234567890" -Method Get
    Write-Host "${Green}✅ Success:${Reset} $($response | ConvertTo-Json -Compress)"
} catch {
    Write-Host "${Red}❌ Failed:${Reset} $($_.Exception.Message)"
}

# 3. Mint tokens (new format)
Write-Host "`n4. POST $BaseUrl/api/token/mint"
$mintBody = @{
    to = "0x1234567890123456789012345678901234567890"
    amount = 1000000000000000000
} | ConvertTo-Json
try {
    $response = Invoke-RestMethod -Uri "$BaseUrl/api/token/mint" -Method Post -Body $mintBody -ContentType "application/json"
    Write-Host "${Green}✅ Success:${Reset} $($response | ConvertTo-Json -Compress)"
} catch {
    Write-Host "${Red}❌ Failed:${Reset} $($_.Exception.Message)"
}

# 4. Transfer tokens
Write-Host "`n5. POST $BaseUrl/api/token/transfer"
$transferBody = @{
    from = "0x1234567890123456789012345678901234567890"
    to = "0x0987654321098765432109876543210987654321"
    amount = 500000000000000000
    private_key = "dummy_key"
} | ConvertTo-Json
try {
    $response = Invoke-RestMethod -Uri "$BaseUrl/api/token/transfer" -Method Post -Body $transferBody -ContentType "application/json"
    Write-Host "${Green}✅ Success:${Reset} $($response | ConvertTo-Json -Compress)"
} catch {
    Write-Host "${Red}❌ Failed:${Reset} $($_.Exception.Message)"
}

# 5. Get transactions
Write-Host "`n6. GET $BaseUrl/api/token/transactions"
try {
    $response = Invoke-RestMethod -Uri "$BaseUrl/api/token/transactions" -Method Get
    Write-Host "${Green}✅ Success:${Reset} Found $($response.Count) transactions"
    if ($response.Count -gt 0) {
        Write-Host "   Latest: $($response[0] | ConvertTo-Json -Compress)"
    }
} catch {
    Write-Host "${Red}❌ Failed:${Reset} $($_.Exception.Message)"
}

# Test Blockchain API endpoints
Write-Host "`n${Yellow}🔍 Testing Blockchain API...${Reset}" -ForegroundColor Yellow

# 6. Get blockchain info
Write-Host "`n7. GET $BaseUrl/api/blockchain/info"
try {
    $response = Invoke-RestMethod -Uri "$BaseUrl/api/blockchain/info" -Method Get
    Write-Host "${Green}✅ Success:${Reset} $($response | ConvertTo-Json -Compress)"
} catch {
    Write-Host "${Red}❌ Failed:${Reset} $($_.Exception.Message)"
}

# 7. Get blocks
Write-Host "`n8. GET $BaseUrl/api/blockchain/blocks"
try {
    $response = Invoke-RestMethod -Uri "$BaseUrl/api/blockchain/blocks" -Method Get
    Write-Host "${Green}✅ Success:${Reset} Found $($response.Count) blocks"
    if ($response.Count -gt 0) {
        Write-Host "   Latest block: $($response[-1] | ConvertTo-Json -Compress)"
    }
} catch {
    Write-Host "${Red}❌ Failed:${Reset} $($_.Exception.Message)"
}

# Test DApp API endpoints
Write-Host "`n${Yellow}🔍 Testing DApp API...${Reset}" -ForegroundColor Yellow

# 8. Create DApp
Write-Host "`n9. POST $BaseUrl/api/dapps"
$dappBody = @{
    name = "Test DApp"
    description = "A test decentralized application"
    contract_address = "0x1111111111111111111111111111111111111111"
} | ConvertTo-Json
try {
    $response = Invoke-RestMethod -Uri "$BaseUrl/api/dapps" -Method Post -Body $dappBody -ContentType "application/json"
    Write-Host "${Green}✅ Success:${Reset} $($response | ConvertTo-Json -Compress)"
    $dappId = $response.data.id
} catch {
    Write-Host "${Red}❌ Failed:${Reset} $($_.Exception.Message)"
    $dappId = "test-id"
}

# 9. List DApps
Write-Host "`n10. GET $BaseUrl/api/dapps"
try {
    $response = Invoke-RestMethod -Uri "$BaseUrl/api/dapps" -Method Get
    Write-Host "${Green}✅ Success:${Reset} Found $($response.data.Count) DApps"
    if ($response.data.Count -gt 0) {
        Write-Host "   Latest: $($response.data[0] | ConvertTo-Json -Compress)"
    }
} catch {
    Write-Host "${Red}❌ Failed:${Reset} $($_.Exception.Message)"
}

# 10. Get specific DApp
Write-Host "`n11. GET $BaseUrl/api/dapps/$dappId"
try {
    $response = Invoke-RestMethod -Uri "$BaseUrl/api/dapps/$dappId" -Method Get
    Write-Host "${Green}✅ Success:${Reset} $($response | ConvertTo-Json -Compress)"
} catch {
    Write-Host "${Red}❌ Failed:${Reset} $($_.Exception.Message)"
}

Write-Host "`n${Green}🎉 API testing completed!${Reset}" -ForegroundColor Green
Write-Host "======================================"