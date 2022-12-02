use std::str::FromStr;

use tracing::Level;

use crate::utils::Network;
use axum::Json;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use mpl_token_metadata::{
    pda::find_metadata_account,
    state::{Metadata, TokenMetadataAccount},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;

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

pub enum FetchError {
    FailedToGetAccountData,
    FailedToDeserializeData,
}

impl IntoResponse for FetchError {
    fn into_response(self) -> Response {
        let body = match self {
            FetchError::FailedToGetAccountData => "Failed to get mint account data.",
            FetchError::FailedToDeserializeData => "Failed to deserialize mint account data.",
        };

        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}

#[derive(Serialize)]
pub struct FetchAccountResponse {
    pub network: String,
    // pub metadata: Metadata,
    pub uri_data: UriData,
}

pub async fn fetch_nft_handler(
    Path((id, network)): Path<(String, String)>,
) -> Result<Json<FetchAccountResponse>, FetchError> {
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

    let metadata_account = match rpc_client.get_account_data(&pda).await {
        Ok(account_data) => account_data,
        Err(_) => {
            tracing::event!(Level::ERROR, "NFT metadata account fetch failed");
            return Err(FetchError::FailedToGetAccountData);
        }
    };
    tracing::event!(Level::INFO, "NFT metadata account fetch successful");

    let deser_metadata = match Metadata::safe_deserialize(&mut metadata_account.as_slice()) {
        Ok(account_data) => account_data,
        Err(_) => {
            tracing::event!(Level::ERROR, "NFT metadata account fetch to deserialize.");
            return Err(FetchError::FailedToDeserializeData);
        }
    };

    tracing::event!(Level::INFO, "NFT metadata account fetch successful");

    let uri_data = match reqwest::get(deser_metadata.clone().data.uri).await {
        Ok(uri_data) => match uri_data.status() {
            StatusCode::OK => {
                let uri_data: UriData = uri_data.json().await.unwrap();
                uri_data
            }
            _s => {
                tracing::event!(Level::ERROR, "Could not retrieve uri metadata.");
                return Err(FetchError::FailedToDeserializeData);
            }
        },
        Err(_) => {
            tracing::event!(Level::ERROR, "Error on fetching metadata from uri request.");
            return Err(FetchError::FailedToDeserializeData);
        }
    };

    Ok(Json(FetchAccountResponse {
        network: network.to_string(),
        // metadata: value,
        uri_data,
    }))
}
