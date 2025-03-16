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
    
    if (result == 0) {
        printf("✅ Token created successfully!\n");
        printf("Transaction signature: %s\n", signature);
        printf("Mint address: %s\n", mint_address);
        printf("View on Solana Explorer: https://explorer.solana.com/address/%s?cluster=devnet\n", mint_address);
    } else {
        printf("❌ Error creating token: %d\n", result);
    }
    
    return 0;
}