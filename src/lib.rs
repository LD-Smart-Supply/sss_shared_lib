use bip39::{Language, Mnemonic, Seed};
use dotenv::dotenv;
use lazy_static::lazy_static;
use mpl_token_metadata::{instructions::CreateV1Builder, types::TokenStandard};
use solana_rpc_client::rpc_client::RpcClient;
use solana_sdk::{
    message::Message,
    pubkey::Pubkey,
    signature::{Keypair, keypair_from_seed},
    signer::Signer,
    transaction::Transaction,
};
use std::{
    env,
    sync::{Arc, Mutex},
};

// Initialize the RPC client using environment variables
lazy_static! {
    pub static ref RPC_CLIENT: RpcClient = {
        dotenv().ok();
        let rpc_url = env::var("SOLANA_RPC_URL")
            .unwrap_or_else(|_| "https://api.devnet.solana.com".to_string());
        RpcClient::new(rpc_url)
    };

    pub static ref PAYER_RESULT: Arc<Mutex<Result<Keypair, String>>> = {
        dotenv().ok();
        let result = match env::var("PAYER_MNEMONIC") {
            Ok(mnemonic_phrase) => {
                match Mnemonic::from_phrase(&mnemonic_phrase, Language::English) {
                    Ok(mnemonic) => {
                        let seed = Seed::new(&mnemonic, "");
                        match keypair_from_seed(seed.as_bytes()) {
                            Ok(keypair) => Ok(keypair),
                            Err(e) => Err(format!("Failed to derive keypair from seed: {}", e)),
                        }
                    },
                    Err(e) => Err(format!("Invalid mnemonic phrase: {}", e)),
                }
            },
            Err(e) => Err(format!("Payer mnemonic not found in .env file: {}", e)),
        };
        Arc::new(Mutex::new(result))
    };
}

// Helper function to get the payer keypair
pub fn get_payer() -> Result<Keypair, Box<dyn std::error::Error>> {
    let lock_result = PAYER_RESULT
        .lock()
        .map_err(|e| format!("Failed to acquire lock: {}", e))?;

    match &*lock_result {
        Ok(keypair) => {
            // Create a new keypair from the bytes of the existing one
            let bytes = keypair.to_bytes();
            match Keypair::from_bytes(&bytes) {
                Ok(new_keypair) => Ok(new_keypair),
                Err(e) => Err(format!("Failed to create keypair from bytes: {}", e).into()),
            }
        }
        Err(e) => Err(e.clone().into()),
    }
}

// Function to create and send a fungible token transaction
pub fn create_consumable_token(
    mint: &Keypair,
    uri: String,
    name: String,
    decimals: u8,
) -> Result<String, Box<dyn std::error::Error>> {
    // Get the payer keypair
    let payer = get_payer()?;

    // Create the instruction to create a consumable token
    let create_ix = CreateV1Builder::new()
        .mint(mint.pubkey(), true)
        .authority(payer.pubkey())
        .payer(payer.pubkey())
        .update_authority(payer.pubkey(), false)
        .name(name)
        .uri(uri)
        .token_standard(TokenStandard::Fungible)
        .decimals(decimals)
        .instruction();

    // Create the message
    let message = Message::new(&[create_ix], Some(&payer.pubkey()));

    // Get the latest blockhash
    let blockhash = RPC_CLIENT.get_latest_blockhash()?;

    // Create and sign the transaction
    let tx = Transaction::new(&[mint, &payer], message, blockhash);

    // Send and confirm the transaction
    let signature = RPC_CLIENT.send_and_confirm_transaction(&tx)?;

    Ok(signature.to_string())
}

// Convenience function that generates a mint keypair for you
pub fn create_new_token(
    uri: String,
    name: String,
    decimals: u8,
) -> Result<(String, Pubkey), Box<dyn std::error::Error>> {
    let mint = Keypair::new();
    
    let signature = create_consumable_token(&mint, uri, name, decimals)?;
    
    Ok((signature, mint.pubkey()))
}

// Example usage:
// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let uri = "https://example.com/metadata.json".to_string();
//     let name = "My Token".to_string();
//     let decimals = 6;
//
//     match create_new_token(uri, name, decimals) {
//         Ok((signature, mint_pubkey)) => {
//             println!("Transaction signature: {}", signature);
//             println!("Mint address: {}", mint_pubkey);
//             Ok(())
//         },
//         Err(e) => {
//             eprintln!("Error creating token: {}", e);
//             Err(e)
//         }
//     }
// }
