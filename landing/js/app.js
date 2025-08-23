// Main application JavaScript for OWami Network Dashboard
const API_BASE_URL = window.location.origin;
const token = new OWamiToken(API_BASE_URL);

// African Theme Integration
class AfricanThemeManager {
    constructor() {
        this.particles = [];
        this.isInitialized = false;
    }

    init() {
        if (this.isInitialized) return;
        this.createParticles();
        this.setupScrollReveal();
        this.setupHoverEffects();
        this.isInitialized = true;
    }

    createParticles() {
        const container = document.createElement('div');
        container.className = 'african-particles';
        document.body.appendChild(container);

        // Create floating particles
        for (let i = 0; i < 20; i++) {
            setTimeout(() => {
                this.createParticle(container);
            }, i * 500);
        }

        // Continuous particle generation
        setInterval(() => {
            this.createParticle(container);
        }, 2000);
    }

    createParticle(container) {
        const particle = document.createElement('div');
        particle.className = 'african-particle';
        
        const colors = ['#FFD700', '#FF6B35', '#DC143C', '#228B22', '#8A2BE2'];
        const color = colors[Math.floor(Math.random() * colors.length)];
        const size = Math.random() * 10 + 5;
        
        particle.style.backgroundColor = color;
        particle.style.width = size + 'px';
        particle.style.height = size + 'px';
        particle.style.left = Math.random() * 100 + '%';
        particle.style.animationDuration = (Math.random() * 10 + 10) + 's';
        
        container.appendChild(particle);
        
        // Remove particle after animation
        setTimeout(() => {
            if (particle.parentNode) {
                particle.parentNode.removeChild(particle);
            }
        }, 20000);
    }

    setupScrollReveal() {
        const observer = new IntersectionObserver((entries) => {
            entries.forEach(entry => {
                if (entry.isIntersecting) {
                    entry.target.classList.add('revealed');
                }
            });
        }, {
            threshold: 0.1,
            rootMargin: '0px 0px -50px 0px'
        });

        // Observe elements with scroll reveal class
        document.querySelectorAll('.african-scroll-reveal').forEach(el => {
            observer.observe(el);
        });
    }

    setupHoverEffects() {
        // Add hover effects to cards and buttons
        document.querySelectorAll('.card, .btn, .feature-card').forEach(element => {
            element.classList.add('african-hover-glow');
        });
    }

    showNotification(message, type = 'info') {
        const notification = document.createElement('div');
        notification.className = `african-notification ${type}`;
        notification.textContent = message;
        
        document.body.appendChild(notification);
        
        setTimeout(() => {
            notification.classList.add('show');
        }, 100);
        
        setTimeout(() => {
            notification.classList.remove('show');
            setTimeout(() => {
                if (notification.parentNode) {
                    notification.parentNode.removeChild(notification);
                }
            }, 300);
        }, 3000);
    }

    showLoadingOverlay(message = 'Processing...') {
        let overlay = document.querySelector('.african-loading-overlay');
        if (!overlay) {
            overlay = document.createElement('div');
            overlay.className = 'african-loading-overlay';
            overlay.innerHTML = `
                <div style="text-align: center;">
                    <div class="african-loading-spinner"></div>
                    <div class="african-loading-text">${message}</div>
                </div>
            `;
            document.body.appendChild(overlay);
        }
        
        overlay.querySelector('.african-loading-text').textContent = message;
        overlay.classList.add('active');
    }

    hideLoadingOverlay() {
        const overlay = document.querySelector('.african-loading-overlay');
        if (overlay) {
            overlay.classList.remove('active');
        }
    }
}

// Initialize African Theme
const africanTheme = new AfricanThemeManager();

// Enhanced Utility functions
function showResult(elementId, content, isError = false) {
    const element = document.getElementById(elementId);
    if (!element) return;
    
    element.textContent = typeof content === 'string' ? content : JSON.stringify(content, null, 2);
    element.className = 'result ' + (isError ? 'error' : 'success');
    
    // Add African theme notification
    if (isError) {
        africanTheme.showNotification('Operation failed: ' + (typeof content === 'string' ? content : 'Unknown error'), 'error');
    } else {
        africanTheme.showNotification('Operation completed successfully!', 'success');
    }
}

