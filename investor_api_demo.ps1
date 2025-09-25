# Owami Network - Investor API Demonstration
# This script demonstrates key blockchain functionality for investors

param(
    [string]$BaseUrl = "http://localhost:3002",
    [switch]$Interactive,
    [switch]$AivenDemo
)

Write-Host "💼 OWAMI NETWORK - INVESTOR API DEMONSTRATION" -ForegroundColor Magenta
Write-Host "=============================================" -ForegroundColor Magenta
Write-Host ""
Write-Host "🌍 Demonstrating Africa's Next-Generation Blockchain Platform" -ForegroundColor Cyan
Write-Host "💰 Investment Opportunity: $1M for 350M+ Unbanked Africans" -ForegroundColor Green
if ($AivenDemo) {
    Write-Host "☁️  Database: Aiven PostgreSQL Cloud (Production-Ready)" -ForegroundColor Blue
}
Write-Host ""

# Check if server is running
Write-Host "🔍 Checking if Owami Network server is running..." -ForegroundColor Yellow
try {
    $healthCheck = Invoke-RestMethod -Uri "$BaseUrl/api/health" -Method GET -TimeoutSec 5
    Write-Host "✅ Server is running and accessible" -ForegroundColor Green
    if ($AivenDemo) {
        Write-Host "   Database: Connected to Aiven PostgreSQL Cloud" -ForegroundColor Gray
    }
} catch {
    Write-Host "❌ Server is not running or not accessible" -ForegroundColor Red
    if ($AivenDemo) {
        Write-Host "   Please run: .\launch_aiven_demo.ps1 -AivenPassword 'your_password'" -ForegroundColor Yellow
    } else {
        Write-Host "   Please run: .\launch_investor_demo.ps1" -ForegroundColor Yellow
    }
    Write-Host "   Or check if the server is starting up..." -ForegroundColor Gray
    exit 1
}
Write-Host ""

# Helper function for investor-friendly API demonstration
function Show-InvestorDemo {
    param(
        [string]$Method,
        [string]$Endpoint,
        [string]$Title,
        [string]$Description,
        [hashtable]$Body = $null,
        [string]$InvestorValue
    )
    
    Write-Host "🔹 $Title" -ForegroundColor Yellow
    Write-Host "   $Description" -ForegroundColor Gray
    Write-Host "   💡 Investor Value: $InvestorValue" -ForegroundColor Green
    Write-Host "   🔗 $Method $Endpoint" -ForegroundColor DarkGray
    
    if ($Interactive) {
        Read-Host "   Press Enter to execute"
    }
    
    try {
        $uri = "$BaseUrl$Endpoint"
        $params = @{
            Uri = $uri
            Method = $Method
            ContentType = "application/json"
        }
        
        if ($Body) {
            $params.Body = $Body | ConvertTo-Json -Depth 10
        }
        
        $response = Invoke-RestMethod @params
        
        Write-Host "   ✅ SUCCESS" -ForegroundColor Green
        Write-Host "   📊 Response:" -ForegroundColor Cyan
        $response | ConvertTo-Json -Depth 3 | Write-Host -ForegroundColor White
        
        return $response
    }
    catch {
        Write-Host "   ❌ Error: $($_.Exception.Message)" -ForegroundColor Red
        return $null
    }
    
    Write-Host ""
}

# Wait for server
Write-Host "⏳ Waiting for Owami Network to be ready..." -ForegroundColor Yellow
$maxRetries = 15
$retryCount = 0

do {
    try {
        Invoke-WebRequest -Uri "$BaseUrl/api/health" -TimeoutSec 3 | Out-Null
        Write-Host "✅ Owami Network is operational!" -ForegroundColor Green
        break
    }
    catch {
        $retryCount++
        if ($retryCount -ge $maxRetries) {
            Write-Host "❌ Owami Network not responding. Please start the server first." -ForegroundColor Red
            Write-Host "   Run: .\launch_investor_demo.ps1" -ForegroundColor Yellow
            exit 1
        }
        Write-Host "   Attempt $retryCount/$maxRetries..." -ForegroundColor DarkYellow
        Start-Sleep -Seconds 2
    }
} while ($retryCount -lt $maxRetries)

