# 🚀 Owami Network Deployment Guide

This guide will help you deploy Owami Network to Render for global access.

## 📋 Prerequisites

1. **Render Account** - Sign up at https://render.com
2. **GitHub Repository** - Push your code to GitHub
3. **Rust Installed** - For local testing

## 🔧 Step 1: Prepare for Deployment

### 1.1 Review render.yaml
The `render.yaml` file is already configured with:
- Web service (port 8081)
- Environment variables
- Docker configuration
- 10GB disk storage for RocksDB

### 1.2 Environment Variables (Optional)
Add these to Render dashboard if needed:

```bash
# Core Configuration
PORT=8081
HOST=0.0.0.0
NODE_ENV=production
RUST_LOG=info

# CORS Settings
CORS_ORIGINS=https://owami-network.onrender.com
CORS_ALLOW_CREDENTIALS=true

# Rate Limiting
RATE_LIMIT_REQUESTS=100
RATE_LIMIT_WINDOW=60

# Faucet Settings
FAUCET_AMOUNT=1000
FAUCET_RATE_LIMIT=86400  # 24 hours in seconds

# WASM Settings
WASM_CACHE_SIZE=100MB
WASM_GAS_LIMIT=1000000
```

## 📦 Step 2: Deploy to Render

### Option A: Automatic Deployment via render.yaml

1. **Go to Render Dashboard**
   - Navigate to https://dashboard.render.com
   
2. **Create New Web Service**
   - Click "New +"
   - Select "Web Service"
   - Connect your GitHub repository
   
3. **Select Branch**
   - Choose `main` or `master` branch
   - Render will auto-detect `render.yaml`

4. **Configure Service**
   ```
   Name: owami-network
   Region: Oregon (or nearest to your users)
   Branch: main
   Build Command: [Detected from render.yaml]
   Start Command: [Detected from render.yaml]
   ```

5. **Environment Variables**
   - Add any custom environment variables from Step 1.2
   - Auto-generated variables:
     - `JWT_SECRET` (auto-generated)
     - `RENDER_EXTERNAL_URL` (auto-generated)

6. **Deploy!**
   - Click "Create Web Service"
   - Wait 5-10 minutes for build
   - Monitor deployment logs

### Option B: Manual Deployment

1. **Go to Render Dashboard**
   - Navigate to https://dashboard.render.com

2. **Create New Web Service**
   - Click "New +" → "Web Service"

3. **Configure Manually**
   ```
   Name: owami-network
   Region: Oregon
   Runtime: Docker
   Repository: [Your GitHub Repo]
   Branch: main
   Docker Context: /
   DockerfilePath: ./Dockerfile
   ```

4. **Add Environment Variables**
   ```bash
   PORT=8081
   HOST=0.0.0.0
   NODE_ENV=production
   ```

5. **Deploy!**

## ✅ Step 3: Verify Deployment

### 3.1 Check Health Status
```bash
curl https://owami-network.onrender.com/health
```

Expected response:
```json
{
  "status": "healthy",
  "network": "owami-testnet",
  "timestamp": "2024-01-15T10:30:00Z",
  "database": "connected",
  "wasm_support": true
}
```

### 3.2 Test Frontend
Open in browser:
```
https://owami-network.onrender.com/mvp.html
```

### 3.3 Test API
```bash
# Create wallet
curl -X POST https://owami-network.onrender.com/api/wallet/create \
  -H "Content-Type: application/json" \
  -d '{"username":"testuser","password":"testpass"}'

# Get blockchain info
curl https://owami-network.onrender.com/api/blockchain/info

# Get explorer stats
curl https://owami-network.onrender.com/api/explorer/stats
```

## 🔒 Step 4: Security Configuration

### 4.1 Update CORS Origins
In Render dashboard, update:
```bash
CORS_ORIGINS=https://owami-network.onrender.com,https://yourdomain.com
```

### 4.2 Generate Secure JWT Secret
```bash
# In Render dashboard
JWT_SECRET=<your-secure-random-string>
```

### 4.3 Enable Rate Limiting
```bash
RATE_LIMIT_REQUESTS=100
RATE_LIMIT_WINDOW=60
```

### 4.4 Set Faucet Rate Limit
```bash
FAUCET_RATE_LIMIT=86400  # 24 hours
```

