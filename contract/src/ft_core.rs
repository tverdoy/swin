use crate::Contract;
use near_sdk::{near_bindgen, AccountId, NearToken, PromiseOrValue};
use near_sdk::json_types::U128;

use crate::*;


trait FungibleTokenCore {
    /// Transfers a specified amount of FTs to a receiver ID.
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: NearToken, memo: Option<String>);

    fn ft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        amount: NearToken,
        memo: Option<String>,
        msg: String,
    );

    /// Returns the total amount of fungible tokens in circulation on the contract.
    fn ft_total_supply(&self) -> U128;


    /// Returns how many fungible tokens a specific user owns.
    fn ft_balance_of(&self, account_id: AccountId) -> NearToken;

    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: NearToken,
        msg: String,
    ) -> PromiseOrValue<NearToken>;

    fn ft_resolve_transfer(
        &mut self,
        sender_id: &AccountId,
        receiver_id: AccountId,
        amount: NearToken,
    ) -> NearToken;
}

#[near_bindgen]
impl FungibleTokenCore for Contract {
    #[payable]
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: NearToken, memo: Option<String>) {
        todo!()
    }

    #[payable]
    fn ft_transfer_call(&mut self, receiver_id: AccountId, amount: NearToken, memo: Option<String>, msg: String) {
        todo!()
    }

    fn ft_total_supply(&self) -> U128 {
        todo!()
    }

    fn ft_balance_of(&self, account_id: AccountId) -> NearToken {
        todo!()
    }

    fn ft_on_transfer(&mut self, sender_id: AccountId, amount: NearToken, msg: String) -> PromiseOrValue<NearToken> {
        todo!()
    }

    #[private]
    fn ft_resolve_transfer(&mut self, sender_id: &AccountId, receiver_id: AccountId, amount: NearToken) -> NearToken {
        todo!()
    }
}
