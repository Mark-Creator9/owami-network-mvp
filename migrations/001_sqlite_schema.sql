-- SQLite compatible schema for Owami Network

-- Create token_balances table
CREATE TABLE IF NOT EXISTS token_balances (
    address TEXT PRIMARY KEY,
    balance TEXT NOT NULL DEFAULT '0'
);

-- Create token_transactions table
CREATE TABLE IF NOT EXISTS token_transactions (
    transaction_hash TEXT PRIMARY KEY,
    from_address TEXT NOT NULL,
    to_address TEXT NOT NULL,
    amount TEXT NOT NULL,
    block_number INTEGER NOT NULL,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Create token_approvals table
CREATE TABLE IF NOT EXISTS token_approvals (
    owner TEXT NOT NULL,
    spender TEXT NOT NULL,
    amount TEXT NOT NULL,
    PRIMARY KEY (owner, spender)
);

-- Create dapps table
CREATE TABLE IF NOT EXISTS dapps (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    contract_address TEXT NOT NULL,
    creator_id TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Create dapp_states table
CREATE TABLE IF NOT EXISTS dapp_states (
    dapp_id TEXT NOT NULL,
    key TEXT NOT NULL,
    value TEXT NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (dapp_id, key),
    FOREIGN KEY (dapp_id) REFERENCES dapps(id) ON DELETE CASCADE
);

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_token_transactions_from ON token_transactions(from_address);
CREATE INDEX IF NOT EXISTS idx_token_transactions_to ON token_transactions(to_address);
CREATE INDEX IF NOT EXISTS idx_token_transactions_timestamp ON token_transactions(timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_dapps_creator ON dapps(creator_id);
CREATE INDEX IF NOT EXISTS idx_dapps_contract_address ON dapps(contract_address);
CREATE INDEX IF NOT EXISTS idx_dapp_states_dapp_id ON dapp_states(dapp_id);

-- Insert some initial test data
INSERT OR IGNORE INTO token_balances (address, balance) VALUES 
    ('0x742d35Cc6634C0532925a3b8D42Fc5CeBa3a7f79', '1000000000000000000000'),
    ('0x8ba1f109551bD432803012645Hac136c3c7E2c8a', '500000000000000000000');