## 📊 Step 5: Monitor Your Deployment

### 5.1 Render Dashboard
- **Logs**: View real-time logs
- **Metrics**: CPU, Memory, Disk usage
- **Events**: Build and deployment history

### 5.2 Health Checks
```bash
# Automated health check (add to cron)
curl https://owami-network.onrender.com/health
```

### 5.3 Error Tracking (Optional)
Add Sentry for production error tracking:
```rust
// In Cargo.toml
[dependencies]
sentry = "0.31"

// In main.rs
let _guard = sentry::init((
    "YOUR_SENTRY_DSN",
    sentry::ClientOptions {
        release: env!("CARGO_PKG_VERSION").into(),
        ..Default::default()
    }
));
```

## 🌍 Step 6: Share with Testers

### 6.1 Share URL
```
Frontend: https://owami-network.onrender.com/mvp.html
API:      https://owami-network.onrender.com/api
Explorer: https://owami-network.onrender.com/api/explorer/stats
```

### 6.2 Create Welcome Message
Share with your testers:
```
🌍 Welcome to Owami Network Testnet!

🔗 Access: https://owami-network.onrender.com/mvp.html

✨ Features:
- Create wallets
- Get test tokens (1000 OWA every 24h)
- Send tokens globally
- Mine blocks
- Explore blockchain
- Deploy DApps (coming soon)

📖 Documentation: [Add your docs link]
💬 Feedback: [Add feedback form/discord link]

Happy testing! 🚀
```

## 🔧 Troubleshooting

### Issue: Build Fails
**Solution**: Check Dockerfile
```bash
# Ensure Dockerfile exists and is valid
cat Dockerfile
```

### Issue: Port Not Accessible
**Solution**: Verify port binding
```bash
# Ensure port 8081 is exposed
PORT=8081
```

### Issue: Database Errors
**Solution**: Check disk space
- Render provides 10GB (free tier)
- Monitor RocksDB growth

### Issue: CORS Errors
**Solution**: Update CORS origins
```bash
CORS_ORIGINS=https://owami-network.onrender.com,*
```

### Issue: Faucet Rate Limiting
**Solution**: Adjust rate limit
```bash
FAUCET_RATE_LIMIT=3600  # 1 hour for testing
```

## 📝 Post-Deployment Checklist

- [ ] Health check passes
- [ ] Frontend loads correctly
- [ ] Wallet creation works
- [ ] Token transfers work
- [ ] Block mining works
- [ ] Explorer shows data
- [ ] Rate limiting works
- [ ] CORS configured correctly
- [ ] Logs show no errors
- [ ] SSL/HTTPS enabled (automatic on Render)
- [ ] Performance is acceptable
- [ ] Shared with testers

## 🚀 Next Steps

### 1. Add Analytics
Add Google Analytics or Plausible to track usage:
```html
<!-- In landing/mvp.html -->
<script async src="https://www.googletagmanager.com/gtag/js?id=GA_TRACKING_ID"></script>
```

### 2. Set Up Monitoring
- Render metrics (built-in)
- Uptime monitoring (UptimeRobot)
- Error tracking (Sentry)

### 3. Create Feedback Form
Add feedback collection:
```javascript
// In mvp.html
async function submitFeedback() {
    await fetch('/api/feedback', {
        method: 'POST',
        body: JSON.stringify({
            rating,
            comments,
            wallet_address: currentWallet?.address
        })
    });
}
```

### 4. Domain Customization
- Add custom domain: owami.network
- Configure DNS A record
- Enable SSL (automatic on Render)

### 5. Scale Up
If traffic increases:
- Upgrade to paid Render tier
- Add multiple instances
- Add load balancer

## 📚 Additional Resources

- **Render Documentation**: https://render.com/docs
- **Rust Deployment**: https://render.com/docs/deploy-rust
- **Docker Guide**: https://render.com/docs/docker
- **Monitoring**: https://render.com/docs/monitoring

## 💬 Support

For issues or questions:
- **GitHub Issues**: https://github.com/your-repo/issues
- **Documentation**: [Add docs link]
- **Email**: [Add contact email]

---

**Deployment Date**: [ADD_DATE]
**Deployed By**: [ADD_NAME]
**Version**: 1.0.0
