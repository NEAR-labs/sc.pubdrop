use crate::*;

#[near_bindgen]
impl Pubdrop {
  #[private]
  pub fn claim(&self, account_id: AccountId) -> Promise {
    self.has_active_drops();

    Promise::new(account_id.clone())
      .transfer(self.drop_balance)
      .then(
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
  pub fn on_claim(&mut self, beneficiary_id: AccountId) -> Promise {
    assert!(is_promise_success(), "Claim failed by @{}", beneficiary_id);

    self.active_drops -= 1;
    log!("Successful claim by @{}", beneficiary_id,);

    Promise::new(env::current_account_id()).delete_key(env::signer_account_pk())
  }
}
