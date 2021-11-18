use crate::*;

#[near_bindgen]
impl Pubdrop {
  #[init]
  pub fn new(account_creator: AccountId, active_drops: u64, drop_balance: Balance) -> Self {
    Self {
      active_drops,
      drop_balance,
      account_creator,
    }
  }
}
