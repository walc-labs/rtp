mod contract;
mod error;
mod view;

pub use contract::*;
pub use error::*;
pub use view::*;

use near_sdk::{
    borsh::{self, BorshSerialize},
    ext_contract, Balance, BorshStorageKey, Gas, ONE_NEAR,
};
use rtp_contract_common::{Outcome, Trade};

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

    fn settle_trade(&mut self, trade_id: String, outcome: Outcome);
}
