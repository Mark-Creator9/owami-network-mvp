-- Create users table for authentication
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(255) NOT NULL UNIQUE,
    password_hash TEXT NOT NULL, -- Store bcrypt hashes
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_login TIMESTAMPTZ
);

-- Create index for username lookup  
CREATE INDEX idx_users_username ON users(username);

-- Add admin user (temporary for development)
INSERT INTO users (username, password_hash)
VALUES (
    'admin',
    crypt('admin123', gen_salt('bf'))
);