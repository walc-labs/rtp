use crate::{Outcome, Trade};
use near_sdk::near_bindgen;

#[near_bindgen(event_json(standard = "rtp"))]
#[derive(Debug)]
pub enum RtpEventBindgen {
    #[event_version("1.0.0")]
    NewPartnership { partnership_id: String },
    #[event_version("1.0.0")]
    SendTrade { bank: String, trade: Trade },
    #[event_version("1.0.0")]
    SettleTrade { trade_id: String, outcome: Outcome },
}
