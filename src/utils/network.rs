use std::{env, str::FromStr, string::ParseError};

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Network {
    Mainnet,
    Testnet,
    Devnet,
}

impl Network {
    pub fn get_network_url(self) -> String {
        let mainnet_url = env::var("MAINNET_URL").expect("MAINNET_URL must be set");
        match self {
            Self::Mainnet => mainnet_url.to_string(),
            Self::Testnet => "https://api.testnet.solana.com".to_string(),
            Self::Devnet => "https://api.devnet.solana.com".to_string(),
        }
    }
}

impl FromStr for Network {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let network = match s {
            "mainnet" => Network::Mainnet,
            "testnet" => Network::Testnet,
            "devnet" => Network::Devnet,
            &_ => Network::Mainnet,
        };

        Ok(network)
    }

    type Err = ParseError;
}
