use crate::*;

#[near_bindgen]
impl Pubdrop {
  #[private]
  pub fn set_drops(&mut self, active_drops: u32, drop_balance: U128) {
    self.active_drops = active_drops;
    self.drop_balance = Balance::from(drop_balance);
  }
}
