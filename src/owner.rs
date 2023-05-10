elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::module]
pub trait OwnerModule: crate::storage::StorageModule {
    #[only_owner]
    #[endpoint(setOldNftTokenId)]
    fn set_old_nft_token_id(&self, old_nft_token_id: TokenIdentifier) {
        self.old_nft_token_id().set(old_nft_token_id);
    }

    #[only_owner]
    #[endpoint(setNewNftTokenId)]
    fn set_new_nft_token_id(&self, new_nft_token_id: TokenIdentifier) {
        self.new_nft_token_id().set(new_nft_token_id);
    }

    #[only_owner]
    #[endpoint(withdraw)]
    fn withdraw(&self,
        opt_token_id: OptionalValue<TokenIdentifier>,
        opt_token_nonce: OptionalValue<u64>,
        opt_token_amount: OptionalValue<BigUint>
    ) {
        // if token_id is not given, set it to eGLD
        let token_id = match opt_token_id {
            OptionalValue::Some(v) => v,
            OptionalValue::None => TokenIdentifier::egld()
        };

        // if token_id is not given, set it to 0
        let token_nonce = match opt_token_nonce {
            OptionalValue::Some(v) => v,
            OptionalValue::None => 0,
        };

        // if token_amount is not given, set it to balance of SC - max value to withdraw
        let token_amount = match opt_token_amount {
            OptionalValue::Some(v) => v,
            OptionalValue::None => self.blockchain().get_sc_balance(&token_id, 0)
        };

        self.send().direct(&self.blockchain().get_caller(), &token_id, token_nonce, &token_amount, &[]);
    }
}