# PowerShell script to compile the simple_contract to WASM

# Navigate to the contract directory
Set-Location wasm_examples/simple_contract

# Install the wasm32 target if not already installed
rustup target add wasm32-unknown-unknown

# Build the contract
cargo build --target wasm32-unknown-unknown

# Return to original directory
Set-Location ../..