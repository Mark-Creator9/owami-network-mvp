class OWamiToken {
    constructor(apiBaseUrl) {
        this.apiBaseUrl = apiBaseUrl;
    }

    async getTokenInfo() {
        const response = await fetch(`${this.apiBaseUrl}/api/token/info`);
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        return response.json();
    }

    async getBalance(address) {
        const response = await fetch(`${this.apiBaseUrl}/api/token/balance/${address}`);
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        return response.json();
    }

    async transfer(from, to, amount) {
        const response = await fetch(`${this.apiBaseUrl}/api/token/transfer`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                from,
                to,
                amount: parseInt(amount),
                private_key: "default" // In production, this should be handled securely
            }),
        });
        
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        return response.json();
    }

    async mint(to, amount) {
        const response = await fetch(`${this.apiBaseUrl}/api/token/mint`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                to,
                amount: parseInt(amount)
            }),
        });
        
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        return response.json();
    }

    async getTransactions() {
        const response = await fetch(`${this.apiBaseUrl}/api/token/transactions`);
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        return response.json();
    }

    // Additional utility methods for the frontend
    formatBalance(balance) {
        // Convert from wei to tokens (assuming 18 decimals)
        return (balance / Math.pow(10, 18)).toFixed(4);
    }

    formatAmount(amount) {
        return new Intl.NumberFormat().format(amount);
    }

    truncateAddress(address) {
        if (!address || address.length < 10) return address;
        return `${address.slice(0, 6)}...${address.slice(-4)}`;
    }

    // Blockchain API methods
    async getBlockchainInfo() {
        const response = await fetch(`${this.apiBaseUrl}/api/blockchain/info`);
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        return response.json();
    }

    async getBlocks() {
        const response = await fetch(`${this.apiBaseUrl}/api/blockchain/blocks`);
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        return response.json();
    }

    async mineBlock() {
        const response = await fetch(`${this.apiBaseUrl}/api/blockchain/mine`, {
            method: 'POST',
        });
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        return response.json();
    }

    // Health check
    async getHealth() {
        const response = await fetch(`${this.apiBaseUrl}/api/health`);
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        return response.json();
    }
}

// Create global instance for backward compatibility
if (typeof window !== 'undefined') {
    window.owaToken = new OWamiToken(window.location.origin);
}