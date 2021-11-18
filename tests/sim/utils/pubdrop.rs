use near_sdk::json_types::{U128, U64};
use near_sdk::serde_json::{json, Value};
use near_sdk_sim::transaction::ExecutionOutcome;
use near_sdk_sim::{lazy_static_include, to_yocto, ExecutionResult, UserAccount, DEFAULT_GAS};
use pubdrop::PubdropContract;
use std::convert::TryFrom;
use std::rc::Rc;

lazy_static_include::lazy_static_include_bytes! {
   PUBDROP_WASM => "wasm/pubdrop.wasm",
}

#[derive(Debug)]
pub struct Pubdrop {
  pub account: UserAccount,
}

impl Pubdrop {
  pub fn deploy(root_account: Rc<UserAccount>, account_id: &str, init_balance: &str) -> Self {
    let account = root_account.deploy(
      &PUBDROP_WASM,
      account_id.parse().unwrap(),
      to_yocto(init_balance),
    );
    Self { account }
  }

  pub fn new(&self, account_creator: &str) -> ExecutionResult {
    self.account.call(
      self.account.account_id.clone(),
      "new",
      json!({ "account_creator": account_creator })
        .to_string()
        .as_bytes(),
      DEFAULT_GAS,
      0,
    )
  }

  pub fn default_new(root_account: Rc<UserAccount>) -> Self {
    let pubdrop = Self::deploy(root_account, "pubdrop", "100");
    pubdrop.new("testnet");
    pubdrop
  }

  pub fn get_metadata(&self) -> Value {
    self
      .account
      .view(self.account.account_id.clone(), "get_metadata", &[])
      .unwrap_json_value()
  }
}
