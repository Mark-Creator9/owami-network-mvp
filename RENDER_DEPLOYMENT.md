# Owami Network - Render Deployment Guide

## Overview
This guide enables deploying a decentralized Owami Network testnet with multiple validator nodes on Render, using RocksDB for local storage and libp2p for P2P node discovery and consensus.

## Architecture
- **3+ Validator Nodes**: Each running independently on Render with RocksDB persistence
- **P2P Mesh**: Nodes discover and sync via libp2p gossip protocol
- **No Central Database**: Each node maintains its own blockchain state
- **Load Balancing**: Users can query any node for identical state (via consensus)

## Prerequisites
- Render account (free tier available)
- Docker installed locally
- Git repository with Owami Network code
- GitHub account connected to Render

## Step 1: Prepare Repository

Ensure your repository has:
```
Dockerfile                    # Container image definition
docker-compose.yml           # Multi-node local testing
.env.example                 # Environment template
src/network/p2p.rs          # P2P node implementation
```

## Step 2: Create Render Services

### Node 1 (Bootstrap Node)
1. Go to https://dashboard.render.com
2. Click "New" → "Web Service"
3. Connect your GitHub repository
4. Configure:
   - **Name**: `owami-validator-1`
   - **Environment**: Docker
   - **Region**: Choose closest to users
   - **Plan**: Free (starter)
   - **Auto-deploy**: Enabled

5. Add Environment Variables:
   ```
   NODE_ID=validator-1
   DATA_DIR=/var/data
   RUST_LOG=info
   API_PORT=3002
   ```

6. Configure Disk:
   - Size: 1 GB (for RocksDB state)
   - Mount at: `/var/data`

7. Deploy

### Node 2 & Node 3
Repeat steps 1-7 with:
- **Name**: `owami-validator-2`, `owami-validator-3`
- **NODE_ID**: `validator-2`, `validator-3`
- **BOOTSTRAP_PEERS**: Point to Node 1's public URL

Example bootstrap peer format:
```
BOOTSTRAP_PEERS=/ip4/owami-validator-1.onrender.com/tcp/4001/p2p/Qm...
```

## Step 3: Configure P2P Node Discovery

After all nodes are deployed, note their public URLs:
- Node 1: `https://owami-validator-1.onrender.com`
- Node 2: `https://owami-validator-2.onrender.com`
- Node 3: `https://owami-validator-3.onrender.com`

Update each node's `BOOTSTRAP_PEERS` with other nodes' addresses:

### Node 1
```
BOOTSTRAP_PEERS=/ip4/owami-validator-2.onrender.com/tcp/4001/p2p/QmNode2,/ip4/owami-validator-3.onrender.com/tcp/4001/p2p/QmNode3
```

### Node 2
```
BOOTSTRAP_PEERS=/ip4/owami-validator-1.onrender.com/tcp/4001/p2p/QmNode1,/ip4/owami-validator-3.onrender.com/tcp/4001/p2p/QmNode3
```

### Node 3
```
BOOTSTRAP_PEERS=/ip4/owami-validator-1.onrender.com/tcp/4001/p2p/QmNode1,/ip4/owami-validator-2.onrender.com/tcp/4001/p2p/QmNode2
```

## Step 4: Verify Deployment

### Health Checks
```bash
curl https://owami-validator-1.onrender.com/health
curl https://owami-validator-2.onrender.com/health
curl https://owami-validator-3.onrender.com/health
```

Expected response:
```json
{
  "status": "healthy",
  "network": "owami-testnet",
  "token": "0x742d35Cc6634C0532925a3b8D4e6D3b6e8d3e8A0"
}
```

### Check Node Sync
```bash
# Get blockchain info from each node
curl https://owami-validator-1.onrender.com/api/blockchain/info
curl https://owami-validator-2.onrender.com/api/blockchain/info
curl https://owami-validator-3.onrender.com/api/blockchain/info
```

All nodes should report the same block height and state.

## Step 5: Test Network Operations

### Mine a Block
```bash
curl -X POST https://owami-validator-1.onrender.com/api/blockchain/mine
```

### Check Replication
Query the same endpoint from Node 2 and 3 to verify block was replicated:
```bash
curl https://owami-validator-2.onrender.com/api/blockchain/blocks
curl https://owami-validator-3.onrender.com/api/blockchain/blocks
```

### Transfer Tokens
```bash
curl -X POST https://owami-validator-1.onrender.com/api/token/transfer \
  -H "Content-Type: application/json" \
  -d '{
    "from": "0x123...",
    "to": "0x456...",
    "amount": 100
  }'
```

## Scaling Up

### Add More Validators
Deploy additional Render services following the same pattern.

### Geographic Distribution
Deploy nodes in different Render regions for fault tolerance:
- US East
- Europe
- Asia

## Cost Estimate
- **Free Tier**: $0/month (with limitations: sleep after 15 min inactivity, shared resources)
- **Starter Plan**: ~$7/month per node (always-on, dedicated resources)
- **3 nodes Starter**: ~$21/month

## Monitoring

### View Logs
In Render dashboard → Service → Logs

### Key Metrics to Monitor
- P2P peer connections
- Block propagation time
- Transaction throughput
- Node sync status

Example log search:
```
"Listening on:" - node P2P address
"Received message" - P2P gossip activity
"Connection established" - peer connections
```

## Troubleshooting

### Nodes Not Syncing
1. Verify bootstrap peer addresses are correct
2. Check BOOTSTRAP_PEERS environment variable on each node
3. Restart nodes to re-establish connections
4. View logs for connection errors

### Data Loss on Free Tier
- Free tier sleeps after 15 min inactivity
- Use **Disk** to persist RocksDB state
- Ensure `DATA_DIR` mount is configured

### Performance Issues
- Upgrade from free to starter plan
- Scale horizontally (add more nodes)
- Optimize P2P gossip settings in libp2p config

## Next Steps
- Implement state snapshots for faster node bootstrapping
- Add metrics/monitoring via Prometheus
- Deploy light client support for mobile
- Test with larger validator set (10+ nodes)

## References
- Render Docs: https://render.com/docs
- libp2p Rust: https://docs.rs/libp2p
- RocksDB: https://github.com/facebook/rocksdb/wiki
