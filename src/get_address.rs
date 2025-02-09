use utils::{open_wallet, wallet_db};

mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut wallet = open_wallet();

    let address = wallet.reveal_next_address(bdk_wallet::KeychainKind::External);

    // Step 3: Print the first address
    println!("our next address: {:?}", address);

    wallet.persist(&mut wallet_db()).unwrap();

    Ok(())
}