function showLoading(elementId) {
    const element = document.getElementById(elementId);
    if (!element) return;
    element.textContent = 'Processing...';
    element.className = 'result loading';
}

function formatNumber(num) {
    return new Intl.NumberFormat().format(num);
}

function formatBalance(balance) {
    return (balance / 1000000000000000000).toFixed(2); // Convert from wei to tokens
}

function truncateAddress(address) {
    if (!address || address.length < 10) return address;
    return `${address.slice(0, 6)}...${address.slice(-4)}`;
}

// Dashboard functions
async function loadNetworkStats() {
    try {
        // Load blockchain info
        const blockchainInfo = await fetch(`${API_BASE_URL}/api/blockchain/info`).then(r => r.json());
        document.getElementById('network-height').textContent = blockchainInfo.height;
        
        // Load token info
        const tokenInfo = await token.getTokenInfo();
        document.getElementById('total-supply').textContent = formatNumber(tokenInfo.total_supply);
        
        // Load transactions for count
        const transactions = await token.getTransactions();
        document.getElementById('tx-count').textContent = transactions.length;
        
        // Load DApps count
        const dapps = await fetch(`${API_BASE_URL}/api/dapps`).then(r => r.json());
        const dappCount = dapps.data ? dapps.data.length : 0;
        document.getElementById('dapp-count').textContent = dappCount;
        
    } catch (error) {
        console.error('Error loading network stats:', error);
    }
}

async function refreshNetworkStatus() {
    try {
        const health = await fetch(`${API_BASE_URL}/api/health`).then(r => r.json());
        document.getElementById('network-name').textContent = health.network;
        document.getElementById('network-status-value').textContent = health.status;
        document.getElementById('token-address').textContent = health.token;
    } catch (error) {
        console.error('Error loading network status:', error);
        document.getElementById('network-status-value').textContent = 'Error';
    }
}

async function loadRecentBlocks() {
    try {
        const response = await fetch(`${API_BASE_URL}/api/blockchain/blocks`);
        const blocks = await response.json();
        
        const container = document.getElementById('recent-blocks');
        if (!blocks || blocks.length === 0) {
            container.innerHTML = '<p>No blocks found</p>';
            return;
        }
        
        let html = '<table style="width: 100%; border-collapse: collapse;">';
        html += '<thead><tr><th>Height</th><th>Hash</th><th>Transactions</th><th>Timestamp</th></tr></thead>';
        html += '<tbody>';
        
        blocks.slice(0, 5).forEach(block => {
            const date = new Date(block.timestamp * 1000).toLocaleString();
            html += `
                <tr>
                    <td>${block.height}</td>
                    <td><code>${truncateAddress(block.hash)}</code></td>
                    <td>${block.transaction_count}</td>
                    <td>${date}</td>
                </tr>
            `;
        });
        
        html += '</tbody></table>';
        container.innerHTML = html;
        
    } catch (error) {
        console.error('Error loading blocks:', error);
        document.getElementById('recent-blocks').innerHTML = '<p style="color: red;">Error loading blocks</p>';
    }
}

async function loadRecentTransactions() {
    try {
        const transactions = await token.getTransactions();
        
        const container = document.getElementById('recent-transactions');
        if (!transactions || transactions.length === 0) {
            container.innerHTML = '<p>No transactions found</p>';
            return;
        }
        
        let html = '<table style="width: 100%; border-collapse: collapse;">';
        html += '<thead><tr><th>Hash</th><th>From</th><th>To</th><th>Amount</th><th>Time</th></tr></thead>';
        html += '<tbody>';
        
        transactions.slice(0, 10).forEach(tx => {
            const date = new Date(tx.timestamp * 1000).toLocaleString();
            html += `
                <tr>
                    <td><code>${truncateAddress(tx.hash)}</code></td>
                    <td><code>${truncateAddress(tx.from)}</code></td>
                    <td><code>${truncateAddress(tx.to)}</code></td>
                    <td>${formatBalance(tx.amount)} OWA</td>
                    <td>${date}</td>
                </tr>
            `;
        });
        
        html += '</tbody></table>';
        container.innerHTML = html;
        
    } catch (error) {
        console.error('Error loading transactions:', error);
        document.getElementById('recent-transactions').innerHTML = '<p style="color: red;">Error loading transactions</p>';
    }
}

