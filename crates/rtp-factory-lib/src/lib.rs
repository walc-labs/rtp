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
use rtp_contract_common::{DealStatus, PaymentConfirmation, TradeDetails};

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
    BankIds,
    ContractCode,
}

/// Gas spent on the call & account creation.
const CREATE_CALL_GAS: Gas = Gas::from_tgas(40);

/// Gas allocated on the callback.
const ON_CREATE_CALL_GAS: Gas = Gas::from_tgas(10);

const BANK_DEPOSIT_COVER_ADDITIONAL_BYTES: usize = 256;

const BANK_DEPOSIT_TO_COVER_GAS: Balance = 2 * ONE_NEAR;

#[ext_contract(rtp)]
trait Rtp {
    fn perform_trade(&mut self, trade_details: TradeDetails);

    fn settle_trade(&mut self, trade_id: String, deal_status: DealStatus);

    fn confirm_payment(&mut self, trade_id: String, confirmation: PaymentConfirmation);

    fn delete_account(&mut self);
}
