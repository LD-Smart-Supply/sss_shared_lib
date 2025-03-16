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
    ffi::{CStr, CString},
    os::raw::{c_char, c_int, c_uchar},
    ptr,
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

    // Derive the metadata account PDA
    let seeds = &[
        "metadata".as_bytes(),
        &mpl_token_metadata::ID.to_bytes(),
        &mint.pubkey().to_bytes(),
    ];
    let (metadata_account, _) = Pubkey::find_program_address(seeds, &mpl_token_metadata::ID);

    // Create the instruction to create a consumable token
    let create_ix = CreateV1Builder::new()
        .metadata(metadata_account)
        .mint(mint.pubkey(), true)
        .authority(payer.pubkey())
        .payer(payer.pubkey())
        .update_authority(payer.pubkey(), false)
        .name(name)
        .uri(uri)
        .seller_fee_basis_points(0)
        .symbol("".to_string())
        .token_standard(TokenStandard::Fungible)
        .decimals(decimals)
        .spl_token_program(Some(spl_token::id()))
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

// FFI functions

/// Creates a new token and returns the transaction signature and mint address
/// 
/// # Safety
/// 
/// This function is unsafe because it works with raw pointers for C interoperability.
/// The caller must ensure that:
/// - uri_ptr and name_ptr are valid, null-terminated C strings
/// - signature_out and mint_address_out are valid pointers to buffers of sufficient size
///
/// @param uri_ptr A pointer to a null-terminated C string containing the token URI
/// @param name_ptr A pointer to a null-terminated C string containing the token name
/// @param decimals The number of decimal places for the token
/// @param signature_out A pointer to a buffer where the transaction signature will be written
/// @param mint_address_out A pointer to a buffer where the mint address will be written
/// @return 0 on success, non-zero error code on failure
#[unsafe(no_mangle)]
pub unsafe extern "C" fn create_token(
    uri_ptr: *const c_char,
    name_ptr: *const c_char,
    decimals: c_uchar,
    signature_out: *mut c_char,
    mint_address_out: *mut c_char,
    signature_len: c_int,
    mint_address_len: c_int,
) -> c_int {
    // Check for null pointers
    if uri_ptr.is_null() || name_ptr.is_null() || signature_out.is_null() || mint_address_out.is_null() {
        return -1;
    }

    // Convert C strings to Rust strings
    let uri = match unsafe { CStr::from_ptr(uri_ptr).to_str() } {
        Ok(s) => s.to_string(),
        Err(_) => return -2,
    };

    let name = match unsafe { CStr::from_ptr(name_ptr).to_str() } {
        Ok(s) => s.to_string(),
        Err(_) => return -3,
    };

    // Call the Rust function
    match create_new_token(uri, name, decimals) {
        Ok((signature, mint_pubkey)) => {
            // Convert the signature to a C string
            let signature_cstr = match CString::new(signature) {
                Ok(s) => s,
                Err(_) => return -4,
            };
            
            // Convert the mint address to a C string
            let mint_address_cstr = match CString::new(mint_pubkey.to_string()) {
                Ok(s) => s,
                Err(_) => return -5,
            };
            
            // Copy the signature to the output buffer
            let signature_bytes = signature_cstr.as_bytes_with_nul();
            if signature_bytes.len() > signature_len as usize {
                return -6;
            }
            unsafe { ptr::copy_nonoverlapping(
                signature_bytes.as_ptr(),
                signature_out as *mut u8,
                signature_bytes.len(),
            ) };
            
            // Copy the mint address to the output buffer
            let mint_address_bytes = mint_address_cstr.as_bytes_with_nul();
            if mint_address_bytes.len() > mint_address_len as usize {
                return -7;
            }
            unsafe { ptr::copy_nonoverlapping(
                mint_address_bytes.as_ptr(),
                mint_address_out as *mut u8,
                mint_address_bytes.len(),
            ) };
            
            0 // Success
        },
        Err(_) => -8, // Error creating token
    }
}

/// Free a string allocated by the Rust library
/// 
/// # Safety
/// 
/// This function is unsafe because it works with raw pointers.
/// The pointer must have been allocated by this library.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        let _ = unsafe { CString::from_raw(ptr) };
    }
}

