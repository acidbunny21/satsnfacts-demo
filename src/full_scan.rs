use bdk_chain::{keychain_txout::FullScanRequestBuilderExt, spk_client::FullScanRequest};
use bdk_esplora::{esplora_client, EsploraExt};

use utils::{open_wallet, wallet_db, ESPLORA_URL, PARALLEL_REQUESTS, STOP_GAP};

mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let mut wallet = open_wallet();
    // let client = esplora_client::Builder::new(ESPLORA_URL).build_blocking();

    // let request = wallet.start_full_scan();
    // let update = client.full_scan(request, STOP_GAP, PARALLEL_REQUESTS).unwrap();
    // wallet.apply_update(update).unwrap();

    // wallet.persist(&mut wallet_db()).unwrap();

    // println!("Our balance: {}", wallet.balance());
    // println!("Our utxos: \n{:?}", wallet.list_unspent().collect::<Vec<_>>());

    // println!("Scanned blockchain, wallet is up to date.");

    Ok(())
}
