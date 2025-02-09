use std::{fs::File, io::Read, str::FromStr};

use bdk_chain::rusqlite;
use bdk_wallet::{
    template::P2TR,
    PersistedWallet, Wallet,
};
use bitcoin::PrivateKey;

pub const ESPLORA_URL: &str = "https://mutinynet.com/api";
pub const PARALLEL_REQUESTS: usize = 5;
pub const STOP_GAP: usize = 5;
pub const DB_FILE: &str = "wallet.db";

pub fn wallet_db() -> rusqlite::Connection {
    rusqlite::Connection::open(DB_FILE).unwrap()
}

pub fn open_wallet() -> PersistedWallet<rusqlite::Connection> {
    println!("opening wallet");
    let mut private_key_str = String::new();
    File::open("key.txt")
        .unwrap()
        .read_to_string(&mut private_key_str)
        .unwrap();

    let prv = PrivateKey::from_str(&private_key_str).unwrap();
    let descriptor = P2TR(prv);

    let mut db = wallet_db();
    Wallet::load()
        .descriptor(bdk_wallet::KeychainKind::External, Some(descriptor))
        .check_network(bitcoin::Network::Signet)
        .extract_keys()
        .load_wallet(&mut db)
        .unwrap()
        .unwrap()
}
