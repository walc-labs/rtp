use near_sdk::serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Trade {
    pub timestamp: u64,
    pub deal_type: DealType,
    pub speed: Speed,
    pub contract: String,
    // counterparty: String, // TODO not needed in smart contract?
    // internal_external: String, // TODO not needed in smart contract?
    pub side: Side,
    pub settlement: Settlement,
    pub delivery_date: u64,
    pub payment_calendars: String, // TODO what is this?
    pub deal_status: DealStatus,
    pub contract_number: String, // TODO `contract_id`?
    pub trade_id: String,        // TODO `trade_id`?
                                 // contract_timestamp: u64, // TODO
                                 // trade: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum DealType {
    FxDeal,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum Speed {
    RealTime,
    Spot,
    Forward,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum Settlement {
    RealTime,
    T(u16),
    // Other, // TODO needed?
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum DealStatus {
    /// new trade sent to smart contract that has not yet been matched and confirmed
    Pending,
    /// confirmed trade and matched with counter trade, but not yet executed
    Confirmed,
    /// confirmed and executed trade by escrow/nostro
    Executed,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
#[serde(tag = "outcome", content = "message")]
pub enum Outcome {
    Success(String),
    Failure(String),
}
