use crate::*;

#[near_bindgen]
impl Pubdrop {
  pub fn get_key_balance(&self) -> U128 {
    self.has_active_drops();
    self.drop_balance.into()
  }
}
