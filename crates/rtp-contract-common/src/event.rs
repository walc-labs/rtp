use crate::{DealStatus, PaymentConfirmation, TradeDetails};
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
    SettleTrade {
        partnership_id: String,
        trade_id: String,
        deal_status: DealStatus,
    },
    #[event_version("1.0.0")]
    ConfirmPayment {
        partnership_id: String,
        bank_id: String,
        trade_id: String,
        confirmation: PaymentConfirmation,
    },
}
