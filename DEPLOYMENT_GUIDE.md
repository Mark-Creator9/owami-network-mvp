# Owami Network - Deployment Guide

## 🚀 Quick Deploy to Render.com

### Option 1: Automatic Deployment (Recommended)

1. **Push to GitHub**:
   ```bash
   git add .
   git commit -m "Professional MVP ready for deployment"
   git push origin main
   ```

2. **Deploy on Render**:
   - Go to [render.com](https://render.com)
   - Connect your GitHub repository
   - Use the `render.yaml` configuration file
   - Deploy both backend and frontend automatically

### Option 2: Manual Deployment

#### Backend Deployment
1. Create a new Web Service on Render
2. Connect your GitHub repo
3. Configure:
   - **Build Command**: `cargo build --release`
   - **Start Command**: `./target/release/owami-network`
   - **Environment**: Rust
   - **Port**: 3002

#### Frontend Deployment
1. Create a new Static Site on Render
2. Set **Publish Directory**: `landing`
3. Add environment variable:
   - `API_BASE_URL`: Your backend URL

### Option 3: Alternative Platforms

#### Vercel (Frontend) + Railway (Backend)
- **Frontend**: Deploy `landing` folder to Vercel
- **Backend**: Deploy Rust app to Railway
- Update API_BASE in frontend to point to Railway URL

#### Netlify (Frontend) + Heroku (Backend)
- **Frontend**: Deploy to Netlify with build redirects
- **Backend**: Deploy to Heroku with Rust buildpack

## 🔧 Configuration Updates for Production

### 1. Update API Base URL
In `mvp.html`, update line 5 of the JavaScript:
```javascript
const API_BASE = 'https://your-backend-url.onrender.com/api';
```

### 2. CORS Configuration
Ensure your backend allows your frontend domain:
```rust
// In your Rust backend
.allowed_origin("https://your-frontend-url.onrender.com")
```

### 3. Database Configuration
- Use Render PostgreSQL or external database
- Update connection string in environment variables

## 🌐 Access Your Deployed MVP

After deployment, your MVP will be available at:
- **Frontend**: `https://your-app-name.onrender.com/mvp.html`
- **Backend API**: `https://your-backend-name.onrender.com/api/health`

## 📱 Features Available in Production

✅ **Working Features**:
- Network health monitoring
- Wallet creation and management
- OWA token faucet (1000 test tokens)
- Global token transfers
- Block mining
- DApp deployment platform
- Real-time activity logging
- Mobile-responsive design

## 🔗 Share Your MVP

Once deployed, share these URLs:
- **Main Demo**: `https://your-app.onrender.com/mvp.html`
- **API Health**: `https://your-backend.onrender.com/api/health`
- **Investor Demo**: Perfect for presentations and feedback

## 🛠️ Local Development

To run locally:
```bash
# Backend
cargo run

# Frontend
# Serve the landing folder on any HTTP server
# The MVP will be at: http://localhost:3002/mvp.html
```

## 📊 Monitoring

- Check backend logs in Render dashboard
- Monitor API health endpoint
- Track user activity through the activity log

Your professional MVP is now ready for global deployment! 🌍