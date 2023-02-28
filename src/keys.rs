use std::str::FromStr;

use bitcoin::{Network, secp256k1::{SecretKey, Secp256k1}, util::bip32::ExtendedPrivKey};
use rand::{distributions::Alphanumeric, thread_rng, Rng, SeedableRng};

pub struct BitcoinKeys {
    pub master_key: String,
    pub network: u32
}

impl BitcoinKeys {
    pub fn new(seed: Option<String>) -> Self {
        let network = Network::Testnet;

        let secret_key = match seed {
            Some(v) => {
                SecretKey::from_str(&v).unwrap()
            },
            None => {
                let v = Secp256k1::new();
                let (secret_key, _) = v.generate_keypair(&mut thread_rng());
                secret_key
            }
        };

        let master_key = ExtendedPrivKey::new_master(network, &secret_key.secret_bytes()).unwrap().to_string();

        println!("your seed is: {}", secret_key.display_secret());

        BitcoinKeys {
            master_key,
            network: network.magic()
        }
    }
}