-- Create token_balances table
CREATE TABLE IF NOT EXISTS token_balances (
    address VARCHAR(42) PRIMARY KEY,
    balance VARCHAR(78) NOT NULL DEFAULT '0'
);

-- Create token_allowances table
CREATE TABLE IF NOT EXISTS token_allowances (
    owner VARCHAR(42) NOT NULL,
    spender VARCHAR(42) NOT NULL,
    amount VARCHAR(78) NOT NULL,
    PRIMARY KEY (owner, spender)
);

-- Create token_transactions table
CREATE TABLE IF NOT EXISTS token_transactions (
    id SERIAL PRIMARY KEY,
    tx_hash VARCHAR(66) NOT NULL,
    from_address VARCHAR(42) NOT NULL,
    to_address VARCHAR(42) NOT NULL,
    amount VARCHAR(78) NOT NULL,
    timestamp BIGINT NOT NULL
);

-- Create dapps table
CREATE TABLE IF NOT EXISTS dapps (
    id VARCHAR(36) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    address VARCHAR(42) NOT NULL,
    owner VARCHAR(42) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create dapp_states table
CREATE TABLE IF NOT EXISTS dapp_states (
    dapp_id VARCHAR(36) REFERENCES dapps(id) ON DELETE CASCADE,
    key VARCHAR(255) NOT NULL,
    value JSONB,
    PRIMARY KEY (dapp_id, key)
);

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_token_transactions_from ON token_transactions(from_address);
CREATE INDEX IF NOT EXISTS idx_token_transactions_to ON token_transactions(to_address);
CREATE INDEX IF NOT EXISTS idx_token_transactions_timestamp ON token_transactions(timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_dapps_owner ON dapps(owner);
CREATE INDEX IF NOT EXISTS idx_dapps_address ON dapps(address);