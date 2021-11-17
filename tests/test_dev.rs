use std::rc::Rc;
use self::utils;

use crate::foo;

#[test]
fn test() {
  let (root, _runtime) = init_simulation();
  let root = Rc::new(root);
}