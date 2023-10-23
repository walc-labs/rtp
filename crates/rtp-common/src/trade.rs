use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    serde::{Deserialize, Serialize},
};

#[derive(Clone, Debug, Eq, PartialEq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Trade {
    timestamp: u64,
    deal_type: DealType,
    speed: Speed,
    contract: String,
    // counterparty: String, // TODO not needed in smart contract?
    // internal_external: String, // TODO not needed in smart contract?
    pub side: Side,
    settlement: Settlement,
    delivery_date: u64,
    payment_calendars: String, // TODO what is this?
    deal_status: DealStatus,
    contract_number: String, // TODO `contract_id`?
    trade_number: String,    // TODO `trade_id`?
                             // contract_timestamp: u64, // TODO
                             // trade: String,
}

#[derive(Clone, Debug, Eq, PartialEq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum DealType {
    FxDeal,
}

#[derive(Clone, Debug, Eq, PartialEq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum Speed {
    RealTime,
    Spot,
    Forward,
}

#[derive(Clone, Debug, Eq, PartialEq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Clone, Debug, Eq, PartialEq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum Settlement {
    RealTime,
    T(u16),
    // Other, // TODO needed?
}

#[derive(Clone, Debug, Eq, PartialEq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum DealStatus {
    /// new trade sent to smart contract that has not yet been matched and confirmed
    Pending,
    /// confirmed trade and matched with counter trade, but not yet executed
    Confirmed,
    /// confirmed and executed trade by escrow/nostro
    Executed,
}