// Token operations (for backward compatibility with token-dapp.html)
async function loadTokenInfo() {
    try {
        showLoading('token-info');
        const info = await token.getTokenInfo();
        showResult('token-info', info);
    } catch (error) {
        showResult('token-info', `Error: ${error.message}`, true);
    }
}

async function checkBalance() {
    const address = document.getElementById('balance-address').value;
    if (!address) {
        showResult('balance-result', 'Please enter an address', true);
        return;
    }

    try {
        showLoading('balance-result');
        const balance = await token.getBalance(address);
        showResult('balance-result', balance);
    } catch (error) {
        showResult('balance-result', `Error: ${error.message}`, true);
    }
}

async function transferTokens() {
    const from = document.getElementById('transfer-from').value;
    const to = document.getElementById('transfer-to').value;
    const amount = document.getElementById('transfer-amount').value;

    if (!from || !to || !amount) {
        showResult('transfer-result', 'Please fill in all fields', true);
        return;
    }

    try {
        showLoading('transfer-result');
        const result = await token.transfer(from, to, amount);
        showResult('transfer-result', result);
        // Refresh dashboard stats
        loadNetworkStats();
    } catch (error) {
        showResult('transfer-result', `Error: ${error.message}`, true);
    }
}

async function mintTokens() {
    const to = document.getElementById('mint-to').value;
    const amount = document.getElementById('mint-amount').value;

    if (!to || !amount) {
        showResult('mint-result', 'Please fill in all fields', true);
        return;
    }

    try {
        showLoading('mint-result');
        const result = await token.mint(to, amount);
        showResult('mint-result', result);
        // Refresh dashboard stats
        loadNetworkStats();
    } catch (error) {
        showResult('mint-result', `Error: ${error.message}`, true);
    }
}

async function loadTransactions() {
    try {
        showLoading('transactions');
        const transactions = await token.getTransactions();
        showResult('transactions', transactions);
    } catch (error) {
        showResult('transactions', `Error: ${error.message}`, true);
    }
}

// DApp operations (updated for correct endpoints)
async function deployDApp() {
    const name = document.getElementById('dapp-name').value;
    const contractAddress = document.getElementById('dapp-address').value;

    if (!name || !contractAddress) {
        showResult('deploy-result', 'Please fill in all fields', true);
        return;
    }

    try {
        showLoading('deploy-result');
        const response = await fetch(`${API_BASE_URL}/api/dapps`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ 
                name, 
                description: "Deployed via frontend", 
                contract_address: contractAddress 
            }),
        });

        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }

        const result = await response.json();
        showResult('deploy-result', result);
        // Refresh dashboard stats
        loadNetworkStats();
    } catch (error) {
        showResult('deploy-result', `Error: ${error.message}`, true);
    }
}

async function loadUserDApps() {
    try {
        showLoading('user-dapps');
        const response = await fetch(`${API_BASE_URL}/api/dapps`);
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        const dapps = await response.json();
        showResult('user-dapps', dapps);
    } catch (error) {
        showResult('user-dapps', `Error: ${error.message}`, true);
    }
}

// Enhanced dashboard functions with African theme
async function loadNetworkStats() {
    try {
        africanTheme.showLoadingOverlay('Loading network statistics...');
        
        // Load blockchain info
        const blockchainInfo = await fetch(`${API_BASE_URL}/api/blockchain/info`).then(r => r.json());
        document.getElementById('network-height').textContent = blockchainInfo.height;
        
        // Load token info
        const tokenInfo = await token.getTokenInfo();
        document.getElementById('total-supply').textContent = formatNumber(tokenInfo.total_supply);
        
        // Load transactions for count
        const transactions = await token.getTransactions();
        document.getElementById('tx-count').textContent = transactions.length;
        
        // Load DApps count
        const dapps = await fetch(`${API_BASE_URL}/api/dapps`).then(r => r.json());
        const dappCount = dapps.data ? dapps.data.length : 0;
        document.getElementById('dapp-count').textContent = dappCount;
        
        africanTheme.hideLoadingOverlay();
        africanTheme.showNotification('Network stats updated successfully!', 'success');
        
    } catch (error) {
        console.error('Error loading network stats:', error);
        africanTheme.hideLoadingOverlay();
        africanTheme.showNotification('Failed to load network stats', 'error');
    }
}

