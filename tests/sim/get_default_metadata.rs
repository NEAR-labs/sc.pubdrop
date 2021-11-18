use crate::utils::init_simulation::init_simulation;
use crate::utils::pubdrop::Pubdrop;
use near_sdk::serde_json::json;

#[test]
fn get_default_metadata() {
  let (root, _) = init_simulation();
  let pubdrop = Pubdrop::default_new(root);

  let metadata = pubdrop.get_metadata();

  let expected_result = json!({
     "active_drops": 0,
     "drop_balance": "0",
     "account_creator": "testnet"
  });

  assert_eq!(expected_result, metadata);
}
