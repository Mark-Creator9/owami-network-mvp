-- Migration: alter id columns to BIGINT to match Rust i64 models
BEGIN;
ALTER TABLE blocks ALTER COLUMN id TYPE BIGINT USING id::BIGINT;
ALTER TABLE transactions ALTER COLUMN id TYPE BIGINT USING id::BIGINT;
ALTER TABLE pending_transactions ALTER COLUMN id TYPE BIGINT USING id::BIGINT;
-- If any sequences exist for these SERIAL columns they will continue to work; this migration ensures column types match Rust models.
COMMIT;