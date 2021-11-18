use crate::utils::init_simulation::init_simulation;
use crate::utils::pubdrop::Pubdrop;
use std::rc::Rc;

#[test]
fn dev() {
  let (root, _runtime) = init_simulation();
  let root = Rc::new(root);
}
