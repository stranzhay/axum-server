use std::{env, str::FromStr, string::ParseError};

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Network {
    Mainnet,
    Testnet,
    Devnet,
    Localnet,
}

impl Network {
    pub fn get_network_url(self) -> String {
        let mainnet_url = env::var("MAINNET_URL").expect("MAINNET_URL must be set");
        match self {
            Self::Mainnet => mainnet_url.to_string(),
            Self::Testnet => "https://api.testnet.solana.com".to_string(),
            Self::Devnet => "https://api.devnet.solana.com".to_string(),
            Self::Localnet => "http://localhost:8899".to_string(),
        }
    }
}

impl FromStr for Network {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let thang = match s {
            "Mainnet" => Network::Mainnet,
            "Testnet" => Network::Mainnet,
            "Devnet" => Network::Mainnet,
            "Localnet" => Network::Mainnet,
            &_ => Network::Mainnet,
        };

        Ok(thang)
    }

    type Err = ParseError;
}
