use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    serde::{Deserialize, Serialize},
};

#[derive(Clone, Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Trade {
    pub bank: String,
    pub trade_details: TradeDetails,
    pub matching_status: MatchingStatus,
    pub payment_status: PaymentStatus,
    pub payments: Payments,
}

#[derive(Clone, Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TradeDetails {
    pub event_timestamp: u64,
    pub recv_time: u64,
    pub instrument_id: String,
    pub asset_class: String,
    pub product: Product,
    pub side: Side,
    pub price: f32,
    pub notional_amount: f32,
    pub event_type: String,
    pub venue: String,
    pub trading_platform: String,
    pub source_data: String,
    pub source_connection: String,
    pub trade_id: String,
    pub execution_date: String,
    pub trader_id: String,
    pub account: String,
    pub counterparty: String,
    pub counterparty_id: String,
    pub ccy: String,
    pub legal_entity_id: String,
    pub legal_entity: String,
    pub termination_date: String,
    pub buyer: String,
    pub seller_id: String,
    pub seller: String,
    pub effective_date: String,
    pub contract_typology: String,
    pub client_loco: String,
    pub mx_family: String,
    pub mx_group: String,
    pub mx_type: String,
    pub agreement: String,
    pub settlement_method: String,
    pub limits: bool,
    pub authorization: bool,
    pub aml_check: bool,
    pub sanctions: bool,
    pub settlement_pvp: Settlement,
    pub delivery_method: String,
    pub delivery_timestap: u64,
    pub delivery_date: String,
    pub time_zone: String,
    pub payment_calendar: String,
    pub rtp1_ccy: String,
    pub rtp1_fee: f32,
    pub rtp1_timestamp: u64,
    pub rtp2_ccy: String,
    pub rtp2_fee: f32,
    pub rtp2_timestamp: u64,
    pub execution_place: String,

    pub ccy1_value_date: String,
    pub ccy2_value_date: Option<String>,
    pub dealt_ccy: String,
    pub ccy1_discount_factor: f32,
    pub ccy1_payer_party_id: String,
    pub ccy1_payment_amt: f32,
    pub ccy1_payment_ccy: String,
    pub ccy1_payer_book_id: Option<String>,
    pub ccy1_rec_book_id: Option<String>,
    pub ccy1_payment_date_u: String,
    pub ccy2_discount_factor: Option<f32>,
    pub ccy2_payer_party_id: Option<String>,
    pub ccy2_payment_amt: Option<f32>,
    pub ccy2_payment_ccy: Option<String>,
    pub ccy2_payer_book_id: Option<String>,
    pub ccy2_rec_book_id: Option<String>,
    pub ccy2_payment_date_u: Option<String>,
    pub secondary_trade_id: String,
    pub source_trade_id: String,
}

impl Default for TradeDetails {
    fn default() -> Self {
        Self {
            event_timestamp: 1704980135044,
            recv_time: 1704980135044,
            instrument_id: "EUR/USD".to_string(),
            asset_class: "Fx".to_string(),
            product: Product::Spot,
            side: Side::Buy,
            price: 1.15,
            notional_amount: 1_000.,
            event_type: "new".to_string(),
            venue: "bank_a".to_string(),
            trading_platform: "Murex".to_string(),
            source_data: "Murex".to_string(),
            source_connection: "file_name".to_string(),
            trade_id: "trade_id".to_string(),
            execution_date: "08.12.2023".to_string(),
            trader_id: "trader_a".to_string(),
            account: "12345".to_string(),
            counterparty: "bank_b".to_string(),
            counterparty_id: "203948".to_string(),
            ccy: "EUR".to_string(),
            legal_entity_id: "68119".to_string(),
            legal_entity: "222233".to_string(),
            termination_date: "22.12.2023".to_string(),
            buyer: "SS_CLIENT_68119".to_string(),
            seller_id: "10078".to_string(),
            seller: "SS_CLIENT_10078".to_string(),
            effective_date: "05.12.2023".to_string(),
            contract_typology: "Spot".to_string(),
            client_loco: "London".to_string(),
            mx_family: "CURR".to_string(),
            mx_group: "FXD".to_string(),
            mx_type: "Spot".to_string(),
            agreement: "ISDA".to_string(),
            settlement_method: "Nostro".to_string(),
            limits: true,
            authorization: true,
            aml_check: true,
            sanctions: true,
            settlement_pvp: Settlement::RealTime,
            delivery_method: "PVP".to_string(),
            delivery_timestap: 1704980135044,
            delivery_date: "Real Time".to_string(),
            time_zone: "UTC".to_string(),
            payment_calendar: "NYLN".to_string(),
            rtp1_ccy: "EUR".to_string(),
            rtp1_fee: 0.1,
            rtp1_timestamp: 1704980135044,
            rtp2_ccy: "USD".to_string(),
            rtp2_fee: 0.1,
            rtp2_timestamp: 1704980135044,
            execution_place: "London".to_string(),
            ccy1_value_date: "11.12.2023".to_string(),
            ccy2_value_date: None,
            dealt_ccy: "EUR".to_string(),
            ccy1_discount_factor: 0.15,
            ccy1_payer_party_id: "10078".to_string(),
            ccy1_payment_amt: 1_000.,
            ccy1_payment_ccy: "EUR".to_string(),
            ccy1_payer_book_id: None,
            ccy1_rec_book_id: None,
            ccy1_payment_date_u: "15.12.2023".to_string(),
            ccy2_discount_factor: None,
            ccy2_payer_party_id: None,
            ccy2_payment_amt: None,
            ccy2_payment_ccy: None,
            ccy2_payer_book_id: None,
            ccy2_rec_book_id: None,
            ccy2_payment_date_u: None,
            secondary_trade_id: Default::default(),
            source_trade_id: Default::default(),
        }
    }
}

impl TradeDetails {
    pub fn default_swap() -> Self {
        Self {
            ccy2_value_date: Some("12.12.2023".to_string()),
            ccy2_discount_factor: Some(0.15),
            ccy2_payer_party_id: Some("73355".to_string()),
            ccy2_payment_amt: Some(1_150.),
            ccy2_payment_ccy: Some("USD".to_string()),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(crate = "near_sdk::serde")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Product {
    Spot,
    Ndf,
    Fwd,
    Swap,
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
}

#[derive(Clone, Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(crate = "near_sdk::serde")]
#[serde(tag = "status", content = "message")]
pub enum MatchingStatus {
    Pending,
    Confirmed(String),
    Rejected(String),
    Error,
}

#[derive(Clone, Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(crate = "near_sdk::serde")]
#[serde(tag = "status", content = "message")]
pub enum PaymentStatus {
    Pending,
    Confirmed(String),
    Rejected(String),
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
