-- Token balances table
CREATE TABLE IF NOT EXISTS token_balances (
    address VARCHAR(42) PRIMARY KEY,
    balance VARCHAR(78) NOT NULL DEFAULT '0'
);

-- Token transactions table
CREATE TABLE IF NOT EXISTS token_transactions (
    transaction_hash VARCHAR(66) PRIMARY KEY,
    from_address VARCHAR(42) NOT NULL,
    to_address VARCHAR(42) NOT NULL,
    amount VARCHAR(78) NOT NULL,
    block_number INTEGER NOT NULL,
    timestamp TIMESTAMPTZ DEFAULT NOW()
);

-- Token approvals table
CREATE TABLE IF NOT EXISTS token_approvals (
    owner VARCHAR(42) NOT NULL,
    spender VARCHAR(42) NOT NULL,
    amount VARCHAR(78) NOT NULL,
    PRIMARY KEY (owner, spender)
);

-- DApps table
CREATE TABLE IF NOT EXISTS dapps (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    address VARCHAR(42) NOT NULL,
    owner VARCHAR(42) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- DApp states table
CREATE TABLE IF NOT EXISTS dapp_states (
    dapp_id INTEGER NOT NULL,
    key VARCHAR(255) NOT NULL,
    value TEXT NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    PRIMARY KEY (dapp_id, key),
    FOREIGN KEY (dapp_id) REFERENCES dapps(id) ON DELETE CASCADE
);

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_token_transactions_from ON token_transactions(from_address);
CREATE INDEX IF NOT EXISTS idx_token_transactions_to ON token_transactions(to_address);
CREATE INDEX IF NOT EXISTS idx_token_transactions_timestamp ON token_transactions(timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_dapps_owner ON dapps(owner);
CREATE INDEX IF NOT EXISTS idx_dapps_address ON dapps(address);