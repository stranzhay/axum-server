use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Key {
    Uninitialized,
    EditionV1,
    MasterEditionV1,
    ReservationListV1,
    MetadataV1,
    ReservationListV2,
    MasterEditionV2,
    EditionMarker,
    UseAuthorityRecord,
    CollectionAuthorityRecord,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub seller_fee_basis_points: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creators: Option<Vec<Creator>>,
}

#[derive(Serialize, Deserialize)]
pub struct DataV2 {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub seller_fee_basis_points: u16,
    pub creators: Option<Vec<Creator>>,
    pub collection: Option<Collection>,
    pub uses: Option<Uses>,
}

#[derive(Serialize, Deserialize)]
pub enum UseMethod {
    Burn,
    Multiple,
    Single,
}

#[derive(Serialize, Deserialize)]
pub enum CollectionDetails {
    V1 { size: u64 },
}

#[derive(Serialize, Deserialize)]
pub struct Uses {
    pub use_method: UseMethod,
    pub remaining: u64,
    pub total: u64,
}

#[derive(Serialize, Deserialize)]
pub enum TokenStandard {
    NonFungible,
    FungibleAsset,
    Fungible,
    NonFungibleEdition,
}

#[derive(Serialize, Deserialize)]
pub struct UseAuthorityRecord {
    pub key: Key,
    pub allowed_uses: u64,
    pub bump: u8,
}

#[derive(Serialize, Deserialize)]
pub struct CollectionAuthorityRecord {
    pub key: Key,
    pub bump: u8,
}

#[derive(Serialize, Deserialize)]
pub struct Collection {
    pub verified: bool,
    pub key: String,
}

#[derive(Serialize, Deserialize)]
pub struct Metadata {
    pub key: Key,
    pub update_authority: String,
    pub mint: String,
    pub primary_sale_happened: bool,
    pub is_mutable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edition_nonce: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_standard: Option<TokenStandard>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection: Option<Collection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uses: Option<Uses>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_details: Option<CollectionDetails>,
}

#[derive(Serialize, Deserialize)]
pub struct MasterEditionV2 {
    pub key: Key,
    pub supply: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_supply: Option<u64>,
}

#[derive(Serialize, Deserialize)]
pub struct MasterEditionV1 {
    pub key: Key,
    pub supply: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_supply: Option<u64>,
    pub printing_mint: String,
    pub one_time_printing_authorization_mint: String,
}

#[derive(Serialize, Deserialize)]

pub struct Edition {
    pub key: Key,
    pub parent: String,
    pub edition: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Creator {
    pub address: String,
    pub verified: bool,
    pub share: u8,
}

#[derive(Serialize, Deserialize)]
pub struct ReservationListV2 {
    pub key: Key,
    pub master_edition: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supply_snapshot: Option<u64>,
    pub reservations: Vec<Reservation>,
    pub total_reservation_spots: u64,
    pub current_reservation_spots: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Reservation {
    pub address: String,
    pub spots_remaining: u64,
    pub total_spots: u64,
}

#[derive(Serialize, Deserialize)]
pub struct ReservationListV1 {
    pub key: Key,
    pub master_edition: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supply_snapshot: Option<u64>,
    pub reservations: Vec<ReservationV1>,
}

#[derive(Serialize, Deserialize)]
pub struct ReservationV1 {
    pub address: String,
    pub spots_remaining: u8,
    pub total_spots: u8,
}

#[derive(Serialize, Deserialize)]
pub struct EditionMarker {
    pub key: Key,
    pub ledger: [u8; 31],
}
