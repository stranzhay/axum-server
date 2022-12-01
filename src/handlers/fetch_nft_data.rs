use std::str::FromStr;

use crate::state::{
    Collection, CollectionDetails, Creator, Data, Key, Metadata as MetadataWrapper, TokenStandard,
    UseMethod, Uses,
};
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

                            let creators = match value.data.creators {
                                Some(c) => Some(
                                    c.into_iter()
                                        .map(|x| Creator {
                                            address: bs58::encode(x.address).into_string(),
                                            verified: x.verified,
                                            share: x.share,
                                        })
                                        .collect(),
                                ),
                                None => None,
                            };

                            Ok(Json(FetchAccountResponse {
                                network: network.to_string(),
                                metadata: MetadataWrapper {
                                    // the axum server did not play nice with mpl_token_metadata state, so thats why there are some seemingly
                                    // redundant data conversions
                                    key: match value.key{
                                        mpl_token_metadata::state::Key::Uninitialized => Key::Uninitialized ,
                                        mpl_token_metadata::state::Key::EditionV1 => Key::EditionV1,
                                        mpl_token_metadata::state::Key::MasterEditionV1 => Key::MasterEditionV1,
                                        mpl_token_metadata::state::Key::ReservationListV1 => Key::ReservationListV1,
                                        mpl_token_metadata::state::Key::MetadataV1 => Key::MetadataV1,
                                        mpl_token_metadata::state::Key::ReservationListV2 => Key::ReservationListV2,
                                        mpl_token_metadata::state::Key::MasterEditionV2 => Key::MasterEditionV2,
                                        mpl_token_metadata::state::Key::EditionMarker => Key::EditionMarker,
                                        mpl_token_metadata::state::Key::UseAuthorityRecord => Key::UseAuthorityRecord,
                                        mpl_token_metadata::state::Key::CollectionAuthorityRecord => Key::CollectionAuthorityRecord,
                                    },
                                    update_authority: bs58::encode(value.update_authority)
                                        .into_string(),
                                    mint: bs58::encode(value.mint).into_string(),
                                    primary_sale_happened: value.primary_sale_happened,
                                    is_mutable: value.is_mutable,
                                    edition_nonce: value.edition_nonce,
                                    token_standard: match value.token_standard {
                                        Some(x) => Some(match x {
                                            mpl_token_metadata::state::TokenStandard::NonFungible =>TokenStandard::NonFungible ,
                                            mpl_token_metadata::state::TokenStandard::FungibleAsset => TokenStandard::FungibleAsset,
                                            mpl_token_metadata::state::TokenStandard::Fungible => TokenStandard::Fungible,
                                            mpl_token_metadata::state::TokenStandard::NonFungibleEdition => TokenStandard::NonFungibleEdition,
                                        }),
                                        None => None,
                                    },
                                    collection: match value.collection {
                                        Some(x) => Some(Collection { verified: x.verified, key: bs58::encode(x.key).into_string() }),
                                        None => None,
                                    },
                                    uses: match value.uses{
                                        Some(x) => Some(Uses { use_method: match x.use_method{
                                            mpl_token_metadata::state::UseMethod::Burn => UseMethod::Burn,
                                            mpl_token_metadata::state::UseMethod::Multiple => UseMethod::Multiple,
                                            mpl_token_metadata::state::UseMethod::Single => UseMethod::Single,
                                        }, remaining: x.remaining, total: x.total }),
                                        None => None,
                                    },
                                    collection_details: match value.collection_details{
                                        Some(x) => Some(match x{
                                            mpl_token_metadata::state::CollectionDetails::V1 { size } => CollectionDetails::V1 { size },
                                        }),
                                        None => None,
                                    },
                                },
                                token_data: Data {
                                    name: value.data.name.trim().replace('\0', ""),
                                    symbol: value.data.symbol.trim().replace('\0', ""),
                                    uri: value.data.uri.trim().replace('\0', ""),
                                    seller_fee_basis_points: value.data.seller_fee_basis_points,
                                    creators,
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
