-- Enable pgcrypto for password hashing
CREATE EXTENSION IF NOT EXISTS pgcrypto;

-- Add test users with bcrypt hashed passwords
INSERT INTO users (username, password_hash) 
VALUES 
    ('test', crypt('test123', gen_salt('bf'))),
    ('admin', crypt('admin123', gen_salt('bf')))
ON CONFLICT (username) DO NOTHING;