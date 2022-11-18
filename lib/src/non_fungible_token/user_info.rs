use gstd::{ActorId, Decode, Encode, TypeInfo};

#[derive(Debug, Default, Decode, Encode, TypeInfo)]
pub struct UserInfo {
    pub address: ActorId, // address of user role
    pub expires: u64,     // unix timestamp
}
