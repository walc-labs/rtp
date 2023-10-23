mod trade;

pub use trade::*;

use near_sdk::{near_bindgen, AccountId};

#[near_bindgen(event_json(standard = "rtp"))]
#[derive(Debug)]
pub enum RtpEvent {
    #[event_version("1.0.0")]
    NewPartnership { partnership_id: AccountId },
    #[event_version("1.0.0")]
    SendTrade { bank: String, trade: Trade },
}
