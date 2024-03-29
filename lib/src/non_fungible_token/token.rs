use gstd::{prelude::*, ActorId};
use primitive_types::U256;

pub type TokenId = U256;

#[derive(Default, Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
pub struct Token {
    pub id: TokenId,
    pub owner_id: ActorId,
    pub name: String,
    pub description: String,
    pub media: String,
    pub reference: String,
    pub approved_account_ids: BTreeSet<ActorId>,
}

#[derive(Default, Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
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
