use near_sdk::{
    borsh::{self, BorshSerialize},
    FunctionError,
};
use thiserror::Error;

#[derive(BorshSerialize, Debug, Error, FunctionError)]
pub enum ContractError {
    #[error("Only the factory contract can call this function")]
    NotFactory,
    #[error("Invalid bank")]
    InvalidBank,
    #[error("Trade ID does not exist")]
    InvalidTradeId,
    #[error("Trade incompete")]
    TradeIncomplete,
}
