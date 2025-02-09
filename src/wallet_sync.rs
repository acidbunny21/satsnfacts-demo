use bdk_esplora::{esplora_client, EsploraExt};

use utils::{open_wallet, ESPLORA_URL, PARALLEL_REQUESTS};

mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut wallet = open_wallet();

    let client = esplora_client::Builder::new(ESPLORA_URL).build_blocking();

    let request = wallet
        .start_sync_with_revealed_spks()
        .outpoints(wallet.list_output().map(|o| o.outpoint))
        .txids(wallet.transactions().filter_map(|t| {
            if t.chain_position.is_confirmed() {
                Some(t.tx_node.compute_txid())
            } else {
                None
            }
        }));

    let update = client.sync(request, PARALLEL_REQUESTS).unwrap();
    wallet.apply_update(update).unwrap();

    println!("Our balance: {}", wallet.balance());

    println!(
        "Our utxos: \n{:?}",
        wallet.list_unspent().collect::<Vec<_>>()
    );

    Ok(())
}
