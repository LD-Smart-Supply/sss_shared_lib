//! Utility functions for FFI operations

use crate::error::{SssError, SssResult};
use solana_sdk::pubkey::Pubkey;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::ptr;
use std::str::FromStr;

/// Safely converts a C string pointer to a Rust String
///
/// # Safety
///
/// The pointer must be a valid, null-terminated C string
pub unsafe fn c_str_to_string(ptr: *const c_char) -> SssResult<String> {
    if ptr.is_null() {
        return Err(SssError::FfiError("Null pointer provided".to_string()));
    }

    unsafe { CStr::from_ptr(ptr)
        .to_str()
        .map(|s| s.to_string())
        .map_err(|e| SssError::FfiError(format!("Invalid UTF-8 string: {}", e))) }
}

/// Safely converts a C string pointer to a Solana Pubkey
///
/// # Safety
///
/// The pointer must be a valid, null-terminated C string containing a valid Solana public key
pub unsafe fn c_str_to_pubkey(ptr: *const c_char) -> SssResult<Pubkey> {
    let key_str = unsafe { c_str_to_string(ptr) }?;
    Pubkey::from_str(&key_str).map_err(|e| SssError::FfiError(format!("Invalid public key: {}", e)))
}

/// Safely converts a C string pointer to an optional Solana Pubkey
///
/// # Safety
///
/// If not null, the pointer must be a valid, null-terminated C string containing a valid Solana public key
pub unsafe fn c_str_to_optional_pubkey(ptr: *const c_char) -> SssResult<Option<Pubkey>> {
    if ptr.is_null() {
        return Ok(None);
    }
    unsafe { c_str_to_pubkey(ptr).map(Some) }
}

/// Copies a Rust string to a C buffer
///
/// # Safety
///
/// The buffer must be large enough to hold the string plus null terminator
pub unsafe fn copy_string_to_buffer(
    string: &str,
    buffer: *mut c_char,
    buffer_len: c_int,
) -> SssResult<()> {
    let c_string = CString::new(string)
        .map_err(|e| SssError::FfiError(format!("Failed to create C string: {}", e)))?;

    let bytes = c_string.as_bytes_with_nul();
    if bytes.len() > buffer_len as usize {
        return Err(SssError::FfiError(format!(
            "Buffer too small: need {} bytes, have {}",
            bytes.len(),
            buffer_len
        )));
    }

    unsafe {
        ptr::copy_nonoverlapping(bytes.as_ptr(), buffer as *mut u8, bytes.len());
    }

    Ok(())
}
