#![allow(unused)]

mod client_wallet;
mod keys;
use dotenv::dotenv;
use client_wallet::WalletContext;

fn main() {
    dotenv().ok();

    let seed = std::env::var("SEED").expect("SEED must be set");

    let wallet = WalletContext::new(Some(String::from(seed)));

    wallet.get_balance();

    let to = "tb1q4280xax2lt0u5a5s9hd4easuvzalm8v9ege9ge";

    wallet.send_coins(to, 100);
}