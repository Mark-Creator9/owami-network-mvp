# Owami Network

A Layer 0 blockchain ecosystem tailored to develop the economic landscape of Africa through decentralized solutions.

## Overview

Owami Network is a blockchain infrastructure designed specifically to address the economic challenges and opportunities in Africa. By leveraging blockchain technology with unique accessibility features such as USSD integration, Owami Network enables financial inclusion for all Africans, including those in remote areas without smartphones or internet access.

## Vision

Our vision is to create a decentralized ecosystem that empowers African economies through accessible blockchain solutions, enabling the development and deployment of dApps such as:

- Decentralized Autonomous Organizations (DAOs)
- Decentralized Exchanges (DEXs)
- Local Cryptocurrencies
- Supply Chain Management Solutions
- Investment Fund DAOs
- Microfinance Platforms
- Agricultural Marketplaces
- Decentralized AI Agents

## Technical Architecture

Owami Network is built as a Layer 0 blockchain with several key components:

### Core Components

- **Consensus Mechanism:** A hybrid approach combining Federated Byzantine Agreement (FBA), Delegated Proof of Stake (DPoS), and Proof of Authority (PoA) for efficient and secure validation.
- **Blake3 Hashing:** Utilizing the latest version of Blake3 for high-performance cryptographic operations.
- **P2P Network Layer:** Built on libp2p for robust peer-to-peer communication.
- **Token System:** Owa TestToken (for testnet) and Owa Token (for mainnet) used as the utility token for the network.
- **USSD Integration:** SMS-based access to blockchain features, enabling users with basic phones to participate in the network.
- **Storage Layer:** Efficient blockchain data storage with sled database.
- **Transaction System:** Support for various transaction types including token transfers and USSD commands.

### Technical Stack

- Language: Rust for performance, safety, and reliability
- Networking: libp2p for P2P communication
- Cryptography: ed25519-dalek for signatures, Blake3 for hashing
- Storage: sled for efficient database operations
- Serialization: serde and bincode for data serialization

### Architecture Overview

The Owami Network consists of two main components:

- **Node Application:** The core blockchain node with USSD API
- **CLI Tool:** Command line interface for interacting with the node

## Current Build Details

- **Node Application:** Implemented in Rust, providing REST API endpoints for wallet creation, balance retrieval, faucet requests, transaction sending, and transaction history.
- **API Server:** Uses Actix-web framework with CORS enabled and JWT authentication.
- **Batch Processing:** Transactions are batched asynchronously for efficient processing.
- **Testing:** Comprehensive unit and integration tests covering blockchain, token, and API functionality.
- **Performance:** Current testnet supports ~2.7 TPS sequentially, with optimizations for concurrency and batching.

## Getting Started

### Prerequisites

- Rust 1.53.0 or higher
- Cargo package manager
- A C compiler (e.g., GCC or Clang) for building certain dependencies

### Installation and Running

#### Installing a C Compiler
To install a C compiler, follow these steps:

1. For Windows:
   - Download and install [Build Tools for Visual Studio](https://visualstudio.microsoft.com/downloads/).
   - Make sure to select a Free Download for Community "C++ build tools" during installation.

2. For macOS (using Homebrew):
   - Run: `brew install gcc`

3. For Linux (Ubuntu/Debian-based):
   - Run: `sudo apt-get update && sudo apt-get install build-essential`

After installing a C compiler, verify it's correctly installed by running:
```bash
gcc --version
```
or
```bash
clang --version
```

1. Clone the repository and navigate into it:
    ```bash
    git clone https://github.com/Mark-Creator9/owami-network
    cd owami-network
    ```

2. Build and run the project:
    ```bash
    cargo run
    ```

3. Access the API endpoint at [http://localhost:8080/](http://localhost:8080/) and the landing page at [http://localhost:8080/landing/index.html](http://localhost:8080/landing/index.html).

4. (Optional) To use the CLI tool, navigate to the CLI directory and run:
    ```bash
    cd owami-cli
    cargo run -- --help
    ```

## Usage Guide

### Node Operation

- Nodes discover peers via mDNS.
- Validators are auto-registered with initial stake on testnet.
- USSD commands supported:
  - Check Balance: `*144*1#`
  - Send Payment: `*144*2#`
  - Receive Payment: `*144*3#`
  - Help Menu: `*144*0#`

### Token Operations

- Token transfers via wallet address or phone number.
- Payment requests via generated codes.

### Running Test Scripts

To run the test scripts:

1. Start the server using `cargo run`.
2. In a new terminal, run `powershell ./test_all_scenarios.ps1` to test all scenarios.
3. Alternatively, run `powershell ./test_alice_bob.ps1` to test the scenario between Alice and Bob.

These scripts will test the API endpoints and provide output for the various test cases.

## Deployment on Render

### Prerequisites
1. A Render account
2. PostgreSQL database instance on Render

### Setting up PostgreSQL on Render
1. Create a new PostgreSQL database in your Render dashboard
2. Note down the following connection details:
   - Internal Database URL
   - External Database URL
   - User
   - Password
   - Host
   - Port
   - Database name

### Deploying the Application
1. Create a new Web Service in Render
2. Connect your repository
3. Choose "Docker" as the environment
4. Set the following environment variables:
   - `POSTGRES_USER`: Database user from Render PostgreSQL
   - `POSTGRES_PASSWORD`: Database password
   - `POSTGRES_HOST`: Database host
   - `POSTGRES_PORT`: Database port (usually 5432)
   - `POSTGRES_DATABASE`: Database name
   - `JWT_SECRET`: Your secure JWT secret
   - `RUST_LOG`: Set to "info" for production logging
   - `PORT`: Set to 8000

### Post-Deployment
1. The application will automatically:
   - Install required dependencies
   - Run database migrations
   - Build and start the Rust application
2. Monitor the deployment logs to ensure successful startup
3. Your API will be available at your Render service URL

## Development Roadmap

- **Current:** MVP Testnet with core blockchain, USSD integration.
- **Next:** Enhanced testnet with improved consensus, SDK, and dApp ecosystem.
- **Future:** Mainnet launch with ICO, governance, cross-chain bridges, and full dApp marketplace.

## Contributing

We welcome contributions! Please fork the repository, create a feature branch, commit your changes, and open a pull request.

Owami Network - Empowering Africa's Economic Future Through Decentralized Solutions

Note: This is a private repository. Contributions are welcome, but access must be granted by the repository owner.
