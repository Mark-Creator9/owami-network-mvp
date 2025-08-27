// API Configuration
const API_BASE = 'http://127.0.0.1:8080/api';
const ENDPOINTS = {
    createWallet: '/wallets/create',
    getBalance: '/wallets/{address}/balance',
    sendTokens: '/transactions',
    getTransactions: '/wallets/{address}/transactions',
    faucet: '/faucet',
    vesting: {
        create: '/vesting/create',
        claimable: '/vesting/claimable',
        claim: '/vesting/claim'
    }
};

// DOM Elements
const elements = {
    createWalletBtn: document.getElementById('create-wallet'),
    walletInfo: document.getElementById('wallet-info'),
    balance: document.getElementById('balance'),
    getTokensBtn: document.getElementById('get-tokens'),
    recipient: document.getElementById('recipient'),
    amount: document.getElementById('amount'),
    sendTokensBtn: document.getElementById('send-tokens'),
    txStatus: document.getElementById('tx-status'),
    refreshTxBtn: document.getElementById('refresh-tx'),
    transactions: document.getElementById('transactions'),
    vestingStatus: document.getElementById('vestingStatus'),
    darkModeToggle: document.getElementById('dark-mode-toggle')
};

// Event Listeners
elements.createWalletBtn?.addEventListener('click', createWallet);
elements.getTokensBtn?.addEventListener('click', requestTokens);
elements.sendTokensBtn?.addEventListener('click', sendTokens);
elements.refreshTxBtn?.addEventListener('click', fetchTransactions);
elements.darkModeToggle?.addEventListener('click', toggleDarkMode);

async function createWallet() {
    try {
        const response = await fetch(`${API_BASE}${ENDPOINTS.createWallet}`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            }
        });
        if (!response.ok) throw new Error('Wallet creation failed');
        
        const wallet = await response.json();
        elements.walletInfo.innerHTML = `
            <p>Address: <span class="wallet-address">${wallet.address}</span></p>
            <p>Private Key: <span class="wallet-key">${wallet.private_key}</span></p>
        `;
        
        localStorage.setItem('wallet', JSON.stringify(wallet));
        fetchBalance();
    } catch (error) {
        console.error('Wallet creation failed:', error);
        elements.txStatus.textContent = error.message;
    }
}

async function fetchBalance() {
    const wallet = getWallet();
    if (!wallet) return;

    try {
        const url = `${API_BASE}${ENDPOINTS.getBalance}`.replace('{address}', wallet.address);
        const response = await fetch(url);
        if (!response.ok) throw new Error('Balance check failed');
        
        const data = await response.json();
        elements.balance.textContent = data.balance;
        await fetchTransactions();
    } catch (error) {
        console.error('Balance check failed:', error);
        elements.balance.textContent = 'Error';
    }
}

async function requestTokens() {
    const wallet = getWallet();
    if (!wallet) return;

    try {
        const response = await fetch(`${API_BASE}${ENDPOINTS.faucet}`, {
            method: 'POST',
            headers: {'Content-Type': 'application/json'},
            body: JSON.stringify({ address: wallet.address })
        });
        
        if (!response.ok) throw new Error('Failed to request tokens');
        
        elements.txStatus.textContent = 'Test tokens received';
        await fetchBalance();
    } catch (error) {
        console.error('Token request failed:', error);
        elements.txStatus.textContent = error.message;
    }
}

async function sendTokens() {
    const wallet = getWallet();
    if (!wallet) return;

    try {
        const tx = {
            from: wallet.address,
            to: elements.recipient.value,
            amount: parseFloat(elements.amount.value)
        };

        const response = await fetch(`${API_BASE}${ENDPOINTS.sendTokens}`, {
            method: 'POST',
            headers: {'Content-Type': 'application/json'},
            body: JSON.stringify(tx)
        });

        if (!response.ok) throw new Error('Transaction failed');

        elements.amount.value = '';
        elements.recipient.value = '';
        elements.txStatus.textContent = 'Transaction sent';
        await fetchBalance();
    } catch (error) {
        console.error('Transaction failed:', error);
        elements.txStatus.textContent = error.message;
    }
}

