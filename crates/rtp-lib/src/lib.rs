use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    near_bindgen, BorshStorageKey, PanicOnDefault,
};

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Contract {}

// #[derive(BorshStorageKey, BorshSerialize)]
// pub enum StorageKey {}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {}
    }
}
