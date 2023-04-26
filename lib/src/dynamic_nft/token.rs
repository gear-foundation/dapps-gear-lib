use gstd::{prelude::*, ActorId};
use primitive_types::U256;

pub type TokenId = U256;

#[derive(Debug, Default, Decode, Encode, TypeInfo, PartialEq, Eq)]
pub struct Token {
    pub id: TokenId,
    pub owner_id: ActorId,
    pub name: String,
    pub description: String,
    pub media: String,
    pub reference: String,
    pub approved_account_ids: BTreeSet<ActorId>,
}

#[derive(Debug, Default, Encode, Decode, Clone, TypeInfo, PartialEq, Eq, PartialOrd, Ord)]
pub struct TokenMetadata {
    // ex. "CryptoKitty #100"
    pub name: String,
    // free-form description
    pub description: String,
    // URL to associated media, preferably to decentralized, content-addressed storage
    pub media: String,
    // URL to an off-chain JSON file with more info.
    pub reference: String,
}

#[derive(Debug, Default, Decode, Encode, TypeInfo, PartialEq, Eq)]
pub struct DynamicToken<T: Encode + Decode + TypeInfo> {
    pub id: TokenId,
    pub owner_id: ActorId,
    pub name: String,
    pub description: String,
    pub media: String,
    pub reference: String,
    pub approved_account_ids: BTreeSet<ActorId>,
    pub dynamic_data: T,
}

#[derive(Debug, Default, Encode, Decode, Clone, TypeInfo, PartialEq, Eq, PartialOrd, Ord)]
pub struct DynamicTokenMetadata<T: Encode + Decode + TypeInfo> {
    // ex. "CryptoKitty #100"
    pub name: String,
    // free-form description
    pub description: String,
    // URL to associated media, preferably to decentralized, content-addressed storage
    pub media: String,
    // URL to an off-chain JSON file with more info.
    pub reference: String,
    // Dynamic (changeable) data
    pub dynamic_data: T,
}
