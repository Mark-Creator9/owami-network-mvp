// Modern Owami Network Frontend Application
class OwamiApp {
  constructor() {
    // API Configuration - Updated to match your backend port
    // Assume app is served from same host/port as backend
    this.API_BASE = '/api';
    this.ENDPOINTS = {
      // Health and status
      health: '/health',
      status: '/status',
      
      // Authentication
      register: '/auth/register',
      login: '/auth/login',
      profile: '/auth/profile',
      
      // Blockchain
      blockchainInfo: '/blockchain/info',
      blocks: '/blockchain/blocks',
      mineBlock: '/blockchain/mine',
      
      // Token operations
      tokenInfo: '/token/info',
      balance: '/token/balance',
      transfer: '/token/transfer',
      mint: '/token/mint',
      transactions: '/token/transactions',
      
      // DApp management
      dapps: '/dapps',
      deploy: '/deploy',
      call: '/call'
    };

    // Application state
    this.state = {
      wallet: null,
      balance: 0,
      networkStatus: 'connecting',
      blocks: [],
      transactions: [],
      dapps: [],
      user: null
    };

    // Initialize the application
    this.init();
  }

  async init() {
    console.log('🚀 Initializing Owami Network Application...');
    
    // Bind event listeners
    this.bindEvents();
    
    // Check network status
    await this.checkNetworkStatus();
    
    // Load saved wallet if exists
    this.loadSavedWallet();
    
    // Load initial data
    await this.loadInitialData();
    
    // Start periodic updates
    this.startPeriodicUpdates();
    
    console.log('✅ Owami Network Application initialized successfully');
  }

  bindEvents() {
    // Wallet events
    document.getElementById('connect-wallet')?.addEventListener('click', () => this.connectWallet());
    document.getElementById('create-wallet')?.addEventListener('click', () => this.createWallet());
    document.getElementById('refresh-wallet')?.addEventListener('click', () => this.refreshWallet());
    
    // Token events
    document.getElementById('get-tokens')?.addEventListener('click', () => this.requestTokens());
    document.getElementById('send-tokens-btn')?.addEventListener('click', () => this.showSendForm());
    document.getElementById('receive-tokens-btn')?.addEventListener('click', () => this.showReceiveModal());
    document.getElementById('send-form')?.addEventListener('submit', (e) => this.handleSendTokens(e));
    
    // Blockchain events
    document.getElementById('explore-network')?.addEventListener('click', () => this.scrollToSection('explorer-section'));
    document.getElementById('mine-block')?.addEventListener('click', () => this.mineBlock());
    document.getElementById('refresh-blocks')?.addEventListener('click', () => this.loadBlocks());
    
    // DApp events
    document.getElementById('create-dapp')?.addEventListener('click', () => this.scrollToSection('dapp-section'));
    document.getElementById('create-new-dapp')?.addEventListener('click', () => this.showDAppForm());
    document.getElementById('dapp-form')?.addEventListener('submit', (e) => this.handleCreateDApp(e));
    
    // Transaction events
    document.getElementById('refresh-transactions')?.addEventListener('click', () => this.loadTransactions());
    
    // Modal events
    document.getElementById('close-qr-modal')?.addEventListener('click', () => this.hideModal('qr-modal'));
    document.getElementById('copy-address')?.addEventListener('click', () => this.copyAddress());
    
    // Footer links
    document.getElementById('api-docs-link')?.addEventListener('click', () => window.open('/api-docs.html', '_blank'));
    document.getElementById('testnet-guide-link')?.addEventListener('click', () => window.open('/TESTNET_GUIDE.md', '_blank'));
    document.getElementById('investor-demo-link')?.addEventListener('click', () => window.open('/INVESTOR_DEMO.md', '_blank'));
  }

  async checkNetworkStatus() {
    try {
      const response = await fetch(`${this.API_BASE}${this.ENDPOINTS.health}`);
      if (response.ok) {
        const data = await response.json();
        this.updateNetworkStatus('connected', data);
        console.log('🌐 Network Status:', data);
      } else {
        throw new Error('Health check failed');
      }
    } catch (error) {
      console.error('❌ Network connection failed:', error);
      this.updateNetworkStatus('error', null);
    }
  }

