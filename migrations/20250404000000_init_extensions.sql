-- Add token-related tables
CREATE TABLE IF NOT EXISTS token_balances (
    address VARCHAR(42) PRIMARY KEY,
    balance VARCHAR(78) NOT NULL DEFAULT '0'
);

CREATE TABLE IF NOT EXISTS token_allowances (
    owner VARCHAR(42) NOT NULL,
    spender VARCHAR(42) NOT NULL,
    amount VARCHAR(78) NOT NULL,
    PRIMARY KEY (owner, spender)
);

CREATE TABLE IF NOT EXISTS token_transactions (
    id SERIAL PRIMARY KEY,
    tx_hash VARCHAR(66) NOT NULL UNIQUE,
    from_address VARCHAR(42) NOT NULL,
    to_address VARCHAR(42) NOT NULL,
    amount VARCHAR(78) NOT NULL,
    timestamp BIGINT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_token_transactions_timestamp ON token_transactions(timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_token_transactions_from ON token_transactions(from_address);
CREATE INDEX IF NOT EXISTS idx_token_transactions_to ON token_transactions(to_address);

-- Insert initial token balances for testing
INSERT INTO token_balances (address, balance) VALUES 
    ('0x742d35Cc6634C0532925a3b8D4e6D3b6e8d3e8A0', '1000000000000000000000000')
ON CONFLICT (address) DO NOTHING;

INSERT INTO token_balances (address, balance) VALUES 
    ('0x1234567890123456789012345678901234567890', '10000000000000000000000')
ON CONFLICT (address) DO NOTHING;

INSERT INTO token_balances (address, balance) VALUES 
    ('0x0987654321098765432109876543210987654321', '5000000000000000000000')
ON CONFLICT (address) DO NOTHING;