use clap::{Parser, Subcommand};
use reqwest::Client;
use serde_json::json;
use std::error::Error;

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
    }

    Ok(())
}