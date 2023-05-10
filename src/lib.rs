#![no_std]

elrond_wasm::imports!();

mod storage;
mod owner;

#[elrond_wasm::derive::contract]
pub trait GeckonSwap:
    owner::OwnerModule
    + storage::StorageModule
{
    
    #[init]
    fn init(
        &self,
        old_nft_token_id: TokenIdentifier,
        new_nft_token_id: TokenIdentifier
    ) {
        self.old_nft_token_id().set(old_nft_token_id);
        self.new_nft_token_id().set(new_nft_token_id);
    }

    #[payable("*")]
    #[endpoint(swap)]
    fn swap(&self) {

        let caller = self.blockchain().get_caller();

        let payments = self.call_value().all_esdt_transfers();
        for payment in payments.iter() {
            let nft_token_id = payment.token_identifier;
            let nft_amount = payment.amount;
            let nft_nonce = payment.token_nonce;
            require!(
                nft_token_id == self.old_nft_token_id().get(),
                "invalid nft_token_id"
            );
            require!(
                nft_amount >= BigUint::zero(),
                "must more than 1 nft"
            );

            self.send().direct(&caller, &self.new_nft_token_id().get(), nft_nonce, &nft_amount, &[]);
        }
    }
}
