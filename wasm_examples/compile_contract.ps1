# PowerShell script to compile the simple_contract to WASM

# Navigate to the contract directory
Set-Location simple_contract

# Install the wasm32 target if not already installed
rustup target add wasm32-unknown-unknown

# Build the contract
cargo build --target wasm32-unknown-unknown --release

# Copy the WASM file to the examples directory
Copy-Item target/wasm32-unknown-unknown/release/simple_contract.wasm -Destination ../simple_contract.wasm

# Return to original directory
Set-Location ..