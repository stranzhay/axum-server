use std::str::FromStr;

use crate::utils::Network;
use axum::extract::Path;
use axum::{http::StatusCode, Json};
use mpl_token_metadata::{
    pda::find_metadata_account,
    state::{Metadata, TokenMetadataAccount},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use tracing::Level;

#[derive(Deserialize, Debug, Serialize)]
pub struct UriData {
    pub name: String,
    pub symbol: String,
    pub description: String,
    pub seller_fee_basis_points: u16,
    pub image: String,
    pub external_url: String,
    pub attributes: Vec<Value>,
    pub properties: Value,
}

#[derive(Serialize)]
pub struct FetchAccountResponse {
    pub network: String,
    pub metadata: Metadata,
    pub uri_data: UriData,
}

pub async fn fetch_nft_handler(
    Path((id, network)): Path<(String, String)>,
) -> Result<Json<FetchAccountResponse>, (StatusCode, Json<serde_json::Value>)> {
    let s_slice: &str = &network;
    let network_string = match s_slice {
        "Mainnet" => Network::Mainnet,
        "Testnet" => Network::Mainnet,
        "Devnet" => Network::Mainnet,
        "Localnet" => Network::Mainnet,
        &_ => Network::Mainnet,
    };

    let pubkey = Pubkey::from_str(&id).unwrap();

    let rpc_client = RpcClient::new(network_string.get_network_url());

    let (pda, _bump) = find_metadata_account(&pubkey);
    let metadata_account = rpc_client.get_account_data(&pda).await;

    match metadata_account {
        Ok(value) => {
            tracing::event!(Level::INFO, "NFT metadata account fetch successful");
            let deser_metadata = Metadata::safe_deserialize(&mut value.as_slice());

            match deser_metadata {
                Ok(value) => {
                    tracing::event!(Level::INFO, "Metadata deserialized successfully");

                    let response = reqwest::get(value.clone().data.uri).await.unwrap();

                    match response.status() {
                        StatusCode::OK => {
                            let uri_data: UriData = response.json().await.unwrap();

                            Ok(Json(FetchAccountResponse {
                                network: network.to_string(),
                                metadata: value,
                                uri_data,
                            }))
                        }
                        _s => {
                            tracing::event!(Level::ERROR, "Could not retrieve uri metadata.");
                            Err((
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(Value::String(
                                    "Failed to fetch http data for uri metadata".to_string(),
                                )),
                            ))
                        }
                    }
                }

                Err(_e) => {
                    tracing::event!(Level::ERROR, "Metadata deserialization failed");
                    Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(Value::String("Failed to deserialize Metadata".to_string())),
                    ))
                }
            }
        }

        Err(_e) => {
            tracing::event!(Level::ERROR, "NFT metadata account fetch failed");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Value::String(
                    "Failed to fetch nft metadata account".to_string(),
                )),
            ))
        }
    }
}
