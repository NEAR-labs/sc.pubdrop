use crate::*;

#[near_bindgen]
impl Pubdrop {
  // // #[init]
  // pub fn dnew(
  //   account_creator: AccountId,
  //   claim_public_key: PublicKey,
  //   claim_secret_key: String,
  // ) -> Promise {
  //   Promise::new(env::current_account_id()).add_access_key(
  //     claim_public_key,
  //     0,
  //     env::current_account_id(),
  //     "claim,create_account_and_claim".to_string(),
  //   )
  // }
  #[init]
  pub fn new(
    account_creator: AccountId,
    claim_public_key: PublicKey,
    claim_secret_key: String,
  ) -> Self {
    Self {
      active_drops: 0,
      drop_balance: 0,
      account_creator,
      claim_public_key,
      claim_secret_key,
    }
  }
}
