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
    #[error("Bank contract already exists")]
    BankAlreadyExists,
    #[error("Bank contract does not yet exists")]
    BankNotYetExists,
    #[error("Invalid bank input")]
    InvalidBankInput,
}
