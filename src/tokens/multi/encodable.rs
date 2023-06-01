//! The encodable multi token state.
//!
//! Due to limitations of the SCALE codec, it's impossible to encode the
//! [`HashMap`] & [`HashSet`](hashbrown::HashSet) types, and therefore
//! [`super::MTState`] too, so as a workaround there's the encodable [`MTState`]
//! type that use [`Vec`] instead of unencodable types and can be constructed
//! from [`super::MTState`].

use super::{Amount, Id, MTError, MTState as SuperMTState, Operator, Owner};
use gstd::prelude::*;

/// The encodable multi token state.
#[derive(Default, Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
pub struct MTState {
    tokens: Vec<(Id, Token)>,
    owners: Vec<(Owner, GeneralOwnerData)>,
    total_supply: Amount,
}

impl MTState {
    /// Gets the current total token supply.
    ///
    /// - If `id` is [`Some`], returns the supply of the tokens with this `id`.
    /// - If `id` is [`None`], returns the supply of all tokens.
    pub fn total_supply(&self, id: Option<&Id>) -> Amount {
        id.map_or(self.total_supply, |unwrapped_id| {
            self.tokens
                .iter()
                .find_map(|(stored_id, token)| {
                    (stored_id == unwrapped_id).then_some(token.total_supply)
                })
                .unwrap_or_default()
        })
    }

    /// Returns a balance of `owner`'s tokens.
    ///
    /// - If `id` is [`Some`], returns the balance of the tokens with this `id`.
    /// - If `id` is [`None`], returns the balance of all tokens.
    pub fn balance_of(&self, owner: Owner, id: Option<&Id>) -> Amount {
        id.map_or_else(
            || {
                self.owners
                    .iter()
                    .find_map(|(stored_owner, general_owner_data)| {
                        (*stored_owner == owner).then_some(general_owner_data.balance)
                    })
            },
            |unwrapped_id| {
                self.tokens.iter().find_map(|(stored_id, token)| {
                    (stored_id == unwrapped_id).then_some(
                        token
                            .owners
                            .iter()
                            .find_map(|(stored_owner, token_owner_data)| {
                                (*stored_owner == owner).then_some(token_owner_data.balance)
                            })
                            .unwrap_or_default(),
                    )
                })
            },
        )
        .unwrap_or_default()
    }

    /// Returns an allowance of `owner`'s tokens for `operator`.
    ///
    /// - If `id` is [`Some`], firstly checks if `operator` is allowed for all
    /// `owner`'s tokens, and if not, returns an approved amount of the tokens
    /// with this `id`.
    /// - If `operator` is allowed for all `owner`'s tokens, returns
    /// [`Amount::MAX`], otherwise returns 0.
    /// - If `id` is [`None`], only checks if `operator` is allowed for all
    /// `owner`'s tokens.
    pub fn allowance(&self, owner: Owner, operator: Operator, id: Option<&Id>) -> Amount {
        let id_search = || {
            id.and_then(|unwrapped_id| {
                self.tokens.iter().find_map(|(stored_id, token)| {
                    (stored_id == unwrapped_id).then_some(
                        token
                            .owners
                            .iter()
                            .find_map(|(stored_owner, token_owner_data)| {
                                (*stored_owner == owner).then_some(
                                    token_owner_data
                                        .allowances
                                        .iter()
                                        .find_map(|(stored_operator, allowance)| {
                                            (*stored_operator == operator).then_some(*allowance)
                                        })
                                        .unwrap_or_default(),
                                )
                            })
                            .unwrap_or_default(),
                    )
                })
            })
        };

        self.owners
            .iter()
            .find_map(|(stored_owner, general_owner_data)| {
                (*stored_owner == owner).then_some(
                    general_owner_data
                        .operators
                        .contains(&operator)
                        .then_some(Amount::MAX)
                        .or_else(id_search)
                        .unwrap_or_default(),
                )
            })
            .unwrap_or_default()
    }

    /// Gets an attribute with given `key` for the tokens with given `id`.
    ///
    /// Returns [`None`] if an attribute with given `key` doesn't exist.
    ///
    /// # Errors
    /// - [`MTError::InsufficientAmount`] if there are no tokens with given
    /// `id`.
    pub fn get_attribute(&self, id: &Id, key: &Vec<u8>) -> Result<Option<&Vec<u8>>, MTError> {
        self.tokens
            .iter()
            .find_map(|(stored_id, token)| (stored_id == id).then_some(&token.attributes))
            .ok_or(MTError::InsufficientAmount)
            .map(|attributes| {
                attributes
                    .iter()
                    .find_map(|(stored_key, value)| (stored_key == key).then_some(value))
            })
    }
}

