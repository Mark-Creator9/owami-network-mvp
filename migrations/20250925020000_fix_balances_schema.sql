-- Safety migration: ensure balances table exists and has correct schema for Postgres
-- Goal:
-- - Ensure table `balances` exists
-- - Ensure `address` is VARCHAR(128) PRIMARY KEY
-- - Ensure `balance` is BIGINT NOT NULL DEFAULT 0 (convert if previously text)
-- - Ensure `updated_at` exists as TIMESTAMP DEFAULT NOW()

-- 1) Ensure balances table exists with desired columns
CREATE TABLE IF NOT EXISTS balances (
    address VARCHAR(128) PRIMARY KEY,
    balance BIGINT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP DEFAULT NOW()
);

-- 2) Widen address to VARCHAR(128) (safe no-op if already wider)
ALTER TABLE IF EXISTS balances
    ALTER COLUMN address TYPE VARCHAR(128);

-- 3) Ensure updated_at column exists
ALTER TABLE IF EXISTS balances
    ADD COLUMN IF NOT EXISTS updated_at TIMESTAMP DEFAULT NOW();

-- 4) Ensure balance column is BIGINT and convert if previously stored as text
DO $$
DECLARE
    v_data_type text;
BEGIN
    SELECT data_type INTO v_data_type
    FROM information_schema.columns
    WHERE table_name = 'balances' AND column_name = 'balance'
    LIMIT 1;

    IF v_data_type IS NULL THEN
        -- Column missing: add it
        EXECUTE 'ALTER TABLE balances ADD COLUMN balance BIGINT NOT NULL DEFAULT 0';
    ELSIF v_data_type <> 'bigint' THEN
        -- Attempt conversion from text/varchar/numeric to BIGINT, fallback non-numeric to 0
        EXECUTE 'ALTER TABLE balances ALTER COLUMN balance TYPE BIGINT USING '
             || 'CASE WHEN balance IS NULL THEN 0 '
             || 'WHEN trim(balance::text) ~ ''^[0-9]+$'' THEN balance::bigint '
             || 'ELSE 0 END';
    END IF;
END $$;

-- 5) Ensure an index on address exists (redundant if PK, but harmless)
CREATE INDEX IF NOT EXISTS idx_balances_address ON balances(address);