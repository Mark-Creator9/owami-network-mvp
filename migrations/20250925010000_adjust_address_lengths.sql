-- Adjust address field lengths to support 64-char hex addresses
-- and ensure compatibility with ED25519 public keys

-- balances.address: 64-hex chars -> widen to VARCHAR(128)
ALTER TABLE IF EXISTS balances
    ALTER COLUMN address TYPE VARCHAR(128);

-- transactions.from_address and to_address: widen to VARCHAR(128)
ALTER TABLE IF EXISTS transactions
    ALTER COLUMN from_address TYPE VARCHAR(128),
    ALTER COLUMN to_address TYPE VARCHAR(128);