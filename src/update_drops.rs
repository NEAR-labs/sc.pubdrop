use crate::*;

#[near_bindgen]
impl Pubdrop {
  #[private]
  #[payable]
  pub fn update_drops(&mut self, active_drops: u64, drop_balance: Balance) {
    self.active_drops = active_drops;
    self.drop_balance = drop_balance;
  }
}
