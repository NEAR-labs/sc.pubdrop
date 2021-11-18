use near_sdk::serde_json::{json, Value};
use near_sdk_sim::{lazy_static_include, to_yocto, ExecutionResult, UserAccount, DEFAULT_GAS};
use pubdrop::PubdropContract;
use std::rc::Rc;

lazy_static_include::lazy_static_include_bytes! {
   PUBDROP_WASM => "wasm/pubdrop.wasm",
}

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

  pub fn initialize_contract(
    &self,
    account_creator: &str,
    claim_public_key: &str,
    claim_secret_key: &str,
  ) {
    self.account.call(
      self.account.account_id.clone(),
      "new",
      json!({
        "account_creator": account_creator,
        "claim_public_key": claim_public_key,
        "claim_secret_key": claim_secret_key
      })
      .to_string()
      .as_bytes(),
      DEFAULT_GAS,
      0,
    );
  }

  pub fn default_init(root_account: Rc<UserAccount>) -> Self {
    let mut pubdrop = Self::deploy(root_account, "pubdrop", "100");
    pubdrop.initialize_contract(
      "testnet",
      "8bFrYwXUEvLH5zkzGn2fG2bKjJu3kNNP4xXqsBvc2nJe",
      "39qnXSsiUUtuyMMJBkepa3qfv44qe6ZfixEMC9no1v6kjnaaKYj1pZ8pFmci1rSE9c2GsMVhF2NpXgu5aAYbCq3Y",
    );
    pubdrop
  }
}
