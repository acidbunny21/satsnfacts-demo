use std::str::FromStr;

use bdk_esplora::esplora_client;
use bdk_wallet::SignOptions;
use bitcoin::{Address, Amount, FeeRate};

use utils::{open_wallet, ESPLORA_URL};

mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let mut wallet = open_wallet();

    // let mut tx_builder = wallet.build_tx();
    // tx_builder.add_recipient(
    //     Address::from_str(
    //         // faucet address to send back coins
    //         "tb1qd28npep0s8frcm3y7dxqajkcy2m40eysplyr9v",
    //     )
    //     .unwrap()
    //     .assume_checked()
    //     .script_pubkey(),
    //     Amount::from_sat(10_000)
    // );
    // tx_builder.fee_rate(FeeRate::from_sat_per_vb(10).unwrap());

    // let psbt = tx_builder.finish().unwrap();

    // println!("our PSBT: \n{}", psbt.to_string());

    Ok(())
}
