# SSS Shared Library

A production-grade Rust library for interacting with Solana blockchain, specifically focused on token operations with FFI bindings for C integration.

## Features

- Create fungible tokens on Solana blockchain
- Mint additional tokens to existing fungible tokens
- FFI bindings for C/C++ integration
- Secure key management using BIP39 mnemonics
- Built-in RPC client configuration
- Comprehensive error handling with custom error types

## Prerequisites

- Rust 2025 edition or higher
- Solana CLI tools
- C compiler (for FFI usage)

## Installation

1. Add the library to your Rust project:

```toml
[dependencies]
sss_shared = "0.1.0"
```

2. For C/C++ projects, include the header file and link against the compiled library.

## Configuration

Create a `.env` file in your project root with the following variables:

```env
SOLANA_RPC_URL=https://api.devnet.solana.com  # or your preferred RPC endpoint
PAYER_MNEMONIC="your twelve word mnemonic phrase here"
```

## Usage

### Rust

```rust
use sss_shared::{create_new_token, mint_token, SssResult};

// Create a new token
let (signature, mint_pubkey): SssResult<(String, Pubkey)> = create_new_token(
    "https://example.com/token.json".to_string(),
    "My Token".to_string(),
    9
)?;

// Mint additional tokens
let signature: SssResult<String> = mint_token(mint_pubkey, None, 1000000000)?;
```

### C/C++

```c
#include "sss_shared.h"

char signature[100];
char mint_address[50];

// Create a new token
int result = create_token(
    "https://example.com/token.json",
    "My Token",
    9,
    signature,
    mint_address,
    sizeof(signature),
    sizeof(mint_address)
);

// Mint additional tokens
result = mint_token_ffi(
    mint_address,
    NULL,  // Use payer as token owner
    1000000000,
    signature,
    sizeof(signature)
);
```

## Error Handling

### Rust API

The library uses a custom `SssResult<T>` type for error handling in the Rust API, which is a type alias for `Result<T, SssError>`. The `SssError` enum provides detailed error categories:

- `ConfigError`: Environment configuration issues
- `KeypairError`: Problems with keypair operations
- `RpcError`: Solana RPC client errors
- `TokenError`: Token creation or minting errors
- `FfiError`: Foreign function interface errors

### C API

For the C API, error codes are returned as integers:

- 0: Success
- -1: Null pointer error
- -2: Invalid mint address or string conversion error
- -3: Invalid string conversion or token owner address
- -4: Buffer size error or token operation error
- -5: Token operation error
- -6: Buffer creation error
- -7: Buffer size error
- -8: Token operation error

## Security Considerations

- Never hardcode mnemonic phrases in your code
- Use environment variables or secure key management solutions
- Keep your RPC endpoint URL secure
- Regularly update dependencies for security patches

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Changelog

### [0.1.0] - 2023-03-17

- Initial release
- Basic token creation and minting functionality
- FFI bindings for C integration
- Custom error handling system
