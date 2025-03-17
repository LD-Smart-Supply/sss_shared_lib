//! Error types for the SSS Shared library

use std::fmt;

/// Custom error type for the SSS Shared library
#[derive(Debug)]
pub enum SssError {
    /// Error related to environment configuration
    ConfigError(String),
    /// Error related to keypair operations
    KeypairError(String),
    /// Error related to Solana RPC operations
    RpcError(String),
    /// Error related to token operations
    TokenError(String),
    /// Error related to FFI operations
    FfiError(String),
}

impl fmt::Display for SssError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SssError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            SssError::KeypairError(msg) => write!(f, "Keypair error: {}", msg),
            SssError::RpcError(msg) => write!(f, "RPC error: {}", msg),
            SssError::TokenError(msg) => write!(f, "Token error: {}", msg),
            SssError::FfiError(msg) => write!(f, "FFI error: {}", msg),
        }
    }
}

impl std::error::Error for SssError {}

/// Result type for the SSS Shared library
pub type SssResult<T> = Result<T, SssError>;

/// Convert from any error type to SssError
pub trait IntoSssError<T> {
    /// Convert the error to an SssError
    fn into_sss_error(self, context: &str) -> Result<T, SssError>;
}

impl<T, E: std::fmt::Display> IntoSssError<T> for Result<T, E> {
    fn into_sss_error(self, context: &str) -> Result<T, SssError> {
        self.map_err(|e| {
            // Determine the appropriate error type based on the context
            if context.contains("config") || context.contains("env") {
                SssError::ConfigError(format!("{}: {}", context, e))
            } else if context.contains("keypair") || context.contains("signer") {
                SssError::KeypairError(format!("{}: {}", context, e))
            } else if context.contains("rpc") || context.contains("client") {
                SssError::RpcError(format!("{}: {}", context, e))
            } else if context.contains("token") || context.contains("mint") {
                SssError::TokenError(format!("{}: {}", context, e))
            } else {
                SssError::FfiError(format!("{}: {}", context, e))
            }
        })
    }
}
