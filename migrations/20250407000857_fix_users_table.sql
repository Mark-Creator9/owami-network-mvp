 -- Fix users table schema if it exists
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = 'users') THEN
        ALTER TABLE users 
        ALTER COLUMN password_hash SET NOT NULL,
        ALTER COLUMN created_at SET DEFAULT CURRENT_TIMESTAMP,
        ALTER COLUMN updated_at SET DEFAULT CURRENT_TIMESTAMP;

        -- Create proper indexes
        CREATE UNIQUE INDEX IF NOT EXISTS idx_users_username ON users(username);
    END IF;
END $$;
