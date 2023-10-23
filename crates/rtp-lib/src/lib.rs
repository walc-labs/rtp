mod contract;
mod error;

pub use contract::*;
pub use error::*;

use near_sdk::{
    borsh::{self, BorshSerialize},
    BorshStorageKey,
};

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
    TradeA,
    TradeB,
}