// Enhanced token operations
async function loadTokenInfo() {
    try {
        africanTheme.showLoadingOverlay('Loading token information...');
        const info = await token.getTokenInfo();
        showResult('token-info', info);
        africanTheme.hideLoadingOverlay();
    } catch (error) {
        africanTheme.hideLoadingOverlay();
        showResult('token-info', `Error: ${error.message}`, true);
    }
}

async function checkBalance() {
    const address = document.getElementById('balance-address').value;
    if (!address) {
        africanTheme.showNotification('Please enter an address', 'warning');
        showResult('balance-result', 'Please enter an address', true);
        return;
    }

    try {
        africanTheme.showLoadingOverlay('Checking balance...');
        const balance = await token.getBalance(address);
        showResult('balance-result', balance);
        africanTheme.hideLoadingOverlay();
    } catch (error) {
        africanTheme.hideLoadingOverlay();
        showResult('balance-result', `Error: ${error.message}`, true);
    }
}

async function transferTokens() {
    const from = document.getElementById('transfer-from').value;
    const to = document.getElementById('transfer-to').value;
    const amount = document.getElementById('transfer-amount').value;

    if (!from || !to || !amount) {
        africanTheme.showNotification('Please fill in all fields', 'warning');
        showResult('transfer-result', 'Please fill in all fields', true);
        return;
    }

    try {
        africanTheme.showLoadingOverlay('Processing transfer...');
        const result = await token.transfer(from, to, amount);
        showResult('transfer-result', result);
        // Refresh dashboard stats
        loadNetworkStats();
        africanTheme.hideLoadingOverlay();
    } catch (error) {
        africanTheme.hideLoadingOverlay();
        showResult('transfer-result', `Error: ${error.message}`, true);
    }
}

async function mintTokens() {
    const to = document.getElementById('mint-to').value;
    const amount = document.getElementById('mint-amount').value;

    if (!to || !amount) {
        africanTheme.showNotification('Please fill in all fields', 'warning');
        showResult('mint-result', 'Please fill in all fields', true);
        return;
    }

    try {
        africanTheme.showLoadingOverlay('Minting tokens...');
        const result = await token.mint(to, amount);
        showResult('mint-result', result);
        // Refresh dashboard stats
        loadNetworkStats();
        africanTheme.hideLoadingOverlay();
    } catch (error) {
        africanTheme.hideLoadingOverlay();
        showResult('mint-result', `Error: ${error.message}`, true);
    }
}

// Enhanced DApp operations
async function deployDApp() {
    const name = document.getElementById('dapp-name').value;
    const contractAddress = document.getElementById('dapp-address').value;

    if (!name || !contractAddress) {
        africanTheme.showNotification('Please fill in all fields', 'warning');
        showResult('deploy-result', 'Please fill in all fields', true);
        return;
    }

    try {
        africanTheme.showLoadingOverlay('Deploying DApp...');
        const response = await fetch(`${API_BASE_URL}/api/dapps`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                name,
                description: "Deployed via frontend",
                contract_address: contractAddress
            }),
        });

        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }

        const result = await response.json();
        showResult('deploy-result', result);
        // Refresh dashboard stats
        loadNetworkStats();
        africanTheme.hideLoadingOverlay();
    } catch (error) {
        africanTheme.hideLoadingOverlay();
        showResult('deploy-result', `Error: ${error.message}`, true);
    }
}