async function fetchTransactions() {
    const wallet = getWallet();
    if (!wallet) return;

    try {
        const url = `${API_BASE}${ENDPOINTS.getTransactions}`.replace('{address}', wallet.address);
        const response = await fetch(url);
        if (!response.ok) throw new Error('Failed to fetch transactions');
        
        const transactions = await response.json();
        elements.transactions.innerHTML = transactions
            .map(tx => `
                <div class="transaction">
                    <span>From: ${tx.from}</span>
                    <span>To: ${tx.to}</span>
                    <span>Amount: ${tx.amount}</span>
                </div>
            `)
            .join('');
    } catch (error) {
        console.error('Failed to fetch transactions:', error);
        elements.transactions.innerHTML = 'Failed to load transactions';
    }
}

function getWallet() {
    const walletJson = localStorage.getItem('wallet');
    if (!walletJson) {
        elements.txStatus.textContent = 'Please create a wallet first';
        return null;
    }
    return JSON.parse(walletJson);
}

// Dark Mode Toggle Functionality
function toggleDarkMode() {
    document.body.classList.toggle('dark-mode');
    localStorage.setItem('darkMode', document.body.classList.contains('dark-mode'));
    
    // Update button state for visual feedback
    const isDarkMode = document.body.classList.contains('dark-mode');
    elements.darkModeToggle.setAttribute('aria-pressed', isDarkMode);
    elements.darkModeToggle.innerHTML = isDarkMode ?
        `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
           <path d="M21 12.79A9 9 0 1 1 11.21 3 9 9 0 0 0 21 12.79z"></path>
           <line x1="12" y1="1" x2="12" y2="3"></line>
           <line x1="12" y1="21" x2="12" y2="23"></line>
           <line x1="1" y1="12" x2="3" y2="12"></line>
           <line x1="21" y1="12" x2="23" y2="12"></line>
           <line x1="18.36" y1="18.36" x2="19.78" y2="19.78"></line>
           <line x1="4.22" y1="4.22" x2="5.64" y2="5.64"></line>
           <line x1="18.36" y1="5.64" x2="19.78" y2="4.22"></line>
           <line x1="4.22" y1="19.78" x2="5.64" y2="18.36"></line>
         </svg>` :
        `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
           <circle cx="12" cy="12" r="5"></circle>
           <line x1="12" y1="1" x2="12" y2="3"></line>
           <line x1="12" y1="21" x2="12" y2="23"></line>
           <line x1="4.22" y1="4.22" x2="5.64" y2="5.64"></line>
           <line x1="18.36" y1="18.36" x2="19.78" y2="19.78"></line>
           <line x1="1" y1="12" x2="3" y2="12"></line>
           <line x1="21" y1="12" x2="23" y2="12"></line>
           <line x1="4.22" y1="19.78" x2="5.64" y2="18.36"></line>
           <line x1="18.36" y1="5.64" x2="19.78" y2="4.22"></line>
         </svg>`;
}

// Initialize
document.addEventListener('DOMContentLoaded', () => {
    // Check for saved dark mode preference
    const savedDarkMode = localStorage.getItem('darkMode') === 'true';
    if (savedDarkMode) {
        document.body.classList.add('dark-mode');
        toggleDarkMode(); // Update button state
    }

    const wallet = getWallet();
    if (wallet) {
        elements.walletInfo.innerHTML = `
            <p>Address: ${wallet.address}</p>
        `;
        fetchBalance();
    }

    // Check backend connection
    fetch(`${API_BASE}/health`)
        .catch(error => {
            console.error('Backend connection failed:', error);
            alert('Cannot connect to backend server. Please ensure it is running.');
        });
});
