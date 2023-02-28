use bitcoin::{util::bip32::ExtendedPrivKey, Network, Address};
use crate::keys::BitcoinKeys;
use std::str::FromStr;
use bdk::{
    Wallet, 
    KeychainKind, 
    template::Bip84, 
    database::MemoryDatabase, 
    electrum_client::Client,
    blockchain::{ElectrumBlockchain, Blockchain}, SyncOptions, FeeRate, SignOptions, wallet::AddressIndex, 
};

pub struct WalletContext {
    wallet_state: Wallet<MemoryDatabase>,
    blockchain: ElectrumBlockchain
}

impl WalletContext {
    pub fn new(seed: Option<String>) -> Self {
        let key = BitcoinKeys::new(seed.to_owned());

        let master_key = ExtendedPrivKey::from_str(&key.master_key).unwrap();

        let network = Network::from_magic(key.network).unwrap();

        let descriptor = Bip84(master_key, KeychainKind::External);

        let wallet_state = Wallet::new(
            descriptor,
            None,
            network,
            MemoryDatabase::default()
        ).unwrap();

        let blockchain = ElectrumBlockchain::from(
            Client::new("ssl://electrum.blockstream.info:60002").unwrap()
        );

        WalletContext {
            blockchain,
            wallet_state
        }
    }

    pub fn get_balance(&self) {
        let _ = &self.wallet_state.sync(&self.blockchain, SyncOptions::default()).unwrap();

        let receive_address = self.wallet_state.get_address(AddressIndex::LastUnused).unwrap();
        
        let balance = self.wallet_state.get_balance().unwrap();

        println!("bitcoin address is {}", receive_address.address);
        println!("balance is {}", balance);
    }

    pub fn send_coins(&self, send_address: &str, stats: u64) {
        let _ = &self.wallet_state.sync(&self.blockchain, SyncOptions::default()).unwrap();

        let address = Address::from_str(send_address).unwrap();

        let mut builder = self.wallet_state.build_tx();

        builder
            .drain_wallet()
            .fee_rate(FeeRate::from_sat_per_vb(2.0))
            .drain_to(address.script_pubkey());
            /* .add_recipient(address.script_pubkey(), stats); */

        let (mut psbt, _) = builder.finish().unwrap();

        let is_valid = &self.wallet_state.sign(&mut psbt , SignOptions::default()).unwrap();

        println!("Is transaction valid: {}", is_valid);

        let tx = psbt.clone().extract_tx();

        println!("Transaction ID: {}", &tx.txid());

        self.blockchain.broadcast(&tx).unwrap();

        println!("broadcasted successfully");
    }
} 