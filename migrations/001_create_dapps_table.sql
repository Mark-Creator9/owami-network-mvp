-- Create dapps table
CREATE TABLE IF NOT EXISTS dapps (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    contract_address VARCHAR(255) NOT NULL,
    creator_id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create index on contract_address for faster lookups
CREATE INDEX IF NOT EXISTS idx_dapps_contract_address ON dapps(contract_address);

-- Create index on creator_id for faster lookups
CREATE INDEX IF NOT EXISTS idx_dapps_creator_id ON dapps(creator_id);