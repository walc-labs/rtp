use crate::{DealStatus, TradeDetails};
use near_sdk::near_bindgen;

#[near_bindgen(event_json(standard = "rtp"))]
#[derive(Debug)]
pub enum RtpEventBindgen {
    #[event_version("1.0.0")]
    NewPartnership { partnership_id: String },
    #[event_version("1.0.0")]
    SendTrade {
        partnership_id: String,
        bank: String,
        trade: TradeDetails,
    },
    #[event_version("1.0.0")]
    SettleTrade {
        partnership_id: String,
        trade_id: String,
        deal_status: DealStatus,
    },
}
