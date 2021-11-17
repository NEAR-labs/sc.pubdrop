use crate::*;

#[near_bindgen]
impl Pubdrop {
  pub fn claim(&self, account_id: AccountId) -> Promise {
    self.can_claim();

    Promise::new(account_id.clone()).transfer(self.drop_balance).then(
      Promise::new(env::current_account_id()).function_call(
        "on_claim".to_string(),
        json!({
          "beneficiary_id": account_id,
        })
        .to_string()
        .into_bytes(),
        0,
        tgas(20),
      ),
    )
  }

  #[private]
  pub fn on_claim(&mut self, beneficiary_id: AccountId) {
    assert!(is_promise_success(), "Claim failed by @{}", beneficiary_id);
    self.active_drops -= 1;
    env::log_str(format!("Successful claim by @{}", beneficiary_id,).as_str());
  }
}

// if !is_promise_success() {
//   env::panic_str(format!("Claim failed by @{}", beneficiary_id).as_str());
// }