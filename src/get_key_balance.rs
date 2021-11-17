use crate::*;

#[near_bindgen]
impl Pubdrop {
  pub fn get_key_balance(&self) -> U128 {
    assert!(self.active_drops > 0, "No active drops available");
    self.drop_balance.into()
  }
}
