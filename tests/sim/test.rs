use crate::utils::init_simulation::init_simulation;
use crate::utils::pubdrop::Pubdrop;
use near_sdk::serde_json::json;

#[test]
fn test() {
  let (root, _) = init_simulation();
  let pubdrop = Pubdrop::default_new(root);

  let res = pubdrop.set_drops(10, "2.1");
  dbg!(res);
  let res = pubdrop.get_metadata();
  dbg!(res);
}
