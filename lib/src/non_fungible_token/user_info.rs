use gstd::ActorId;

#[derive(Debug, Default)]
pub struct UserInfo 
{
    pub address: ActorId, // address of user role
    pub expires: u64, // unix timestamp
}