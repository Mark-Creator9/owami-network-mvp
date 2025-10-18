-- Complete schema migration for PostgreSQL
-- Combines all previous migrations into a single PostgreSQL-compatible script

-- Create users table
CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Create token balances table
CREATE TABLE IF NOT EXISTS token_balances (
    address VARCHAR(42) PRIMARY KEY,
    balance VARCHAR(78) NOT NULL DEFAULT '0'
);

-- Create token transactions table
CREATE TABLE IF NOT EXISTS token_transactions (
    transaction_hash VARCHAR(66) PRIMARY KEY,
    from_address VARCHAR(42) NOT NULL,
    to_address VARCHAR(42) NOT NULL,
    amount VARCHAR(78) NOT NULL,
    block_number INTEGER NOT NULL,
    timestamp TIMESTAMP DEFAULT NOW()
);

-- Create token approvals table
CREATE TABLE IF NOT EXISTS token_approvals (
    owner VARCHAR(42) NOT NULL,
    spender VARCHAR(42) NOT NULL,
    amount VARCHAR(78) NOT NULL,
    PRIMARY KEY (owner, spender)
);

-- Create dapps table
CREATE TABLE IF NOT EXISTS dapps (
    id TEXT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    contract_address VARCHAR(42) NOT NULL,
    creator_id TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Create dapp states table
CREATE TABLE IF NOT EXISTS dapp_states (
    dapp_id TEXT NOT NULL,
    key VARCHAR(255) NOT NULL,
    value TEXT NOT NULL,
    updated_at TIMESTAMP DEFAULT NOW(),
    PRIMARY KEY (dapp_id, key),
    FOREIGN KEY (dapp_id) REFERENCES dapps(id) ON DELETE CASCADE
);

-- Create blocks table
CREATE TABLE IF NOT EXISTS blocks (
    id BIGSERIAL PRIMARY KEY,
    height BIGINT NOT NULL UNIQUE,
    previous_hash VARCHAR(64) NOT NULL,
    hash VARCHAR(64) NOT NULL UNIQUE,
    timestamp BIGINT NOT NULL,
    validator VARCHAR(42) NOT NULL,
    transactions BYTEA NOT NULL, -- Serialized transactions array
    created_at TIMESTAMP DEFAULT NOW()
);

-- Create transactions table
CREATE TABLE IF NOT EXISTS transactions (
    id BIGSERIAL PRIMARY KEY,
    hash VARCHAR(64) NOT NULL UNIQUE,
    from_address VARCHAR(42) NOT NULL,
    to_address VARCHAR(42) NOT NULL,
    amount BIGINT NOT NULL,
    signature TEXT NOT NULL,
    block_height BIGINT,
    created_at TIMESTAMP DEFAULT NOW(),
    FOREIGN KEY (block_height) REFERENCES blocks(height) ON DELETE SET NULL
);

-- Create balances table
CREATE TABLE IF NOT EXISTS balances (
    address VARCHAR(42) PRIMARY KEY,
    balance BIGINT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Create pending_transactions table
CREATE TABLE IF NOT EXISTS pending_transactions (
    id BIGSERIAL PRIMARY KEY,
    transaction_data BYTEA NOT NULL, -- Serialized transaction
    created_at TIMESTAMP DEFAULT NOW()
);

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_token_transactions_from ON token_transactions(from_address);
CREATE INDEX IF NOT EXISTS idx_token_transactions_to ON token_transactions(to_address);
CREATE INDEX IF NOT EXISTS idx_token_transactions_timestamp ON token_transactions(timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_dapps_creator ON dapps(creator_id);
CREATE INDEX IF NOT EXISTS idx_dapps_address ON dapps(contract_address);
CREATE INDEX IF NOT EXISTS idx_blocks_height ON blocks(height);
CREATE INDEX IF NOT EXISTS idx_blocks_hash ON blocks(hash);
CREATE INDEX IF NOT EXISTS idx_blocks_timestamp ON blocks(timestamp);
CREATE INDEX IF NOT EXISTS idx_transactions_hash ON transactions(hash);
CREATE INDEX IF NOT EXISTS idx_transactions_from ON transactions(from_address);
CREATE INDEX IF NOT EXISTS idx_transactions_to ON transactions(to_address);
CREATE INDEX IF NOT EXISTS idx_transactions_block_height ON transactions(block_height);
CREATE INDEX IF NOT EXISTS idx_balances_address ON balances(address);
CREATE INDEX IF NOT EXISTS idx_pending_transactions_created_at ON pending_transactions(created_at);