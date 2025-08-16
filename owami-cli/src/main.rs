use clap::{Parser, Subcommand};
use reqwest::Client;
use serde_json::json;
use std::error::Error;
use std::path::Path;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    #[arg(long, default_value = "http://localhost:8080")]
    api_url: String,
    
    #[arg(short, long)]
    api_key: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Check server status
    Status,
    /// Generate a new API key
    GenerateKey {
        #[arg(short, long)]
        email: String,
        #[arg(short, long)]
        permissions: Vec<String>,
    },
    /// Get test tokens from faucet
    Faucet {
        #[arg(short, long)]
        address: String,
        #[arg(short, long, default_value = "1000")]
        amount: u64,
    },
    /// Deploy a smart contract
    Deploy {
        /// Path to the smart contract file
        #[arg(short, long)]
        file: String,
        /// Network to deploy to (testnet or mainnet)
        #[arg(short, long, default_value = "testnet")]
        network: String,
    },
    /// Call a smart contract function
    Call {
        /// Contract address
        #[arg(short, long)]
        address: String,
        /// Function name to call
        #[arg(short, long)]
        function: String,
        /// Parameters for the function call (JSON format)
        #[arg(short, long)]
        params: Option<String>,
        /// Network to call on (testnet or mainnet)
        #[arg(short, long, default_value = "testnet")]
        network: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let client = Client::new();

    match cli.command {
        Commands::Status => {
            let response = client.get(&format!("{}/", cli.api_url))
                .send()
                .await?;
            println!("Server Status: {}", response.text().await?);
        }
        Commands::GenerateKey { email, permissions } => {
            let response = client.post(&format!("{}/auth/keys", cli.api_url))
                .json(&json!({
                    "email": email,
                    "permissions": permissions
                }))
                .send()
                .await?;
            println!("API Key: {}", response.text().await?);
        }
        Commands::Faucet { address, amount } => {
            let response = client.post(&format!("{}/testnet/faucet", cli.api_url))
                .json(&json!({
                    "wallet_address": address,
                    "amount": amount
                }))
                .send()
                .await?;
            let status = response.status();
            let text = response.text().await?;
            
            match serde_json::from_str::<serde_json::Value>(&text) {
                Ok(json) => {
                    println!("Faucet Response ({}):\n{}",
                        status,
                        serde_json::to_string_pretty(&json)?
                    );
                }
                Err(_) => {
                    println!("Faucet Response ({}):\n{}", status, text);
                }
            }
        }
        Commands::Deploy { file, network } => {
            // Check if file exists
            if !Path::new(&file).exists() {
                return Err(format!("File {} does not exist", file).into());
            }

            // For now, we'll simulate the deployment by calling the API endpoint
            // In a real implementation, this would compile and deploy the contract
            let response = client.post(&format!("{}/api/dapp/deploy", cli.api_url))
                .json(&json!({
                    "contract_path": file,
                    "network": network
                }))
                .send()
                .await?;

            let status = response.status();
            let text = response.text().await?;
            
            match serde_json::from_str::<serde_json::Value>(&text) {
                Ok(json) => {
                    println!("Deployment Response ({}):\n{}",
                        status,
                        serde_json::to_string_pretty(&json)?
                    );
                }
                Err(_) => {
                    println!("Deployment Response ({}):\n{}", status, text);
                }
            }
        }
        Commands::Call { address, function, params, network } => {
            // Parse params if provided
            let params_json = if let Some(p) = params {
                match serde_json::from_str::<serde_json::Value>(&p) {
                    Ok(json) => Some(json),
                    Err(e) => return Err(format!("Invalid JSON in params: {}", e).into()),
                }
            } else {
                None
            };

            // Call the API endpoint
            let response = client.post(&format!("{}/api/dapp/call", cli.api_url))
                .json(&json!({
                    "contract_address": address,
                    "function_name": function,
                    "params": params_json,
                    "network": network
                }))
                .send()
                .await?;

            let status = response.status();
            let text = response.text().await?;
            
            match serde_json::from_str::<serde_json::Value>(&text) {
                Ok(json) => {
                    println!("Call Response ({}):\n{}",
                        status,
                        serde_json::to_string_pretty(&json)?
                    );
                }
                Err(_) => {
                    println!("Call Response ({}):\n{}", status, text);
                }
            }
        }
    }

    Ok(())
}