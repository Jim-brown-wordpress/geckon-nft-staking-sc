elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::module]
pub trait StorageModule {

    // storage
    #[view(getOldNftTokenId)]
    #[storage_mapper("old_nft_token_id")]
    fn old_nft_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getNewNftTokenId)]
    #[storage_mapper("new_nft_token_id")]
    fn new_nft_token_id(&self) -> SingleValueMapper<TokenIdentifier>;
    
}