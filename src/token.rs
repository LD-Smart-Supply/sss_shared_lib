//! Token creation and management functionality

use reqwest::blocking::Client;
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct MetaplexRequestBody {
    jsonrpc: String,
    id: u64,
    method: String,
    params: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DigitalAsset {
    pub id: String,
    pub content: serde_json::Value,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
struct MetaplexResult {
    total: u64,
    limit: u64,
    items: Vec<DigitalAsset>,
}

#[derive(Debug, Serialize, Deserialize)]
struct MetaplexResponse {
    jsonrpc: String,
    result: MetaplexResult,
    id: u64,
}

use crate::error::{IntoSssError, SssResult};
use mpl_token_metadata::instructions::{CreateV1Builder, MintV1Builder};
use mpl_token_metadata::types::TokenStandard;
use solana_sdk::{
    message::Message, pubkey::Pubkey, signature::Keypair, signer::Signer, transaction::Transaction,
};

use crate::RPC_CLIENT;
use crate::get_payer;

/// Creates a fungible token with the specified parameters
///
/// # Arguments
///
/// * `mint` - The keypair for the mint account
/// * `uri` - The URI pointing to the token's metadata
/// * `name` - The name of the token
/// * `decimals` - The number of decimal places for the token
///
/// # Returns
///
/// The transaction signature as a string
pub fn create_consumable_token(
    mint: &Keypair,
    uri: String,
    name: String,
    decimals: u8,
) -> SssResult<String> {
    // Get the payer keypair
    let payer = get_payer().into_sss_error("Failed to get payer keypair")?;

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
    let blockhash = RPC_CLIENT
        .get_latest_blockhash()
        .into_sss_error("Failed to get latest blockhash")?;

    // Create and sign the transaction
    let tx = Transaction::new(&[mint, &payer], message, blockhash);

    // Send and confirm the transaction
    let signature = RPC_CLIENT
        .send_and_confirm_transaction(&tx)
        .into_sss_error("Failed to send and confirm transaction")?;

    Ok(signature.to_string())
}

/// Creates a new token with a newly generated mint keypair
///
/// # Arguments
///
/// * `uri` - The URI pointing to the token's metadata
/// * `name` - The name of the token
/// * `decimals` - The number of decimal places for the token
///
/// # Returns
///
/// A tuple containing the transaction signature and the mint public key
pub fn create_new_token(uri: String, name: String, decimals: u8) -> SssResult<(String, Pubkey)> {
    let mint = Keypair::new();
    let signature = create_consumable_token(&mint, uri, name, decimals)?;
    Ok((signature, mint.pubkey()))
}

/// Mints tokens for an existing token
///
/// # Arguments
///
/// * `mint` - The public key of the token's mint account
/// * `token_owner` - Optional public key of the token owner. If None, the payer will be used
/// * `amount` - The amount of tokens to mint
///
/// # Returns
///
/// The transaction signature as a string
pub fn mint_token(mint: Pubkey, token_owner: Option<Pubkey>, amount: u64) -> SssResult<String> {
    // Get the payer keypair which will also be the mint authority
    let payer = get_payer().into_sss_error("Failed to get payer keypair")?;
    let authority = Keypair::from_bytes(&payer.to_bytes())
        .into_sss_error("Failed to create authority keypair")?;

    // Derive the metadata PDA
    let seeds = &[
        "metadata".as_bytes(),
        &mpl_token_metadata::ID.to_bytes(),
        &mint.to_bytes(),
    ];
    let (metadata, _) = Pubkey::find_program_address(seeds, &mpl_token_metadata::ID);

    // Get token account - if token_owner is provided, use it, otherwise use payer
    let owner = token_owner.unwrap_or(payer.pubkey());
    let token = spl_associated_token_account::get_associated_token_address(&owner, &mint);

    // Create the mint instruction
    let mint_ix = MintV1Builder::new()
        .token(token)
        .token_owner(Some(owner))
        .metadata(metadata)
        .mint(mint)
        .authority(authority.pubkey())
        .payer(payer.pubkey())
        .amount(amount)
        .instruction();

    // Create the message
    let message = Message::new(&[mint_ix], Some(&payer.pubkey()));

    // Get the latest blockhash
    let blockhash = RPC_CLIENT
        .get_latest_blockhash()
        .into_sss_error("Failed to get latest blockhash")?;

    // Create and sign the transaction
    let tx = Transaction::new(&[&authority, &payer], message, blockhash);

    // Send and confirm the transaction
    let signature = RPC_CLIENT
        .send_and_confirm_transaction(&tx)
        .into_sss_error("Failed to send and confirm transaction")?;

    Ok(signature.to_string())
}

/// Fetches digital assets owned by a specific wallet address
///
/// # Arguments
///
/// * `owner` - The public key of the wallet address to fetch assets for
///
/// # Returns
///
/// A vector of DigitalAsset objects representing the assets owned by the wallet
pub fn fetch_digital_assets_by_owner(owner: Pubkey) -> SssResult<Vec<DigitalAsset>> {
    let url = "https://devnet-aura.metaplex.com/31aff70e-1d9a-4b18-b875-17a899d8ba16";
    let client = Client::new();

    let request_body = MetaplexRequestBody {
        jsonrpc: "2.0".to_string(),
        id: 1,
        method: "getAssetsByOwner".to_string(),
        params: serde_json::json!({
            "ownerAddress": owner.to_string(),
            "grouping": ["collection"],
            "sortBy": { "sortBy": "created", "sortDirection": "desc" }
        }),
    };

    let response = client
        .post(url)
        .header(CONTENT_TYPE, "application/json")
        .json(&request_body)
        .send()
        .into_sss_error("Failed to send request to Metaplex API")?;

    let metaplex_response: MetaplexResponse = response
        .json()
        .into_sss_error("Failed to parse Metaplex API response")?;

    Ok(metaplex_response.result.items)
}
