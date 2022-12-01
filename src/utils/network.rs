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
        // let secret = if let Some(secret) = secret_store.get("MAINNET_URL") {
        //     secret
        // } else {
        //     return Err(anyhow!("secret was not found").into());
        // };

        // let mainnet_url = env::var("MAINNET_URL").expect("MAINNET_URL must be set");
        match self {
            Self::Mainnet => "".to_string(),
            Self::Testnet => "https://api.testnet.solana.com".to_string(),
            Self::Devnet => "https://api.devnet.solana.com".to_string(),
            Self::Localnet => "http://localhost:8899".to_string(),
        }
    }
}
