#include <stdio.h>
#include <stdlib.h>
#include "../include/sss_shared.h"

int main() {
    const char* uri = "https://example.com/token-metadata.json";
    const char* name = "Test Token from C";
    unsigned char decimals = 6;
    
    // Buffers to receive the output
    char signature[100];
    char mint_address[50];
    
    printf("Creating token: %s\n", name);
    
    int result = create_token(
        uri, 
        name, 
        decimals, 
        signature, 
        mint_address, 
        sizeof(signature), 
        sizeof(mint_address)
    );
    
    if (result != 0) {
        printf("❌ Error creating token: %d\n", result);
        return 1;
    }

    printf("✅ Token created successfully!\n");
    printf("Transaction signature: %s\n", signature);
    printf("Mint address: %s\n", mint_address);
    printf("View on Solana Explorer: https://explorer.solana.com/address/%s?cluster=devnet\n", mint_address);

    // Now mint some tokens
    printf("\nMinting tokens...\n");
    
    char mint_signature[100];
    result = mint_token_ffi(
        mint_address,    // mint address
        NULL,           // token owner (NULL means use payer)
        1000000,        // amount (1 token with 6 decimals)
        mint_signature,
        sizeof(mint_signature)
    );

    if (result == 0) {
        printf("✅ Tokens minted successfully!\n");
        printf("Mint transaction signature: %s\n", mint_signature);
        printf("View mint transaction: https://explorer.solana.com/tx/%s?cluster=devnet\n", mint_signature);
    } else {
        printf("❌ Error minting tokens: %d\n", result);
    }
    
    return 0;
}