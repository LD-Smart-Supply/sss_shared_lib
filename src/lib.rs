//! SSS Shared Library for Solana token operations
//!
//! This library provides functionality for creating and managing tokens on the Solana blockchain.
//! It includes both Rust functions for direct use and FFI functions for C interoperability.

mod error;
mod ffi;
mod ffi_utils;
mod token;

pub use error::{SssError, SssResult};
pub use ffi::{create_token, free_string, mint_token_ffi};
pub use token::{
    create_consumable_token, create_new_token, fetch_digital_assets_by_owner, mint_token,
};

use bip39::{Language, Mnemonic, Seed};
use dotenv::dotenv;
use lazy_static::lazy_static;
use solana_rpc_client::rpc_client::RpcClient;
use solana_sdk::signature::{Keypair, keypair_from_seed};
use std::{
    env,
    sync::{Arc, Mutex},
};

// Initialize the RPC client using environment variables
lazy_static! {
    /// Global RPC client initialized from environment variables
    pub static ref RPC_CLIENT: RpcClient = {
        dotenv().ok();
        let rpc_url = env::var("SOLANA_RPC_URL")
            .unwrap_or_else(|_| "https://api.devnet.solana.com".to_string());
        RpcClient::new(rpc_url)
    };

    /// Global payer keypair result initialized from environment variables
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
                    }
                    Err(e) => Err(format!("Invalid mnemonic phrase: {}", e)),
                }
            }
            Err(e) => Err(format!("Payer mnemonic not found in .env file: {}", e)),
        };
        Arc::new(Mutex::new(result))
    };
}

/// Helper function to get the payer keypair
///
/// # Returns
///
/// A new keypair cloned from the global payer keypair
///
/// # Errors
///
/// Returns an error if the payer keypair is not initialized or if there's an error cloning it
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
