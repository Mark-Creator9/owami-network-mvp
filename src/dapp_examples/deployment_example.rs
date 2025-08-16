use std::process::Command;

fn main() {
    // This script demonstrates deploying a smart contract to the Owami Network using owami-cli.
    // Ensure owami-cli is installed and available in your PATH.
    
    let output = Command::new("owami-cli")
        .arg("contract")
        .arg("deploy")
        .arg("--file")
        .arg("src/dapp_examples/smart_contract.sol")
        .arg("--network")
        .arg("mainnet") // or testnet, depending on the environment
        .output()
        .expect("Failed to execute owami-cli command");
    
    if output.status.success() {
        println!("Contract deployed successfully!");
        println!("Output: {}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("Deployment failed: {}", String::from_utf8_lossy(&output.stderr));
    }
}