impl From<SuperMTState> for MTState {
    fn from(state: SuperMTState) -> Self {
        let owner_convert = |(owner, token_owner_data): (_, super::TokenOwnerData)| {
            (
                owner,
                TokenOwnerData {
                    allowances: token_owner_data
                        .allowances
                        .into_iter()
                        .map(|(operator, allowance)| (operator, allowance.get()))
                        .collect(),
                    balance: token_owner_data.balance.get(),
                },
            )
        };

        let tokens = state
            .tokens
            .into_iter()
            .map(|(id, token)| {
                (
                    id,
                    Token {
                        total_supply: token.total_supply,
                        owners: token.owners.into_iter().map(owner_convert).collect(),
                        attributes: token.attributes.into_iter().collect(),
                    },
                )
            })
            .collect();

        let owners = state
            .owners
            .into_iter()
            .map(|(owner, general_owner_data)| {
                (
                    owner,
                    GeneralOwnerData {
                        balance: general_owner_data.balance,
                        operators: general_owner_data.operators.into_iter().collect(),
                    },
                )
            })
            .collect();

        Self {
            tokens,
            owners,
            total_supply: state.total_supply,
        }
    }
}

#[derive(Default, Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
struct Token {
    total_supply: Amount,
    owners: Vec<(Owner, TokenOwnerData)>,
    attributes: Vec<(Vec<u8>, Vec<u8>)>,
}

#[derive(Default, Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
struct TokenOwnerData {
    allowances: Vec<(Operator, Amount)>,
    balance: Amount,
}

#[derive(Default, Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
struct GeneralOwnerData {
    balance: Amount,
    operators: Vec<Operator>,
}

#[cfg(test)]
mod tests {
    use super::{super::ApproveType, *};
    use crate::tokens::test_helper::msg;

    #[test]
    fn total_supply() {
        let mut state = SuperMTState::new();
        let total_supply = 1000u64.into();

        state.mint(1.into(), 1u8.into(), total_supply).unwrap();

        let encoded_state = MTState::from(state);

        assert_eq!(encoded_state.total_supply(None), total_supply);
        assert_eq!(encoded_state.total_supply(Some(&0u8.into())), 0u64.into());
        assert_eq!(encoded_state.total_supply(Some(&1u8.into())), total_supply);
    }

    #[test]
    fn balance_of() {
        let mut state = SuperMTState::new();
        let amount = 1000u64.into();

        state.mint(1.into(), 0u8.into(), amount).unwrap();
        state.mint(1.into(), 1u8.into(), amount).unwrap();

        let encoded_state = MTState::from(state);

        assert_eq!(encoded_state.balance_of(2.into(), None), 0u64.into());
        assert_eq!(
            encoded_state.balance_of(2.into(), Some(&0u8.into())),
            0u64.into()
        );
        assert_eq!(encoded_state.balance_of(1.into(), None), amount + amount);
        assert_eq!(
            encoded_state.balance_of(1.into(), Some(&0u8.into())),
            amount
        );
        assert_eq!(
            encoded_state.balance_of(1.into(), Some(&0u8.into())),
            amount
        );
    }

    #[test]
    fn allowance() {
        let mut state = SuperMTState::new();

        state.mint(1.into(), 0u8.into(), 1000u64.into()).unwrap();
        msg::set_source(1.into());
        state
            .approve(
                2.into(),
                ApproveType::Allowance((0u8.into(), 1223u64.into())),
            )
            .unwrap();

        let mut encoded_state = MTState::from(state.clone());

        assert_eq!(
            encoded_state.allowance(1.into(), 2.into(), Some(&0u8.into())),
            1223u64.into()
        );

        state
            .approve(2.into(), ApproveType::Operator(true))
            .unwrap();

        encoded_state = MTState::from(state);

        assert_eq!(
            encoded_state.allowance(1.into(), 2.into(), Some(&0u8.into())),
            Amount::MAX
        );
    }

    #[test]
    fn get_attribute() {
        let key: Vec<_> = "A".into();
        let value: Vec<_> = "B".into();
        let mut state = SuperMTState::new();

        state.mint(1.into(), 0u8.into(), 0u64.into()).unwrap();
        state
            .set_attribute(&0u8.into(), key.clone(), value.clone())
            .unwrap();

        let encoded_state = MTState::from(state);

        assert_eq!(
            encoded_state.get_attribute(&123u8.into(), &key),
            Err(MTError::InsufficientAmount)
        );
        assert_eq!(
            encoded_state.get_attribute(&0u8.into(), &"ABCD".into()),
            Ok(None)
        );
        assert_eq!(
            encoded_state.get_attribute(&0u8.into(), &key),
            Ok(Some(&value))
        );
    }
}