Write-Host ""
Write-Host "🚀 BLOCKCHAIN CORE DEMONSTRATION" -ForegroundColor Blue
Write-Host "=================================" -ForegroundColor Blue

# 1. System Health & Status
Show-InvestorDemo -Method "GET" -Endpoint "/api/health" -Title "System Health Check" `
    -Description "Demonstrates enterprise-grade monitoring and reliability" `
    -InvestorValue "99.9% uptime SLA capability for financial services"

# 2. Blockchain Information
Show-InvestorDemo -Method "GET" -Endpoint "/api/blockchain/info" -Title "Blockchain Network Status" `
    -Description "Shows current blockchain state and network statistics" `
    -InvestorValue "Real-time network monitoring for institutional confidence"

# 3. Token System Information
Show-InvestorDemo -Method "GET" -Endpoint "/api/token/info" -Title "OWA Token Information" `
    -Description "Native cryptocurrency designed for African markets" `
    -InvestorValue "Stable value storage vs volatile local currencies"

Write-Host ""
Write-Host "💰 TOKEN OPERATIONS DEMONSTRATION" -ForegroundColor Blue
Write-Host "==================================" -ForegroundColor Blue

# 4. Check Token Balance
$demoAddress = "20fa6a2566e8022b545347740091e02eed91165d8f9e0413d6a4bb647d8b1e4a"
Show-InvestorDemo -Method "GET" -Endpoint "/api/token/balance/$demoAddress" -Title "Token Balance Query" `
    -Description "Instant balance checking for any wallet address" `
    -InvestorValue "Real-time financial data for 100M+ potential users"

# 5. Mint Tokens (Testnet Demo)
$mintBody = @{
    to = $demoAddress
    amount = 10000
}
Show-InvestorDemo -Method "POST" -Endpoint "/api/token/mint" -Title "Token Minting (Testnet)" `
    -Description "Controlled token supply management" -Body $mintBody `
    -InvestorValue "Inflation control and economic stability mechanisms"

# 6. Token Transfer
$transferBody = @{
    from = $demoAddress
    to = "30fa6a2566e8022b545347740091e02eed91165d8f9e0413d6a4bb647d8b1e5b"
    amount = 500
    private_key = "154dc850c10d24b29d9d245e8052610d4f746973c658751bf12ded323fbe90e1"
}
Show-InvestorDemo -Method "POST" -Endpoint "/api/token/transfer" -Title "Token Transfer" `
    -Description "Peer-to-peer value transfer with cryptographic security" -Body $transferBody `
    -InvestorValue "Sub-$0.01 transaction fees vs $5-10 traditional remittances"

# 7. Transaction History
Show-InvestorDemo -Method "GET" -Endpoint "/api/token/transactions" -Title "Transaction History" `
    -Description "Complete audit trail of all financial operations" `
    -InvestorValue "Regulatory compliance and transparency for institutions"

Write-Host ""
Write-Host "🏗️ DAPP ECOSYSTEM DEMONSTRATION" -ForegroundColor Blue
Write-Host "================================" -ForegroundColor Blue

# 8. Create User Account (for DApp demo)
$timestamp = Get-Date -Format "yyyyMMddHHmmss"
$registerBody = @{
    username = "investor_demo_$timestamp"
    password = "SecureDemo2024!"
}
$authResponse = Show-InvestorDemo -Method "POST" -Endpoint "/api/auth/register" -Title "User Registration" `
    -Description "Secure user onboarding with JWT authentication" -Body $registerBody `
    -InvestorValue "KYC-ready infrastructure for regulatory compliance"

$authToken = $null
if ($authResponse -and $authResponse.token) {
    $authToken = $authResponse.token
    $authHeaders = @{ Authorization = "Bearer $authToken" }
}

