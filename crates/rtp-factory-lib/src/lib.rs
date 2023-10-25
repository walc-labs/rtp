mod contract;
mod error;

pub use contract::*;
pub use error::*;

use near_sdk::{
    borsh::{self, BorshSerialize},
    ext_contract, Balance, BorshStorageKey, Gas, ONE_NEAR,
};
use rtp_common::Trade;

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
    PartnershipContracts,
    ContractCode,
}

/// Gas spent on the call & account creation.
const CREATE_CALL_GAS: Gas = Gas::from_tgas(40);

/// Gas allocated on the callback.
const ON_CREATE_CALL_GAS: Gas = Gas::from_tgas(10);

const REPRESENTATIVE_DEPOSIT_TO_COVER_GAS: Balance = ONE_NEAR;

#[ext_contract(rtp)]
trait Rtp {
    fn perform_trade(&mut self, bank: String, trade: Trade);
}