  updateNetworkStatus(status, data = null) {
    this.state.networkStatus = status;
    
    const statusElement = document.getElementById('network-status');
    const footerStatus = document.getElementById('footer-status');
    const indicator = statusElement?.querySelector('.status-indicator');
    
    if (statusElement && indicator) {
      indicator.className = `status-indicator ${status}`;
      
      switch (status) {
        case 'connected':
          statusElement.querySelector('span').textContent = 'Connected';
          footerStatus.textContent = 'Online';
          break;
        case 'connecting':
          statusElement.querySelector('span').textContent = 'Connecting...';
          footerStatus.textContent = 'Connecting';
          break;
        case 'error':
          statusElement.querySelector('span').textContent = 'Disconnected';
          footerStatus.textContent = 'Offline';
          break;
      }
    }
  }

  loadSavedWallet() {
    const savedWallet = localStorage.getItem('owami_wallet');
    if (savedWallet) {
      try {
        this.state.wallet = JSON.parse(savedWallet);
        this.updateWalletUI();
        console.log('💼 Loaded saved wallet');
      } catch (error) {
        console.error('❌ Failed to load saved wallet:', error);
        localStorage.removeItem('owami_wallet');
      }
    }
  }

  async loadInitialData() {
    if (this.state.networkStatus === 'connected') {
      await Promise.all([
        this.loadBlockchainInfo(),
        this.loadBlocks(),
        this.loadDApps(),
        this.state.wallet ? this.loadBalance() : Promise.resolve(),
        this.state.wallet ? this.loadTransactions() : Promise.resolve()
      ]);
    }
  }

  startPeriodicUpdates() {
    // Update network status every 30 seconds
    setInterval(() => this.checkNetworkStatus(), 30000);
    
    // Update blockchain data every 10 seconds
    setInterval(() => {
      if (this.state.networkStatus === 'connected') {
        this.loadBlockchainInfo();
        this.loadBlocks();
      }
    }, 10000);
    
    // Update wallet data every 15 seconds if wallet is connected
    setInterval(() => {
      if (this.state.wallet && this.state.networkStatus === 'connected') {
        this.loadBalance();
        this.loadTransactions();
      }
    }, 15000);
  }

  async connectWallet() {
    // For demo purposes, we'll create a new wallet
    await this.createWallet();
  }

