#ifndef SSS_SHARED_H
#define SSS_SHARED_H

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Creates a new token and returns the transaction signature and mint address
 *
 * @param uri_ptr A pointer to a null-terminated C string containing the token URI
 * @param name_ptr A pointer to a null-terminated C string containing the token name
 * @param decimals The number of decimal places for the token
 * @param signature_out A pointer to a buffer where the transaction signature will be written
 * @param mint_address_out A pointer to a buffer where the mint address will be written
 * @param signature_len The length of the signature_out buffer
 * @param mint_address_len The length of the mint_address_out buffer
 * @return 0 on success, non-zero error code on failure
 */
int create_token(
    const char* uri_ptr,
    const char* name_ptr,
    unsigned char decimals,
    char* signature_out,
    char* mint_address_out,
    int signature_len,
    int mint_address_len
);

/**
 * Frees a string allocated by the Rust library
 *
 * @param ptr The pointer to free
 */
void free_string(char* ptr);

// Function to mint tokens
int mint_token_ffi(
    const char* mint_address,
    const char* token_owner,  // can be NULL to use payer
    uint64_t amount,
    char* signature_out,
    int signature_len
);

#ifdef __cplusplus
}
#endif

#endif /* SSS_SHARED_H */