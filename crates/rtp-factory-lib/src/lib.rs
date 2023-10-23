mod contract;
mod error;

pub use contract::*;
pub use error::*;

use near_sdk::{
    borsh::{self, BorshSerialize},
    BorshStorageKey, Gas,
};

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
    PartnershipContracts,
    ContractCode,
}

/// Gas spent on the call & account creation.
const CREATE_CALL_GAS: Gas = Gas::from_tgas(40);

/// Gas allocated on the callback.
const ON_CREATE_CALL_GAS: Gas = Gas::from_tgas(10);
