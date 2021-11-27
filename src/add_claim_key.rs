use crate::*;

const ACCESS_KEY_ALLOWANCE: u128 = 90_000_000_000_000_000_000_000; // 0.09 NEAR

#[near_bindgen]
impl Pubdrop {
  #[private]
  pub fn add_claim_key(&mut self, public_key: PublicKey) -> Promise {
    Promise::new(env::current_account_id()).add_access_key(
      public_key,
      ACCESS_KEY_ALLOWANCE,
      env::current_account_id(),
      "claim,create_account_and_claim".to_string(),
    )
  }
}
