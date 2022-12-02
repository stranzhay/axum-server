use std::str::FromStr;

use mpl_token_metadata::state::Data;
use tracing::Level;

use crate::state::{Collection, FetchAccountResponse, FetchError, MetadataWrapper, UriData};
use crate::utils::Network;
use axum::Json;
use axum::{extract::Path, http::StatusCode};

use mpl_token_metadata::{
    pda::find_metadata_account,
    state::{Metadata, TokenMetadataAccount},
};

use solana_client::nonblocking::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;

pub async fn fetch_nft_handler(
    Path((id, network)): Path<(String, String)>,
) -> Result<Json<FetchAccountResponse>, FetchError> {
    let pubkey = Pubkey::from_str(&id).unwrap();

    let rpc_client = RpcClient::new(Network::from_str(&network).unwrap().get_network_url());

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
        Ok(account_data) => MetadataWrapper {
            key: account_data.key,
            update_authority: account_data.update_authority.to_string(),
            mint: account_data.mint.to_string(),
            data: Data {
                name: account_data
                    .data
                    .name
                    .trim_matches(char::from(0))
                    .to_string(),
                symbol: account_data
                    .data
                    .symbol
                    .trim_matches(char::from(0))
                    .to_string(),
                uri: account_data
                    .data
                    .uri
                    .trim_matches(char::from(0))
                    .to_string(),
                seller_fee_basis_points: account_data.data.seller_fee_basis_points,
                creators: account_data.data.creators,
            },
            primary_sale_happened: account_data.primary_sale_happened,
            is_mutable: account_data.is_mutable,
            edition_nonce: account_data.edition_nonce,
            token_standard: account_data.token_standard,
            collection: match account_data.collection {
                Some(collection) => Some(Collection {
                    verified: collection.verified,
                    key: collection.key.to_string(),
                }),
                None => None,
            },
            uses: account_data.uses,
            collection_details: account_data.collection_details,
        },
        Err(_) => {
            tracing::event!(Level::ERROR, "NFT metadata account fetch to deserialize.");
            return Err(FetchError::FailedToDeserializeData);
        }
    };

    tracing::event!(
        Level::INFO,
        "NFT metadata account deserialized successfully."
    );

    let uri_data = match reqwest::get(deser_metadata.clone().data.uri).await {
        Ok(uri_data) => match uri_data.status() {
            StatusCode::OK => {
                let uri_data: UriData = uri_data.json().await.unwrap();
                uri_data
            }
            _s => {
                tracing::event!(Level::ERROR, "Could not retrieve uri metadata.");
                return Err(FetchError::FailedToFetchUriData);
            }
        },
        Err(_) => {
            tracing::event!(Level::ERROR, "Error on fetching metadata from uri request.");
            return Err(FetchError::FailedToFetchUriData);
        }
    };

    Ok(Json(FetchAccountResponse {
        network: network.to_string(),
        metadata: deser_metadata,
        uri_data,
    }))
}