  async createWallet() {
    this.showLoading('Creating wallet...');
    
    try {
      // Generate a unique username for demo
      const username = `user_${Date.now()}_${Math.random().toString(36).substring(2, 8)}`;
      const password = this.generateSecurePassword();
      
      const response = await fetch(`${this.API_BASE}${this.ENDPOINTS.register}`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          username: username,
          password: password
        })
      });

      if (!response.ok) {
        const errorData = await response.json().catch(() => ({}));
        throw new Error(errorData.message || `Registration failed: ${response.status}`);
      }

      const result = await response.json();
      
      // Create wallet object with demo data
      const wallet = {
        username: username,
        address: this.generateDemoAddress(),
        privateKey: this.generateDemoPrivateKey(),
        createdAt: new Date().toISOString()
      };

      this.state.wallet = wallet;
      localStorage.setItem('owami_wallet', JSON.stringify(wallet));
      
      this.updateWalletUI();
      await this.loadBalance();
      
      this.showToast('Wallet created successfully!', 'success');
      console.log('✅ Wallet created:', wallet);
      
    } catch (error) {
      console.error('❌ Wallet creation failed:', error);
      this.showToast(`Failed to create wallet: ${error.message}`, 'error');
    } finally {
      this.hideLoading();
    }
  }

  generateSecurePassword() {
    const chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*';
    let password = '';
    for (let i = 0; i < 16; i++) {
      password += chars.charAt(Math.floor(Math.random() * chars.length));
    }
    return password;
  }

  generateDemoAddress() {
    return '0x' + Array.from({length: 40}, () => Math.floor(Math.random() * 16).toString(16)).join('');
  }

  generateDemoPrivateKey() {
    return '0x' + Array.from({length: 64}, () => Math.floor(Math.random() * 16).toString(16)).join('');
  }

  updateWalletUI() {
    const walletInfo = document.getElementById('wallet-info');
    const walletStatus = document.getElementById('wallet-status');
    
    if (this.state.wallet && walletInfo) {
      walletInfo.innerHTML = `
        <div class="wallet-details">
          <div class="wallet-field">
            <label>Username</label>
            <div class="wallet-address">${this.state.wallet.username}</div>
          </div>
          <div class="wallet-field">
            <label>Wallet Address</label>
            <div class="wallet-address">${this.state.wallet.address}</div>
          </div>
          <div class="wallet-field">
            <label>Created</label>
            <div class="wallet-address">${new Date(this.state.wallet.createdAt).toLocaleString()}</div>
          </div>
        </div>
      `;
      
      if (walletStatus) {
        walletStatus.innerHTML = `
          <span class="status-dot connected"></span>
          <span>Connected</span>
        `;
      }
    }
  }

  async refreshWallet() {
    if (this.state.wallet) {
      await Promise.all([
        this.loadBalance(),
        this.loadTransactions()
      ]);
      this.showToast('Wallet refreshed', 'success');
    }
  }

  async loadBalance() {
    if (!this.state.wallet) return;
    
    try {
      const addr = this.state.wallet.address;
      const response = await fetch(`${this.API_BASE}${this.ENDPOINTS.balance}/${encodeURIComponent(addr)}`);
      if (!response.ok) throw new Error('Failed to load balance');
      const data = await response.json();
      const bal = Number(data.balance || 0);
      this.state.balance = bal;
      
      const balanceElement = document.getElementById('balance');
      if (balanceElement) {
        balanceElement.textContent = bal.toFixed(2);
      }
      
      console.log('💰 Balance loaded:', bal);
      
    } catch (error) {
      console.error('❌ Failed to load balance:', error);
      const balanceElement = document.getElementById('balance');
      if (balanceElement) {
        balanceElement.textContent = 'Error';
      }
    }
  }

  async requestTokens() {
    if (!this.state.wallet) {
      this.showToast('Please create a wallet first', 'warning');
      return;
    }

    this.showLoading('Requesting test tokens...');
    
    try {
      // Use JSON mint endpoint: POST /api/token/mint with {to, amount}
      const response = await fetch(`${this.API_BASE}${this.ENDPOINTS.mint}`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          amount: 100,
          to: this.state.wallet.address
        })
      });

      if (!response.ok) {
        const errorData = await response.json().catch(() => ({}));
        throw new Error(errorData.message || 'Token request failed');
      }

      // Update balance after successful mint
      await this.loadBalance();
      this.showToast('Test tokens received!', 'success');
      
    } catch (error) {
      console.error('❌ Token request failed:', error);
      this.showToast(`Failed to get tokens: ${error.message}`, 'error');
    } finally {
      this.hideLoading();
    }
  }

  showSendForm() {
    const sendCard = document.querySelector('.send-card');
    if (sendCard) {
      sendCard.scrollIntoView({ behavior: 'smooth' });
      document.getElementById('recipient')?.focus();
    }
  }

  async handleSendTokens(event) {
    event.preventDefault();
    
    if (!this.state.wallet) {
      this.showToast('Please create a wallet first', 'warning');
      return;
    }

    const recipient = document.getElementById('recipient').value;
    const amount = parseFloat(document.getElementById('amount').value);

    if (!recipient || !amount || amount <= 0) {
      this.showToast('Please enter valid recipient and amount', 'warning');
      return;
    }

    if (amount > this.state.balance) {
      this.showToast('Insufficient balance', 'error');
      return;
    }

    this.showLoading('Sending tokens...');
    
    try {
      const response = await fetch(`${this.API_BASE}${this.ENDPOINTS.transfer}`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          from: this.state.wallet.address,
          to: recipient,
          amount: amount,
          private_key: this.state.wallet.privateKey
        })
      });

      if (!response.ok) {
        const errorData = await response.json().catch(() => ({}));
        throw new Error(errorData.message || 'Transfer failed');
      }

      // Clear form and update balance
      document.getElementById('recipient').value = '';
      document.getElementById('amount').value = '';
      
      await this.loadBalance();
      await this.loadTransactions();
      
      this.showToast(`Successfully sent ${amount} OWA tokens!`, 'success');
      
    } catch (error) {
      console.error('❌ Transfer failed:', error);
      this.showToast(`Transfer failed: ${error.message}`, 'error');
    } finally {
      this.hideLoading();
    }
  }

  showReceiveModal() {
    if (!this.state.wallet) {
      this.showToast('Please create a wallet first', 'warning');
      return;
    }

    const modal = document.getElementById('qr-modal');
    const addressInput = document.getElementById('address-to-copy');
    
    if (modal && addressInput) {
      addressInput.value = this.state.wallet.address;
      modal.style.display = 'flex';
    }
  }

  hideModal(modalId) {
    const modal = document.getElementById(modalId);
    if (modal) {
      modal.style.display = 'none';
    }
  }

  copyAddress() {
    const addressInput = document.getElementById('address-to-copy');
    if (addressInput) {
      addressInput.select();
      document.execCommand('copy');
      this.showToast('Address copied to clipboard!', 'success');
    }
  }

  async loadBlockchainInfo() {
    try {
      const response = await fetch(`${this.API_BASE}${this.ENDPOINTS.blockchainInfo}`);
      if (!response.ok) throw new Error('Failed to load blockchain info');
      
      const data = await response.json();
      
      // Update stats
      document.getElementById('total-blocks').textContent = data.block_count || 0;
      document.getElementById('total-transactions').textContent = data.transaction_count || 0;
      document.getElementById('network-hashrate').textContent = data.network_hashrate || '0 H/s';
      
      document.getElementById('block-height').textContent = data.block_count || 0;
      document.getElementById('difficulty').textContent = data.difficulty || 1;
      document.getElementById('network-hash').textContent = data.latest_block_hash || '0x...';
      document.getElementById('last-block-time').textContent = data.last_block_time || '-';
      
      console.log('⛓️ Blockchain info loaded:', data);
      
    } catch (error) {
      console.error('❌ Failed to load blockchain info:', error);
    }
  }

  async loadBlocks() {
    try {
      const response = await fetch(`${this.API_BASE}${this.ENDPOINTS.blocks}`);
      if (!response.ok) throw new Error('Failed to load blocks');
      
      const blocks = await response.json();
      this.state.blocks = blocks;
      
      const blocksList = document.getElementById('blocks-list');
      if (blocksList && blocks.length > 0) {
        blocksList.innerHTML = blocks.slice(0, 10).map(block => `
          <div class="block-item">
            <div class="block-info">
              <div class="block-hash">${block.hash || 'N/A'}</div>
              <div class="block-meta">
                Block #${block.height || 0} • ${block.transaction_count || 0} transactions • ${new Date((block.timestamp || 0) * 1000).toLocaleString()}
              </div>
            </div>
            <div class="block-actions">
              <button class="btn btn-sm btn-outline" onclick="app.viewBlock('${block.hash}')">
                <i class="fas fa-eye"></i>
              </button>
            </div>
          </div>
        `).join('');
      } else if (blocksList) {
        blocksList.innerHTML = `
          <div class="empty-state">
            <i class="fas fa-cube"></i>
            <p>No blocks found</p>
            <small>Mine the first block to get started</small>
          </div>
        `;
      }
      
      console.log('🧱 Blocks loaded:', blocks.length);
      
    } catch (error) {
      console.error('❌ Failed to load blocks:', error);
      const blocksList = document.getElementById('blocks-list');
      if (blocksList) {
        blocksList.innerHTML = `
          <div class="loading-state">
            <i class="fas fa-exclamation-triangle"></i>
            <p>Failed to load blocks</p>
          </div>
        `;
      }
    }
  }

  async mineBlock() {
    this.showLoading('Mining new block...');
    
    try {
      const response = await fetch(`${this.API_BASE}${this.ENDPOINTS.mineBlock}`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          miner: this.state.wallet?.address || 'anonymous'
        })
      });

      if (!response.ok) {
        const errorData = await response.json().catch(() => ({}));
        throw new Error(errorData.message || 'Mining failed');
      }

      const result = await response.json();
      
      // Refresh blockchain data
      await Promise.all([
        this.loadBlockchainInfo(),
        this.loadBlocks()
      ]);
      
      this.showToast('Block mined successfully!', 'success');
      console.log('⛏️ Block mined:', result);
      
    } catch (error) {
      console.error('❌ Mining failed:', error);
      this.showToast(`Mining failed: ${error.message}`, 'error');
    } finally {
      this.hideLoading();
    }
  }

  async loadDApps() {
    try {
      const response = await fetch(`${this.API_BASE}${this.ENDPOINTS.dapps}`);
      if (!response.ok) throw new Error('Failed to load DApps');
      
      const dapps = await response.json();
      this.state.dapps = dapps;
      
      const dappsList = document.getElementById('dapps-list');
      if (dappsList && dapps.length > 0) {
        dappsList.innerHTML = dapps.map(dapp => `
          <div class="dapp-item">
            <div class="dapp-header">
              <div class="dapp-name">${dapp.name}</div>
              <div class="dapp-category">${dapp.category || 'General'}</div>
            </div>
            <div class="dapp-description">${dapp.description || 'No description available'}</div>
            <div class="dapp-actions">
              <button class="btn btn-sm btn-primary" onclick="app.interactWithDApp('${dapp.id}')">
                <i class="fas fa-play"></i>
                Launch
              </button>
              <button class="btn btn-sm btn-outline" onclick="app.viewDApp('${dapp.id}')">
                <i class="fas fa-eye"></i>
                View
              </button>
            </div>
          </div>
        `).join('');
      } else if (dappsList) {
        dappsList.innerHTML = `
          <div class="empty-state">
            <i class="fas fa-code"></i>
            <p>No DApps deployed yet</p>
            <small>Create your first DApp to get started</small>
          </div>
        `;
      }
      
      console.log('🚀 DApps loaded:', dapps.length);
      
    } catch (error) {
      console.error('❌ Failed to load DApps:', error);
    }
  }

  showDAppForm() {
    const createCard = document.querySelector('.create-dapp-card');
    if (createCard) {
      createCard.scrollIntoView({ behavior: 'smooth' });
      document.getElementById('dapp-name')?.focus();
    }
  }

  async handleCreateDApp(event) {
    event.preventDefault();
    
    if (!this.state.wallet) {
      this.showToast('Please create a wallet first', 'warning');
      return;
    }

    const name = document.getElementById('dapp-name').value;
    const description = document.getElementById('dapp-description').value;
    const category = document.getElementById('dapp-category').value;

    if (!name || !category) {
      this.showToast('Please fill in all required fields', 'warning');
      return;
    }

    this.showLoading('Creating DApp...');
    
    try {
      const response = await fetch(`${this.API_BASE}${this.ENDPOINTS.dapps}`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          name: name,
          description: description,
          category: category,
          owner: this.state.wallet.address
        })
      });

      if (!response.ok) {
        const errorData = await response.json().catch(() => ({}));
        throw new Error(errorData.message || 'DApp creation failed');
      }

      const result = await response.json();
      
      // Clear form
      document.getElementById('dapp-name').value = '';
      document.getElementById('dapp-description').value = '';
      document.getElementById('dapp-category').value = '';
      
      // Refresh DApps list
      await this.loadDApps();
      
      this.showToast('DApp created successfully!', 'success');
      console.log('🚀 DApp created:', result);
      
    } catch (error) {
      console.error('❌ DApp creation failed:', error);
      this.showToast(`DApp creation failed: ${error.message}`, 'error');
    } finally {
      this.hideLoading();
    }
  }

  async loadTransactions() {
    try {
      const response = await fetch(`${this.API_BASE}${this.ENDPOINTS.transactions}`);
      if (!response.ok) throw new Error('Failed to load transactions');
      
      const transactions = await response.json();
      this.state.transactions = transactions;
      
      const transactionsList = document.getElementById('transactions-list');
      if (transactionsList && transactions.length > 0) {
        transactionsList.innerHTML = transactions.slice(0, 20).map(tx => `
          <div class="transaction-item">
            <div class="transaction-info">
              <div class="transaction-hash">${tx.hash || 'N/A'}</div>
              <div class="transaction-details">
                From: ${tx.from || 'N/A'} → To: ${tx.to || 'N/A'} • ${new Date((tx.timestamp || 0) * 1000).toLocaleString()}
              </div>
            </div>
            <div class="transaction-amount">
              ${tx.amount || 0} OWA
            </div>
          </div>
        `).join('');
      } else if (transactionsList) {
        transactionsList.innerHTML = `
          <div class="empty-state">
            <i class="fas fa-receipt"></i>
            <p>No transactions yet</p>
            <small>Your transaction history will appear here</small>
          </div>
        `;
      }
      
      console.log('📋 Transactions loaded:', transactions.length);
      
    } catch (error) {
      console.error('❌ Failed to load transactions:', error);
    }
  }

  // Utility methods
  scrollToSection(sectionId) {
    const section = document.getElementById(sectionId);
    if (section) {
      section.scrollIntoView({ behavior: 'smooth' });
    }
  }

  showLoading(message = 'Loading...') {
    const overlay = document.getElementById('loading-overlay');
    const text = document.getElementById('loading-text');
    
    if (overlay && text) {
      text.textContent = message;
      overlay.style.display = 'flex';
    }
  }

  hideLoading() {
    const overlay = document.getElementById('loading-overlay');
    if (overlay) {
      overlay.style.display = 'none';
    }
  }

  showToast(message, type = 'info') {
    const container = document.getElementById('toast-container');
    if (!container) return;

    const toast = document.createElement('div');
    toast.className = `toast ${type}`;
    
    const iconMap = {
      success: 'fas fa-check-circle',
      error: 'fas fa-exclamation-circle',
      warning: 'fas fa-exclamation-triangle',
      info: 'fas fa-info-circle'
    };

    toast.innerHTML = `
      <div class="toast-content">
        <i class="${iconMap[type] || iconMap.info} toast-icon"></i>
        <span class="toast-message">${message}</span>
      </div>
    `;

    container.appendChild(toast);

    // Auto remove after 5 seconds
    setTimeout(() => {
      if (toast.parentNode) {
        toast.parentNode.removeChild(toast);
      }
    }, 5000);

    // Remove on click
    toast.addEventListener('click', () => {
      if (toast.parentNode) {
        toast.parentNode.removeChild(toast);
      }
    });
  }

  // Placeholder methods for future implementation
  viewBlock(hash) {
    this.showToast(`Viewing block: ${hash}`, 'info');
  }

  interactWithDApp(id) {
    this.showToast(`Launching DApp: ${id}`, 'info');
  }

  viewDApp(id) {
    this.showToast(`Viewing DApp: ${id}`, 'info');
  }
}

// Initialize the application when DOM is loaded
let app;
document.addEventListener('DOMContentLoaded', () => {
  app = new OwamiApp();
});

// Global error handler
window.addEventListener('error', (event) => {
  console.error('Global error:', event.error);
  if (app) {
    app.showToast('An unexpected error occurred', 'error');
  }
});

// Handle unhandled promise rejections
window.addEventListener('unhandledrejection', (event) => {
  console.error('Unhandled promise rejection:', event.reason);
  if (app) {
    app.showToast('An unexpected error occurred', 'error');
  }
});

// Export for global access
window.OwamiApp = OwamiApp;