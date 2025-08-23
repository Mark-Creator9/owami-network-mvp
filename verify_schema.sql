-- Check all tables in the database
\dt

-- Check the structure of each table
\d token_balances
\d token_transactions
\d token_approvals
\d dapps
\d dapp_states
\d users
\d wallets

-- Check if tables exist
SELECT table_name 
FROM information_schema.tables 
WHERE table_schema = 'public' 
ORDER BY table_name;