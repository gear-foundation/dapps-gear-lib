use crate::non_fungible_token::token::*;
use gstd::{exec, msg, prelude::*, ActorId};
use sp_core::{
    sr25519::{Pair as Sr25519Pair, Public, Signature},
    Pair,
};

#[derive(Debug, Encode, Decode, TypeInfo)]
pub struct DelegatedApproveMessage {
    pub token_owner_id: ActorId,
    pub approved_actor_id: ActorId,
    pub nft_program_id: ActorId,
    pub token_id: TokenId,
    pub expiration_timestamp: u64,
}

impl DelegatedApproveMessage {
    pub(crate) fn validate(&self, signed_approve: &Signature, true_token_owner: &ActorId) {
        if msg::source() != self.approved_actor_id {
            panic!("Source is wrong, msg::source must be equal to approved_actor_id")
        }

        if exec::program_id() != self.nft_program_id {
            panic!("You have tried to use delegated_approve with wrong program")
        }

        if self.approved_actor_id == ActorId::zero() {
            panic!("NonFungibleToken: Zero address");
        }

        if true_token_owner != &self.token_owner_id {
            panic!("This user doesn't own the token")
        }

        if exec::block_timestamp() >= self.expiration_timestamp {
            panic!("Delegated approve has expired")
        }

        let massage_bytes = unsafe { self.bytes_ref() };
        let owner = Public(self.token_owner_id.into());
        if !Sr25519Pair::verify(signed_approve, massage_bytes, &owner) {
            panic!("Failed sign verification")
        }
    }

    unsafe fn bytes_ref(&self) -> &[u8] {
        slice::from_raw_parts((self as *const Self) as *const u8, mem::size_of::<Self>())
    }
}
