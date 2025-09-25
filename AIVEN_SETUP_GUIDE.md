# Owami Network - Aiven PostgreSQL Setup Guide

## 🌟 Production-Ready Cloud Database for Investor Demo

This guide helps you set up Owami Network with Aiven PostgreSQL cloud database for a professional investor demonstration.

## 📋 Prerequisites

1. **Aiven PostgreSQL Database** (already created)
   - Host: `pg-d1beeb4-marknyathi08-1cb1.h.aivencloud.com:16963`
   - Database: `defaultdb`
   - Username: `avnadmin`
   - SSL: Required

2. **PostgreSQL Client Tools**
   - The script will attempt to install these automatically
   - Or install manually: `winget install PostgreSQL.PostgreSQL`

3. **Rust Development Environment**
   - Already installed and verified

## 🚀 Quick Start

### Step 1: Get Your Aiven Password
1. Log into your Aiven console
2. Navigate to your PostgreSQL service
3. Copy the password for the `avnadmin` user

### Step 2: Test Database Connection
```powershell
# Test connection manually (optional)
psql 'postgres://avnadmin:YOUR_PASSWORD@pg-d1beeb4-marknyathi08-1cb1.h.aivencloud.com:16963/defaultdb?sslmode=require'
```

### Step 3: Launch Owami Network with Aiven
```powershell
# Launch the investor demo with Aiven database
.\launch_aiven_demo.ps1 -AivenPassword "YOUR_AIVEN_PASSWORD"

# Or set environment variable and launch
$env:AIVEN_PASSWORD = "YOUR_AIVEN_PASSWORD"
.\launch_aiven_demo.ps1 -AivenPassword $env:AIVEN_PASSWORD
```

### Step 4: Run Investor API Demo
```powershell
# Run the comprehensive API demonstration
.\investor_api_demo.ps1 -AivenDemo -Interactive

# Or run non-interactive
.\investor_api_demo.ps1 -AivenDemo
```

## 🔧 Configuration Files

### Environment Configuration (`.env.aiven`)
```env
DATABASE_URL=postgres://avnadmin:YOUR_PASSWORD@pg-d1beeb4-marknyathi08-1cb1.h.aivencloud.com:16963/defaultdb?sslmode=require
RUST_LOG=info
CONFIG_PATH=config/production.toml
```

### Production Configuration (`config/production.toml`)
```toml
[server]
host = "0.0.0.0"
port = 3002
workers = 4

[database]
pool_size = 20
timeout_seconds = 10

[security]
cors_origins = ["https://owami.network"]
rate_limiting = { requests = 100, per_seconds = 60 }
```

## 🌐 Access URLs (After Launch)

- **Landing Page**: http://localhost:3002
- **API Health Check**: http://localhost:3002/api/health
- **Blockchain Info**: http://localhost:3002/api/blockchain/info
- **Token Information**: http://localhost:3002/api/token/info
- **API Documentation**: http://localhost:3002/api-docs.html

## 💼 Investor Demo Features

### 🔹 Cloud-Ready Infrastructure
- **Production Database**: Aiven PostgreSQL with SSL/TLS
- **Scalable Architecture**: Layer-0 blockchain design
- **Enterprise Security**: ED25519 cryptography
- **High Availability**: Cloud-native deployment ready

### 🔹 African Market Focus
- **USSD Integration**: Works with basic phones (no internet required)
- **Target Market**: 350+ million unbanked Africans
- **Local Optimization**: African network conditions considered
- **Financial Inclusion**: Designed for emerging markets

### 🔹 Technical Capabilities
- **High Throughput**: 80,000+ TPS target
- **Native Token**: OWA token system
- **DApp Platform**: Full development environment
- **REST API**: Complete integration capabilities
- **Real-time Mining**: Live blockchain demonstration

## 🧪 Testing & Validation

### Automated API Testing
```powershell
# Run comprehensive API tests
.\test_api_endpoints.ps1 -BaseUrl "http://localhost:3002" -Verbose
```

### Manual Testing Checklist
- [ ] Database connection successful
- [ ] Server starts without errors
- [ ] API endpoints respond correctly
- [ ] Blockchain mining works
- [ ] Token operations function
- [ ] DApp creation/management works

## 🔍 Troubleshooting

### Common Issues

1. **Connection Failed**
   ```
   Error: Connection to Aiven failed
   Solution: Verify password and network connectivity
   ```

2. **psql Not Found**
   ```
   Error: psql command not recognized
   Solution: Install PostgreSQL client tools
   ```

3. **Migration Errors**
   ```
   Error: Database migration failed
   Solution: Check database permissions and schema
   ```

### Debug Commands
```powershell
# Test database connection
psql $env:DATABASE_URL -c "SELECT version();"

# Check server logs
$env:RUST_LOG = "debug"
cargo run --release --bin owami-network

# Verify configuration
Get-Content config/production.toml
```

## 📊 Performance Metrics

### Expected Performance (Aiven Cloud)
- **Database Latency**: < 50ms (cloud optimized)
- **API Response Time**: < 100ms average
- **Concurrent Users**: 1000+ supported
- **Transaction Throughput**: 80,000+ TPS target
- **Uptime**: 99.9% (Aiven SLA)

## 💰 Investment Highlights

### Market Opportunity
- **Total Addressable Market**: 350M+ unbanked Africans
- **Unique Technology**: USSD blockchain access
- **First Mover Advantage**: African-focused Layer-0 platform
- **Scalability**: Cloud-native architecture

### Technical Differentiators
- **USSD Integration**: No smartphone required
- **Layer-0 Architecture**: Maximum scalability
- **Production Database**: Aiven PostgreSQL cloud
- **Enterprise Security**: Military-grade cryptography
- **African Optimization**: Network and usage patterns

### Investment Terms
- **Amount**: $1,000,000
- **Equity**: 20%
- **Use of Funds**: Development, marketing, African expansion
- **Timeline**: 18-month roadmap to production

## 📞 Next Steps

1. **Review Demo**: Complete API demonstration
2. **Technical Deep Dive**: Architecture and scalability discussion
3. **Market Analysis**: African blockchain adoption potential
4. **Investment Discussion**: Terms and timeline
5. **Due Diligence**: Technical and business validation

---

**Contact Information**
- **Project**: Owami Network
- **Focus**: African Blockchain Infrastructure
- **Technology**: Layer-0 + USSD Integration
- **Investment**: $1M for African Market Revolution