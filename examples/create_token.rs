use sss_shared::create_new_token;
use std::error::Error;
use std::env;

fn main() -> Result<(), Box<dyn Error>> {
    // Set the environment variable directly in the code
    unsafe { env::set_var("PAYER_MNEMONIC", "index today clay record roast pride attitude flip room frame ghost dumb") };
    unsafe { env::set_var("SOLANA_RPC_URL", "https://young-purple-ensemble.solana-devnet.quiknode.pro/d2eb170d87e75c79d44cfafc57638a8105d3b245/") };
    
    // Print the current directory to check where we're looking for the .env file
    println!("Current directory: {:?}", env::current_dir()?);
    
    // Try to read the mnemonic directly to see if it exists
    match env::var("PAYER_MNEMONIC") {
        Ok(mnemonic) => println!("Found mnemonic: {}", mnemonic),
        Err(e) => println!("Failed to read mnemonic: {}", e),
    }
    
    // Set up token parameters
    let uri = "https://example.com/token-metadata.json".to_string();
    let name = "Test Token".to_string();
    let decimals = 6;

    println!("Creating new token: {}", name);
    
    // Create the token
    match create_new_token(uri, name, decimals) {
        Ok((signature, mint_pubkey)) => {
            println!("✅ Token created successfully!");
            println!("Transaction signature: {}", signature);
            println!("Mint address: {}", mint_pubkey);
            
            println!("View on Solana Explorer: https://explorer.solana.com/address/{}?cluster=devnet", mint_pubkey);
            Ok(())
        },
        Err(e) => {
            println!("❌ Error creating token: {}", e);
            Err(e)
        }
    }
}