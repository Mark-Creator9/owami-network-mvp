document.addEventListener('DOMContentLoaded', () => {
    // --- DOM Elements ---
    const elements = {
        themeToggle: document.getElementById('theme-toggle'),
        createWalletBtn: document.getElementById('create-wallet'),
        exportWalletBtn: document.getElementById('export-wallet'),
        sendTokensBtn: document.getElementById('send-tokens'),
        requestTokensBtn: document.getElementById('request-tokens'),
        claimVestedBtn: document.getElementById('claim-vested'),
        refreshTxBtn: document.getElementById('refresh-tx'),
        generateKeyBtn: document.getElementById('generate-key'),
        walletInfo: document.getElementById('wallet-info'),
        apiKeyInfo: document.getElementById('api-key-info'),
        recipientInput: document.getElementById('recipient'),
        amountInput: document.getElementById('amount'),
        transactionsList: document.getElementById('transactions-list'),
        vestingInfo: document.getElementById('vesting-info'),
    };

    // --- API Configuration ---
    const API_BASE = '/api';
    const ENDPOINTS = {
        createWallet: `${API_BASE}/wallets/create`,
        getBalance: (address) => `/wallets/${address}/balance`,
        getTransactions: (address) => `/wallets/${address}/transactions`,
        sendTokens: `/transactions`,
        faucet: `/faucet`,
        vestingClaimable: (address) => `/vesting/claimable?address=${address}`,
        vestingClaim: `/vesting/claim`,
        generateKey: `${API_BASE}/keys/generate`,
    };

    // --- State ---
    let wallet = null;

    // --- Functions ---
    const apiRequest = async (url, options = {}) => {
        try {
            const response = await fetch(url, options);
            if (!response.ok) {
                const errorData = await response.json().catch(() => ({ message: 'An unknown error occurred' }));
                throw new Error(errorData.message || `HTTP error! status: ${response.status}`);
            }
            return response.json();
        } catch (error) {
            console.error('API Request Failed:', error);
            alert(`Error: ${error.message}`);
            throw error;
        }
    };

    const updateWalletInfo = () => {
        if (wallet) {
            elements.walletInfo.innerHTML = `
                <p><strong>Address:</strong> <span class="tx-address">${wallet.address}</span></p>
                <p><strong>Balance:</strong> ${wallet.balance || 0} OWA</p>
                <p class="private-key"><strong>Private Key:</strong> <span>${wallet.private_key}</span></p>
            `;
        } else {
            elements.walletInfo.innerHTML = '<p>Create or load a wallet to begin.</p>';
        }
        elements.requestTokensBtn.disabled = !wallet;
    };

    const createWallet = async () => {
        const newWallet = await apiRequest(ENDPOINTS.createWallet, { method: 'POST' });
        wallet = { ...newWallet, balance: 0 };
        localStorage.setItem('owamiWallet', JSON.stringify(wallet));
        updateWalletInfo();
        await fetchBalance();
        await fetchTransactions();
        await fetchVestingInfo();
    };

    const requestTokens = async () => {
        if (!wallet) return;
        await apiRequest(ENDPOINTS.faucet, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ address: wallet.address }),
        });
        await fetchBalance();
    };

    const fetchBalance = async () => {
        if (!wallet) return;
        const data = await apiRequest(ENDPOINTS.getBalance(wallet.address));
        wallet.balance = data.balance;
        updateWalletInfo();
    };

    const fetchTransactions = async () => {
        if (!wallet) return;
        const transactions = await apiRequest(ENDPOINTS.getTransactions(wallet.address));
        elements.transactionsList.innerHTML = transactions.length ? '' : '<p>No transactions yet.</p>';
        transactions.forEach(tx => {
            const item = document.createElement('div');
            item.className = 'transaction-item';
            item.innerHTML = `
                <div class="tx-icon"><i class="fas fa-exchange-alt"></i></div>
                <div class="tx-details">
                    <p class="tx-address"><strong>From:</strong> ${tx.from}</p>
                    <p class="tx-address"><strong>To:</strong> ${tx.to}</p>
                </div>
                <div class="tx-amount">${tx.amount} OWA</div>
            `;
            elements.transactionsList.appendChild(item);
        });
    };

    const sendTokens = async () => {
        if (!wallet) return;
        const to = elements.recipientInput.value;
        const amount = parseFloat(elements.amountInput.value);
        if (!to || !amount) return alert('Please enter a recipient and amount.');

        await apiRequest(ENDPOINTS.sendTokens, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ from: wallet.address, to, amount }),
        });

        elements.recipientInput.value = '';
        elements.amountInput.value = '';
        alert('Transaction sent successfully!');
        await fetchBalance();
        await fetchTransactions();
    };

    const fetchVestingInfo = async () => {
        if (!wallet) return;
        const data = await apiRequest(ENDPOINTS.vestingClaimable(wallet.address));
        if (data.claimable > 0) {
            elements.vestingInfo.innerHTML = `<p>You have ${data.claimable} OWA available to claim.</p>`;
            elements.claimVestedBtn.disabled = false;
        } else {
            elements.vestingInfo.innerHTML = '<p>No active vesting schedules.</p>';
            elements.claimVestedBtn.disabled = true;
        }
    };

    const claimVestedTokens = async () => {
        if (!wallet) return;
        await apiRequest(ENDPOINTS.vestingClaim, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ address: wallet.address }),
        });
        alert('Vested tokens claimed successfully!');
        await fetchBalance();
        await fetchVestingInfo();
    };

    const exportWallet = () => {
        if (!wallet) return alert('No wallet to export.');
        const data = `Address: ${wallet.address}\nPrivate Key: ${wallet.private_key}`;
        const blob = new Blob([data], { type: 'text/plain' });
        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = `owami-wallet.txt`;
        a.click();
        URL.revokeObjectURL(url);
    };

    const toggleTheme = () => {
        document.body.classList.toggle('dark-mode');
        localStorage.setItem('theme', document.body.classList.contains('dark-mode') ? 'dark' : 'light');
        elements.themeToggle.innerHTML = `<i class="fas ${document.body.classList.contains('dark-mode') ? 'fa-sun' : 'fa-moon'}"></i>`;
    };

    const generateApiKey = async () => {
        if (!wallet) {
            alert('Please create a wallet first to associate with your API key.');
            return;
        }
        const email = prompt('Please enter your email address to generate an API key:');
        if (!email) return;

        const apiKey = await apiRequest(ENDPOINTS.generateKey, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ email }),
        });

        elements.apiKeyInfo.innerHTML = `
            <p><strong>Your API Key:</strong></p>
            <p class="tx-address">${apiKey.key}</p>
            <p><small>Store this key securely. It will not be shown again.</small></p>
        `;
        elements.generateKeyBtn.disabled = true;
    };

    // --- Initialization ---
    const init = () => {
        // --- Universal Logic (Theme) ---
        if (elements.themeToggle) {
            if (localStorage.getItem('theme') === 'dark') {
                document.body.classList.add('dark-mode');
                elements.themeToggle.innerHTML = '<i class="fas fa-sun"></i>';
            }
            elements.themeToggle.addEventListener('click', toggleTheme);
        }

        // --- Wallet Page Specific Logic ---
        // We can detect if we are on the wallet page by checking for a key element.
        if (elements.createWalletBtn) {
            const savedWallet = localStorage.getItem('owamiWallet');
            if (savedWallet) {
                wallet = JSON.parse(savedWallet);
                updateWalletInfo();
                fetchBalance();
                fetchTransactions();
                fetchVestingInfo();
            }

            // Add event listeners for wallet page elements
            elements.createWalletBtn.addEventListener('click', createWallet);
            elements.exportWalletBtn.addEventListener('click', exportWallet);
            elements.sendTokensBtn.addEventListener('click', sendTokens);
            elements.requestTokensBtn.addEventListener('click', requestTokens);
            elements.claimVestedBtn.addEventListener('click', claimVestedTokens);
            elements.refreshTxBtn.addEventListener('click', fetchTransactions);
            elements.generateKeyBtn.addEventListener('click', generateApiKey);
        }
    };

    init();
});
