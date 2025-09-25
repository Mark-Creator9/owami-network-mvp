-- Create blocks table for PostgreSQL
CREATE TABLE IF NOT EXISTS blocks (
    id SERIAL PRIMARY KEY,
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
    id SERIAL PRIMARY KEY,
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
    id SERIAL PRIMARY KEY,
    transaction_data BYTEA NOT NULL, -- Serialized transaction
    created_at TIMESTAMP DEFAULT NOW()
);

-- Create indexes for performance
CREATE INDEX IF NOT EXISTS idx_blocks_height ON blocks(height);
CREATE INDEX IF NOT EXISTS idx_blocks_hash ON blocks(hash);
CREATE INDEX IF NOT EXISTS idx_blocks_timestamp ON blocks(timestamp);
CREATE INDEX IF NOT EXISTS idx_transactions_hash ON transactions(hash);
CREATE INDEX IF NOT EXISTS idx_transactions_from ON transactions(from_address);
CREATE INDEX IF NOT EXISTS idx_transactions_to ON transactions(to_address);
CREATE INDEX IF NOT EXISTS idx_transactions_block_height ON transactions(block_height);
CREATE INDEX IF NOT EXISTS idx_balances_address ON balances(address);
CREATE INDEX IF NOT EXISTS idx_pending_transactions_created_at ON pending_transactions(created_at);