use crate::{MatchingStatus, PaymentConfirmation, PaymentStatus, TradeDetails};
use near_sdk::near_bindgen;

#[near_bindgen(event_json(standard = "rtp"))]
#[derive(Debug)]
pub enum RtpEvent {
    #[event_version("1.0.0")]
    NewBank { bank: String, bank_id: String },
    #[event_version("1.0.0")]
    SendTrade {
        partnership_id: String,
        bank_id: String,
        trade: TradeDetails,
    },
    #[event_version("1.0.0")]
    SetMatchingStatus {
        partnership_id: String,
        trade_id: String,
        matching_status: MatchingStatus,
    },
    #[event_version("1.0.0")]
    ConfirmPayment {
        partnership_id: String,
        bank_id: String,
        trade_id: String,
        confirmation: PaymentConfirmation,
    },
    #[event_version("1.0.0")]
    SetPaymentStatus {
        partnership_id: String,
        trade_id: String,
        payment_status: PaymentStatus,
    },
}
