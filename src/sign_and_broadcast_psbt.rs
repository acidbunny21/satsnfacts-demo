use std::str::FromStr;

use bdk_esplora::esplora_client;
use bdk_wallet::SignOptions;
use bitcoin::{consensus::encode::serialize_hex, Psbt};
use utils::{open_wallet, ESPLORA_URL};

mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wallet = open_wallet();

    let mut psbt = Psbt::from_str(
        // TODO: use crafted PSBT here
        ""
    )
    .unwrap();

    let finalized = wallet.sign(&mut psbt, SignOptions::default()).unwrap();
    assert!(finalized, "should be all signed");
    let signed = psbt.extract_tx().unwrap();
    println!("Our signed transaction hex: \n{:?}", serialize_hex(&signed));

    let client = esplora_client::Builder::new(ESPLORA_URL).build_blocking();
    client.broadcast(&signed).unwrap();
    println!("Our tx got broadcasted! Txid: {}", signed.compute_txid());

    Ok(())
}
