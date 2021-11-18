mod add_claim_key;
mod claim;
mod create_account_and_claim;
mod get_key_balance;
mod get_metadata;
mod new;
mod update_drops;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::serde_json::json;
use near_sdk::{
  env, is_promise_success, near_bindgen, AccountId, Balance, Gas, PanicOnDefault, Promise,
  PublicKey,
};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Pubdrop {
  active_drops: u64,
  drop_balance: Balance,
  account_creator: AccountId,
}

// Utils
impl Pubdrop {
  fn can_claim(&self) {
    assert!(self.active_drops > 0, "No active drops available");
  }
}

fn tgas(value: u64) -> Gas {
  Gas(value * 10u64.pow(12))
}
