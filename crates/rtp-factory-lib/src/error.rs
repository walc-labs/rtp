use near_sdk::{
    borsh::{self, BorshSerialize},
    Balance, FunctionError,
};
use thiserror::Error;

#[derive(BorshSerialize, Debug, Error, FunctionError)]
pub enum ContractError {
    #[error("No input")]
    NoInput,
    #[error("Not enough deposit. Required: {_0}; actual: {_1}")]
    NotEnoughDeposit(Balance, Balance),
    #[error("Partnership contract already exists")]
    PartnershipAlreadyExists,
    #[error("Invalid bank input")]
    InvalidBankInput,
}
