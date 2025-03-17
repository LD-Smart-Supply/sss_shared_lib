use sss_shared::create_new_token;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    
    // Set up token parameters
    let uri = "https://example.com/token-metadata.json".to_string();
    let name = "Test Token 666".to_string();
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
            Err(Box::new(e))
        }
    }
}