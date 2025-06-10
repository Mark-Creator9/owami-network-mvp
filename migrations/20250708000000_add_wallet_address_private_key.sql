-- Add address and private_key columns to wallets table
ALTER TABLE wallets
ADD COLUMN address TEXT UNIQUE,
ADD COLUMN private_key TEXT;