// Enhanced transaction loading with African theme
async function loadRecentTransactions() {
    try {
        const transactions = await token.getTransactions();
        
        const container = document.getElementById('recent-transactions');
        if (!transactions || transactions.length === 0) {
            container.innerHTML = '<div class="african-empty-state">No transactions found</div>';
            return;
        }
        
        let html = '<div class="african-transaction-list">';
        
        transactions.slice(0, 10).forEach(tx => {
            const date = new Date(tx.timestamp * 1000).toLocaleString();
            const amount = formatBalance(tx.amount);
            const isIncoming = tx.to && tx.to.toLowerCase().includes('0x');
            
            html += `
                <div class="african-transaction-item ${isIncoming ? 'incoming' : 'outgoing'}">
                    <div class="transaction-icon">
                        <i class="fas ${isIncoming ? 'fa-arrow-down' : 'fa-arrow-up'}"></i>
                    </div>
                    <div class="transaction-details">
                        <div class="transaction-hash">${truncateAddress(tx.hash)}</div>
                        <div class="transaction-addresses">
                            <span>From: ${truncateAddress(tx.from)}</span>
                            <span>To: ${truncateAddress(tx.to)}</span>
                        </div>
                        <div class="transaction-meta">
                            <span class="amount">${amount} OWAMI</span>
                            <span class="time">${date}</span>
                        </div>
                    </div>
                </div>
            `;
        });
        
        html += '</div>';
        container.innerHTML = html;
        
    } catch (error) {
        console.error('Error loading transactions:', error);
        document.getElementById('recent-transactions').innerHTML =
            '<div class="african-error-state">Error loading transactions</div>';
    }
}

// Enhanced block loading with African theme
async function loadRecentBlocks() {
    try {
        const response = await fetch(`${API_BASE_URL}/api/blockchain/blocks`);
        const blocks = await response.json();
        
        const container = document.getElementById('recent-blocks');
        if (!blocks || blocks.length === 0) {
            container.innerHTML = '<div class="african-empty-state">No blocks found</div>';
            return;
        }
        
        let html = '<div class="african-blocks-grid">';
        
        blocks.slice(0, 5).forEach(block => {
            const date = new Date(block.timestamp * 1000).toLocaleString();
            const blockColor = block.height % 2 === 0 ? 'gold' : 'orange';
            
            html += `
                <div class="african-block-card ${blockColor}">
                    <div class="block-header">
                        <span class="block-number">#${block.height}</span>
                        <span class="block-tx-count">${block.transaction_count} tx</span>
                    </div>
                    <div class="block-hash">${truncateAddress(block.hash)}</div>
                    <div class="block-time">${date}</div>
                </div>
            `;
        });
        
        html += '</div>';
        container.innerHTML = html;
        
    } catch (error) {
        console.error('Error loading blocks:', error);
        document.getElementById('recent-blocks').innerHTML =
            '<div class="african-error-state">Error loading blocks</div>';
    }
}

// African-themed utility functions
function formatBalance(balance) {
    return (balance / 1000000000000000000).toFixed(4); // Convert from wei to tokens with 4 decimals
}

function formatNumber(num) {
    return new Intl.NumberFormat('en-US', {
        notation: 'compact',
        maximumFractionDigits: 2
    }).format(num);
}

function truncateAddress(address, length = 8) {
    if (!address || address.length <= length * 2 + 2) return address;
    return address.substring(0, length) + '...' + address.substring(address.length - length);
}

function refreshNetworkStatus() {
    const statusElement = document.getElementById('network-status');
    if (statusElement) {
        statusElement.innerHTML = '<span class="status-indicator online">●</span> Online';
    }
}

// Add African-themed loading states
function showAfricanLoading(elementId, message = 'Loading...') {
    const element = document.getElementById(elementId);
    if (!element) return;
    
    element.innerHTML = `
        <div class="african-loading">
            <div class="african-spinner"></div>
            <div class="african-loading-text">${message}</div>
        </div>
    `;
}

// Add African-themed success/error states
function showAfricanResult(elementId, content, type = 'info') {
    const element = document.getElementById(elementId);
    if (!element) return;
    
    const icon = type === 'success' ? '✓' : type === 'error' ? '✗' : 'ℹ';
    const className = `african-result ${type}`;
    
    element.innerHTML = `
        <div class="${className}">
            <span class="result-icon">${icon}</span>
            <span class="result-content">${content}</span>
        </div>
    `;
}

