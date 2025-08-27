// Owami Token JavaScript SDK
class OwamiToken {
    constructor(baseUrl = 'http://127.0.0.1:8080') {
        this.baseUrl = baseUrl;
    }

    async getTokenInfo() {
        const response = await fetch(`${this.baseUrl}/api/token/info`);
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        return await response.json();
    }

    async getBalance(address) {
        const response = await fetch(`${this.baseUrl}/api/wallets/${address}/balance`);
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        return await response.json();
    }

    async transfer(from, to, amount) {
        const response = await fetch(`${this.baseUrl}/api/transactions`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ from, to, amount }),
        });
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        return await response.json();
    }

    async mint(to, amount) {
        const response = await fetch(`${this.baseUrl}/api/faucet`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ address: to, amount }),
        });
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        return await response.json();
    }

    async getTransactions(address) {
        const response = await fetch(`${this.baseUrl}/api/wallets/${address}/transactions`);
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        return await response.json();
    }

    // Vesting functionality
    async createVestingSchedule(beneficiary, amount, duration, cliff) {
        const response = await fetch(`${this.baseUrl}/api/vesting/create`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ beneficiary, amount, duration, cliff }),
        });
        if (!response.ok) {
            throw new Error(`Vesting creation failed! status: ${response.status}`);
        }
        return await response.json();
    }

    async getClaimableAmount(beneficiary) {
        const response = await fetch(`${this.baseUrl}/api/vesting/claimable`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ beneficiary }),
        });
        if (!response.ok) {
            throw new Error(`Failed to get claimable amount! status: ${response.status}`);
        }
        return await response.json();
    }

    async claimTokens(beneficiary) {
        const response = await fetch(`${this.baseUrl}/api/vesting/claim`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ beneficiary }),
        });
        if (!response.ok) {
            throw new Error(`Claim failed! status: ${response.status}`);
        }
        return await response.json();
    }

    /**
     * Deploy smart contract to Owami Network
     * @param {string} owner - Deployer's wallet address
     * @param {string} contractCode - Compiled contract bytecode
     * @param {number} gasLimit - Gas limit for deployment
     * @returns {Promise} Deployment transaction result
     */
    async deployContract(owner, contractCode, gasLimit) {
        const response = await fetch(`${this.baseUrl}/api/deploy`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ owner, contractCode, gasLimit }),
        });
        if (!response.ok) {
            throw new Error(`Deployment failed! status: ${response.status}`);
        }
        return await response.json();
    }
}

// Example usage
if (typeof window !== 'undefined') {
    window.OwamiToken = OwamiToken;
}

// Export for Node.js
if (typeof module !== 'undefined' && module.exports) {
    module.exports = OwamiToken;
}