use crate::*;

#[near_bindgen]
impl Pubdrop {
  #[private]
  #[payable]
  pub fn set_drops(&mut self, active_drops: u32, drop_balance: Balance) {
    let balance = env::account_balance();
    log!("{}", balance);

    self.active_drops = active_drops;
    self.drop_balance = drop_balance;
  }
}
