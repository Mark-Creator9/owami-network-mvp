@echo off
echo Starting Owami Network Server...
set DATABASE_URL=sqlite:owami_testnet.db
set PORT=3000
cargo run