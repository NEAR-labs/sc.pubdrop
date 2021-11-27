use crate::*;

#[near_bindgen]
impl Pubdrop {
  #[private]
  pub fn create_account_and_claim(
    self,
    new_account_id: AccountId,
    new_public_key: PublicKey,
  ) -> Promise {
    self.has_active_drops();

    Promise::new(self.account_creator)
      .function_call(
        "create_account".to_string(),
        json!({
          "new_account_id": new_account_id,
          "new_public_key": new_public_key
        })
        .to_string()
        .into_bytes(),
        self.drop_balance,
        tgas(50),
      )
      .then(
        Promise::new(env::current_account_id()).function_call(
          "on_create_account_and_claim".to_string(),
          json!({
            "new_account_id": new_account_id,
          })
          .to_string()
          .into_bytes(),
          0,
          tgas(20),
        ),
      )
  }

  #[private]
  pub fn on_create_account_and_claim(&mut self, new_account_id: AccountId) -> Promise {
    assert!(
      is_promise_success(),
      "New account @{} wasn't created",
      new_account_id
    );

    self.active_drops -= 1;
    log!("New account @{} claimed a drop", new_account_id,);

    Promise::new(env::current_account_id()).delete_key(env::signer_account_pk())
  }
}
