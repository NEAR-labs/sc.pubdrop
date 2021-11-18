use crate::*;

#[near_bindgen]
impl Pubdrop {
  #[init]
  pub fn new(account_creator: AccountId) -> Self {
    Self {
      active_drops: 0,
      drop_balance: 0,
      account_creator,
    }
  }
}
