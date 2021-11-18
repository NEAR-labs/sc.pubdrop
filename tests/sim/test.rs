use crate::utils::init_simulation::init_simulation;
use crate::utils::pubdrop::Pubdrop;
use near_sdk::serde_json::json;

#[test]
fn test() {
  let (root, _) = init_simulation();
  let pubdrop = Pubdrop::default_new(root);

  let metadata = pubdrop.get_metadata();
}
