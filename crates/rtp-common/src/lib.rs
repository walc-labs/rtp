use near_sdk::AccountId;
use owo_colors::OwoColorize;
use rtp_contract_common::{Outcome, Trade};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "standard")]
#[serde(rename_all = "kebab-case")]
pub enum ContractEvent {
    Rtp(RtpEvent),
}

pub const KNOWN_EVENT_KINDS: [&str; 3] = ["new_partnership", "send_trade", "settle_trade"];

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RtpEvent {
    pub version: String,
    #[serde(flatten)]
    pub event_kind: RtpEventKind,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum RtpEventKind {
    NewPartnership(NewPartnership),
    SendTrade(SendTrade),
    SettleTrade(SettleTrade),
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NewPartnership {
    partnership_id: AccountId,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SendTrade {
    bank: String,
    trade: Trade,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SettleTrade {
    trade_id: String,
    outcome: Outcome,
}

impl Display for ContractEvent {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ContractEvent::Rtp(event) => formatter.write_fmt(format_args!("{}", event)),
        }
    }
}

impl Display for RtpEvent {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match &self.event_kind {
            RtpEventKind::NewPartnership(_) => {
                formatter.write_fmt(format_args!("{}: new_partnership", "event".bright_cyan()))?;
            }
            RtpEventKind::SendTrade(_) => {
                formatter.write_fmt(format_args!("{}: send_trade", "event".bright_cyan()))?;
            }
            RtpEventKind::SettleTrade(_) => {
                formatter.write_fmt(format_args!("{}: settle_trade", "event".bright_cyan()))?;
            }
        }
        formatter.write_fmt(format_args!("\n{}: rtp", "standard".bright_cyan(),))?;
        formatter.write_fmt(format_args!(
            "\n{}: {}",
            "version".bright_cyan(),
            self.version
        ))?;
        match &self.event_kind {
            RtpEventKind::NewPartnership(data) => {
                formatter.write_fmt(format_args!("\n{}: {:?}", "data".bright_cyan(), data))?;
            }
            RtpEventKind::SendTrade(data) => {
                formatter.write_fmt(format_args!("\n{}: {:?}", "data".bright_cyan(), data))?;
            }
            RtpEventKind::SettleTrade(data) => {
                formatter.write_fmt(format_args!("\n{}: {:?}", "data".bright_cyan(), data))?;
            }
        }
        Ok(())
    }
}
