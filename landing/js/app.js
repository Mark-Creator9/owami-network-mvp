// Main application JavaScript
const API_BASE_URL = 'http://localhost:3000';
const token = new OWamiToken(API_BASE_URL);

// Utility functions
function showResult(elementId, content, isError = false) {
    const element = document.getElementById(elementId);
    element.textContent = typeof content === 'string' ? content : JSON.stringify(content, null, 2);
    element.className = 'result ' + (isError ? 'error' : 'success');
}

function showLoading(elementId) {
    const element = document.getElementById(elementId);
    element.textContent = 'Loading...';
    element.className = 'result';
}

// Token operations
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

// DApp operations
async function deployDApp() {
    const name = document.getElementById('dapp-name').value;
    const address = document.getElementById('dapp-address').value;

    if (!name || !address) {
        showResult('deploy-result', 'Please fill in all fields', true);
        return;
    }

    try {
        showLoading('deploy-result');
        const response = await fetch(`${API_BASE_URL}/api/dapp`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ name, address }),
        });

        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }

        const result = await response.json();
        showResult('deploy-result', result);
    } catch (error) {
        showResult('deploy-result', `Error: ${error.message}`, true);
    }
}

async function loadUserDApps() {
    const address = document.getElementById('user-address').value;
    if (!address) {
        showResult('user-dapps', 'Please enter an address', true);
        return;
    }

    try {
        showLoading('user-dapps');
        const response = await fetch(`${API_BASE_URL}/api/dapp/user/${address}`);
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        const dapps = await response.json();
        showResult('user-dapps', dapps);
    } catch (error) {
        showResult('user-dapps', `Error: ${error.message}`, true);
    }
}

// Initialize
document.addEventListener('DOMContentLoaded', () => {
    // Load initial data
    loadTokenInfo();
    loadTransactions();
});

// Handle Enter key for inputs
document.querySelectorAll('input[type="text"]').forEach(input => {
    input.addEventListener('keypress', (e) => {
        if (e.key === 'Enter') {
            const button = e.target.parentElement.querySelector('button');
            if (button) button.click();
        }
    });
});
