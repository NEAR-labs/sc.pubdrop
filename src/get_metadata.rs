use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Metadata {
  active_drops: u32,
  drop_balance: U128,
  account_creator: AccountId,
}

#[near_bindgen]
impl Pubdrop {
  pub fn get_metadata(self) -> Metadata {
    Metadata {
      active_drops: self.active_drops,
      drop_balance: U128::from(self.drop_balance),
      account_creator: self.account_creator,
    }
  }
}
