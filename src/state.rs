use mpl_token_metadata::state::{CollectionDetails, Data, Key, TokenStandard, Uses};

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use serde::{Deserialize, Serialize};
use serde_json::Value;

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

#[derive(Serialize, Deserialize, Clone)]
pub struct Collection {
    pub verified: bool,
    pub key: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MetadataWrapper {
    pub key: Key,
    pub update_authority: String,
    pub mint: String,
    pub data: Data,
    pub primary_sale_happened: bool,
    pub is_mutable: bool,
    pub edition_nonce: Option<u8>,
    pub token_standard: Option<TokenStandard>,
    pub collection: Option<Collection>,
    pub uses: Option<Uses>,
    pub collection_details: Option<CollectionDetails>,
}

pub enum FetchError {
    FailedToGetAccountData,
    FailedToDeserializeData,
    FailedToFetchUriData,
}

impl IntoResponse for FetchError {
    fn into_response(self) -> Response {
        let body = match self {
            FetchError::FailedToGetAccountData => "Failed to get mint account data.",
            FetchError::FailedToDeserializeData => "Failed to deserialize mint account data.",
            FetchError::FailedToFetchUriData => "Failed to retrieve uri data.",
        };

        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}

#[derive(Serialize)]
pub struct FetchAccountResponse {
    pub network: String,
    pub metadata: MetadataWrapper,
    pub uri_data: UriData,
}
