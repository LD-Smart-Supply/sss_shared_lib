//! FFI functions for C interoperability

use crate::ffi_utils::{
    c_str_to_optional_pubkey, c_str_to_pubkey, c_str_to_string, copy_string_to_buffer,
};
use crate::token::{create_new_token, mint_token};
use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_uchar};

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
/// @param signature_len The length of the signature_out buffer
/// @param mint_address_len The length of the mint_address_out buffer
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
    if uri_ptr.is_null()
        || name_ptr.is_null()
        || signature_out.is_null()
        || mint_address_out.is_null()
    {
        return -1;
    }

    // Convert C strings to Rust strings
    let uri = match unsafe { c_str_to_string(uri_ptr) } {
        Ok(s) => s,
        Err(_) => return -2,
    };

    let name = match unsafe { c_str_to_string(name_ptr) } {
        Ok(s) => s,
        Err(_) => return -3,
    };

    // Call the Rust function
    match create_new_token(uri, name, decimals) {
        Ok((signature, mint_pubkey)) => {
            // Copy the signature to the output buffer
            if unsafe { copy_string_to_buffer(&signature, signature_out, signature_len).is_err() } {
                return -6;
            }

            // Copy the mint address to the output buffer
            if unsafe { copy_string_to_buffer(&mint_pubkey.to_string(), mint_address_out, mint_address_len) }.is_err()
            {
                return -7;
            }

            0 // Success
        }
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

/// FFI function to mint tokens for an existing token
///
/// # Safety
///
/// This function is unsafe because it works with raw pointers for C interoperability.
/// The caller must ensure that:
/// - mint_str is a valid, null-terminated C string containing a valid Solana public key
/// - token_owner_str is either null or a valid, null-terminated C string containing a valid Solana public key
/// - signature_out is a valid pointer to a buffer of sufficient size (signature_len)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mint_token_ffi(
    mint_str: *const c_char,
    token_owner_str: *const c_char,
    amount: u64,
    signature_out: *mut c_char,
    signature_len: c_int,
) -> c_int {
    // Check for null pointers
    if mint_str.is_null() || signature_out.is_null() {
        return -1;
    }

    // Convert mint address string to Pubkey
    let mint = match unsafe { c_str_to_pubkey(mint_str) } {
        Ok(p) => p,
        Err(_) => return -2,
    };

    // Convert token owner string to Pubkey if provided
    let token_owner = match unsafe { c_str_to_optional_pubkey(token_owner_str) } {
        Ok(opt) => opt,
        Err(_) => return -3,
    };

    // Call the Rust function
    match mint_token(mint, token_owner, amount) {
        Ok(signature) => {
            // Copy the signature to the output buffer
            if unsafe { copy_string_to_buffer(&signature, signature_out, signature_len).is_err() } {
                return -4;
            }

            0 // Success
        }
        Err(_) => -5, // Error minting token
    }
}
