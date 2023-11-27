use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    serde::{Deserialize, Serialize},
};

#[derive(Clone, Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TradeDetails {
    pub trade_id: String,
    pub timestamp: u64,
    pub deal_type: DealType,
    pub speed: Speed,
    pub contract: String,
    pub counterparty: String,
    // internal_external: String, // TODO not needed in smart contract?
    // TODO amount & price?
    pub amount: String,
    pub price: String,
    pub side: Side,
    pub settlement: Settlement,
    pub delivery_date: u64,
    pub payment_calendars: String, // TODO what is this?
    pub contract_number: String,   // TODO `contract_id`?
                                   // contract_timestamp: u64, // TODO
                                   // trade: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum DealType {
    FxDeal,
}

#[derive(Clone, Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum Speed {
    RealTime,
    Spot,
    Forward,
}

#[derive(Clone, Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Clone, Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum Settlement {
    RealTime,
    T(u16),
    // Other, // TODO needed?
}

#[derive(Clone, Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Trade {
    pub bank: String,
    pub trade_details: TradeDetails,
    pub deal_status: DealStatus,
    pub payments: Payments,
}

#[derive(Clone, Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(crate = "near_sdk::serde")]
#[serde(tag = "status", content = "message")]
pub enum DealStatus {
    /// new trade sent to smart contract that has not yet been matched and confirmed
    Pending,
    /// confirmed trade and matched with counter trade, but not yet executed
    Confirmed(String),
    /// rejected trade. Invalid match against counter party
    Rejected(String),
    /// confirmed and executed trade by escrow/nostro
    Executed(String),
    Error,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Payments {
    pub credit: bool,
    pub debit: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum PaymentConfirmation {
    Credit,
    Debit,
}
