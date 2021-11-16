use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, ext_contract, near_bindgen, Promise, AccountId};

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Pubdrop {}
