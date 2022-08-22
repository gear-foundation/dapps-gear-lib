use crate::non_fungible_token::{royalties::*, token::*};
use codec::{Decode, Encode};
use gstd::{prelude::*, ActorId};
use scale_info::TypeInfo;

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum NFTEvent {
    Minted {
        token_id: TokenId,
        owner: ActorId,
        token_metadata: Option<TokenMetadata>,
    },
    Burnt {
        token_id: TokenId,
    },
    Transfer {
        from: ActorId,
        to: ActorId,
        token_id: TokenId,
    },
    TransferPayout {
        from: ActorId,
        to: ActorId,
        token_id: TokenId,
        payouts: Payout,
    }, 
    Approval {
        owner: ActorId,
        approved_account: ActorId,
        token_id: TokenId,
    },
}
