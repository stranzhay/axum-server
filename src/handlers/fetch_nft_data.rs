use std::str::FromStr;

use crate::state::{Data, Metadata as MetadataWrapper};
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
use solana_sdk::bs58;
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
    pub metadata: MetadataWrapper,
    pub token_data: Data,
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
                                metadata: MetadataWrapper {
                                    // key: value.key,
                                    update_authority: bs58::encode(value.update_authority)
                                        .into_string(),
                                    mint: bs58::encode(value.mint).into_string(),
                                    primary_sale_happened: value.primary_sale_happened,
                                    is_mutable: value.is_mutable,
                                    edition_nonce: value.edition_nonce,
                                    // token_standard: value.token_standard,
                                    // collection: value.collection,
                                    // uses: value.uses,
                                    // collection_details: value.collection_details,
                                },
                                token_data: Data {
                                    name: value.data.name.trim().replace('\0', ""),
                                    symbol: value.data.symbol.trim().replace('\0', ""),
                                    uri: value.data.uri.trim().replace('\0', ""),
                                    seller_fee_basis_points: value.data.seller_fee_basis_points,
                                    // creators: value.data.creators,
                                },
                                uri_data: UriData {
                                    name: uri_data.name,
                                    symbol: uri_data.symbol,
                                    description: uri_data.description,
                                    seller_fee_basis_points: uri_data.seller_fee_basis_points,
                                    image: uri_data.image,
                                    external_url: uri_data.external_url,
                                    attributes: uri_data.attributes,
                                    properties: uri_data.properties,
                                },
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