// Add keyboard shortcuts
document.addEventListener('keydown', (e) => {
    // Ctrl/Cmd + R to refresh dashboard
    if ((e.ctrlKey || e.metaKey) && e.key === 'r') {
        e.preventDefault();
        if (document.getElementById('network-height')) {
            loadNetworkStats();
            refreshNetworkStatus();
            loadRecentBlocks();
            loadRecentTransactions();
            africanTheme.showNotification('Dashboard refreshed!', 'success');
        }
    }
    
    // Ctrl/Cmd + K to focus search (if search exists)
    if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
        e.preventDefault();
        const searchInput = document.querySelector('input[type="search"], input[placeholder*="search" i]');
        if (searchInput) {
            searchInput.focus();
            searchInput.select();
        }
    }
});

// Add African-themed Easter egg
let konamiCode = [];
const konamiSequence = ['ArrowUp', 'ArrowUp', 'ArrowDown', 'ArrowDown', 'ArrowLeft', 'ArrowRight', 'ArrowLeft', 'ArrowRight', 'KeyB', 'KeyA'];

document.addEventListener('keydown', (e) => {
    konamiCode.push(e.code);
    if (konamiCode.length > konamiSequence.length) {
        konamiCode.shift();
    }
    
    if (konamiCode.join(',') === konamiSequence.join(',')) {
        // Easter egg activated
        document.body.classList.add('african-easter-egg');
        africanTheme.showNotification('🎉 African Mode Activated! 🎉', 'success');
        
        // Create celebration particles
        for (let i = 0; i < 50; i++) {
            setTimeout(() => {
                const particle = document.createElement('div');
                particle.className = 'african-celebration-particle';
                particle.style.left = Math.random() * 100 + '%';
                particle.style.animationDelay = Math.random() * 2 + 's';
                document.body.appendChild(particle);
                
                setTimeout(() => {
                    if (particle.parentNode) {
                        particle.parentNode.removeChild(particle);
                    }
                }, 3000);
            }, i * 50);
        }
        
        // Reset after 10 seconds
        setTimeout(() => {
            document.body.classList.remove('african-easter-egg');
        }, 10000);
        
        konamiCode = [];
    }
});

// Initialize dashboard with African theme
document.addEventListener('DOMContentLoaded', () => {
    // Initialize African theme
    africanTheme.init();
    
    // Add African theme classes to existing elements
    document.querySelectorAll('.card, .feature-card, .stat-card').forEach(card => {
        card.classList.add('african-scroll-reveal', 'african-hover-glow');
    });
    
    // Add floating elements to hero section
    const heroSection = document.querySelector('.hero');
    if (heroSection) {
        const floatingElements = document.createElement('div');
        floatingElements.className = 'floating-elements';
        floatingElements.innerHTML = `
            <div class="floating-element"></div>
            <div class="floating-element"></div>
            <div class="floating-element"></div>
        `;
        heroSection.appendChild(floatingElements);
    }
    
    // Load dashboard data if we're on the dashboard page
    if (document.getElementById('network-height')) {
        loadNetworkStats();
        refreshNetworkStatus();
        loadRecentBlocks();
        loadRecentTransactions();
        
        // Auto-refresh every 30 seconds
        setInterval(() => {
            loadNetworkStats();
            loadRecentBlocks();
            loadRecentTransactions();
        }, 30000);
    }
    
    // Load initial data for other pages
    if (typeof loadTokenInfo === 'function' && document.getElementById('token-info')) {
        loadTokenInfo();
    }
    if (typeof loadTransactions === 'function' && document.getElementById('transactions')) {
        loadTransactions();
    }
    
    // Add African pattern background
    const body = document.body;
    if (body && !body.querySelector('.african-pattern')) {
        const pattern = document.createElement('div');
        pattern.className = 'african-pattern';
        body.insertBefore(pattern, body.firstChild);
    }
});

// Handle Enter key for inputs
document.addEventListener('keypress', (e) => {
    if (e.key === 'Enter' && e.target.tagName === 'INPUT') {
        const button = e.target.parentElement.querySelector('button');
        if (button && !button.disabled) button.click();
    }
});

// Add smooth scrolling for anchor links
document.addEventListener('click', (e) => {
    if (e.target.matches('a[href^="#"]')) {
        e.preventDefault();
        const target = document.querySelector(e.target.getAttribute('href'));
        if (target) {
            target.scrollIntoView({
                behavior: 'smooth',
                block: 'start'
            });
        }
    }
});