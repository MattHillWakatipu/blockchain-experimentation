use crate::*;

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn nft_cure(&mut self) {
        // get the sender address
        let sender_id = env::predecessor_account_id();
        let burn_address: AccountId = "burn.near".to_string();

        // get a token to cure
        let tokens_set = self.tokens_per_owner.get(&sender_id).expect("Account not infected.");

        tokens_set.iter()
            .map(|token_id| self.internal_transfer(
                &sender_id,
                &burn_address,
                &token_id,
                None,
                None,
            )).for_each(drop);
    }
}