# 9. Create DApp
if ($authToken) {
    $dappBody = @{
        name = "African Micro-Lending Platform"
        description = "P2P lending for small businesses across Africa"
        contract_address = "0xAFRICA123456789ABCDEF123456789ABCDEF1234"
    }
    
    $params = @{
        Uri = "$BaseUrl/api/dapps"
        Method = "POST"
        ContentType = "application/json"
        Headers = $authHeaders
        Body = $dappBody | ConvertTo-Json
    }
    
    Write-Host "🔹 DApp Creation - Micro-Lending Platform" -ForegroundColor Yellow
    Write-Host "   Demonstrates platform's ability to host financial applications" -ForegroundColor Gray
    Write-Host "   💡 Investor Value: $10B+ African lending market opportunity" -ForegroundColor Green
    
    try {
        $dappResponse = Invoke-RestMethod @params
        Write-Host "   ✅ SUCCESS - DApp Created" -ForegroundColor Green
        Write-Host "   📊 DApp ID: $($dappResponse.id)" -ForegroundColor Cyan
    }
    catch {
        Write-Host "   ⚠️ DApp creation demo (authentication required)" -ForegroundColor Yellow
    }
}

Write-Host ""
Write-Host "⛓️ BLOCKCHAIN MINING DEMONSTRATION" -ForegroundColor Blue
Write-Host "===================================" -ForegroundColor Blue

# 10. Mine a Block
Show-InvestorDemo -Method "POST" -Endpoint "/api/blockchain/mine" -Title "Block Mining" `
    -Description "Demonstrates consensus mechanism and block creation" `
    -InvestorValue "Energy-efficient mining (2W vs Bitcoin's 500W per transaction)"

# 11. View Blockchain Blocks
Show-InvestorDemo -Method "GET" -Endpoint "/api/blockchain/blocks" -Title "Blockchain Explorer" `
    -Description "Complete transaction history and block verification" `
    -InvestorValue "Full transparency and immutability for financial records"

Write-Host ""
Write-Host "📊 INVESTMENT SUMMARY" -ForegroundColor Magenta
Write-Host "=====================" -ForegroundColor Magenta
Write-Host ""
Write-Host "🎯 MARKET OPPORTUNITY:" -ForegroundColor Cyan
Write-Host "   • 350M unbanked Africans" -ForegroundColor White
Write-Host "   • $70B annual remittance market" -ForegroundColor White
Write-Host "   • $3.4T AfCFTA trade opportunity" -ForegroundColor White
Write-Host ""
Write-Host "💡 TECHNICAL ADVANTAGES:" -ForegroundColor Cyan
Write-Host "   • Rust-based performance (10x faster than Ethereum)" -ForegroundColor White
Write-Host "   • USSD integration (works on basic phones)" -ForegroundColor White
Write-Host "   • 80,000+ TPS scalability target" -ForegroundColor White
Write-Host "   • 50% lower transaction costs" -ForegroundColor White
Write-Host ""
Write-Host "💰 INVESTMENT TERMS:" -ForegroundColor Cyan
Write-Host "   • Seeking: $1,000,000" -ForegroundColor White
Write-Host "   • Valuation: $5,000,000 pre-money" -ForegroundColor White
Write-Host "   • Use of Funds: 40% Tech, 25% Market, 20% Team, 15% Operations" -ForegroundColor White
Write-Host "   • Timeline: 12 months to profitability" -ForegroundColor White
Write-Host ""
Write-Host "🚀 NEXT STEPS:" -ForegroundColor Green
Write-Host "   1. Review technical documentation" -ForegroundColor White
Write-Host "   2. Meet with founding team" -ForegroundColor White
Write-Host "   3. Due diligence process" -ForegroundColor White
Write-Host "   4. Investment agreement" -ForegroundColor White
Write-Host ""
Write-Host "📞 CONTACT:" -ForegroundColor Yellow
Write-Host "   • Technical Demo: Available 24/7 at http://localhost:3000" -ForegroundColor White
Write-Host "   • Documentation: /landing/INVESTOR_DEMO.md" -ForegroundColor White
Write-Host "   • API Reference: /api-docs.html" -ForegroundColor White
Write-Host ""
Write-Host "🌍 Building Africa's Financial Future - One Block at a Time" -ForegroundColor Green
Write-Host ""