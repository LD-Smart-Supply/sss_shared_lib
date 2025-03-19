use std::str::FromStr;

use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use sss_shared::fetch_digital_assets_by_owner;

fn main() {
    // Use a specific wallet address
    let owner = Pubkey::from_str("DZ2DqjMAUxxn3jrC71cPbi67JUHC2BcqrT7FRJ2VNGRM")
        .expect("Invalid wallet address");
    println!("Fetching digital assets for wallet: {}", owner);

    // Fetch the digital assets for the owner
    match fetch_digital_assets_by_owner(owner) {
        Ok(assets) => {
            println!("\nFound {} digital assets:", assets.len());
            for asset in assets {
                println!("Asset ID: {}", asset.id);
                println!("Content: {:?}", asset.content);
                println!("Metadata: {:?}\n", asset.metadata);
            }
        }
        Err(e) => println!("Error fetching assets: {:?}", e),
    }
}
