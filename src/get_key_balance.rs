use crate::*;

#[near_bindgen]
impl Pubdrop {
  #[allow(unused_variables)]
  pub fn get_key_balance(&self, key: PublicKey) -> U128 {
    self.has_active_drops();
    self.drop_balance.into()
  }
}
