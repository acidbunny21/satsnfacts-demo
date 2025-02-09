use std::{fs::File, io::Write, path::Path};

use bdk_wallet::{template::{P2Wpkh, P2TR}, Wallet};
use bitcoin::{key::Secp256k1, Network, PrivateKey, PublicKey};
use utils::wallet_db;

mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let private_key = PrivateKey::generate(Network::Signet);
    let path = Path::new("key.txt");
    if path.exists() {
        panic!("Key already created");
    }

    let mut file = File::create("key.txt")?;
    file.write_all(private_key.to_string().as_bytes())?;

    let secp = Secp256k1::new();
    let public_key = PublicKey::from_private_key(&secp, &private_key);
    println!("Public key: {}", public_key);

    let descriptor = P2TR(private_key);

    Wallet::create_single(descriptor)
        .network(Network::Signet)
        .create_wallet(&mut wallet_db())
        .unwrap();

    Ok(())
}
