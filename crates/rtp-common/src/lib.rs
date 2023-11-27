use owo_colors::OwoColorize;
use rtp_contract_common::{DealStatus, PaymentConfirmation, TradeDetails};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "standard")]
#[serde(rename_all = "kebab-case")]
pub enum ContractEvent {
    Rtp(RtpEvent),
}

pub const KNOWN_EVENT_KINDS: [&str; 4] =
    ["new_bank", "send_trade", "settle_trade", "confirm_payment"];

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
    NewBank(NewBank),
    SendTrade(SendTrade),
    SettleTrade(SettleTrade),
    ConfirmPayment(ConfirmPayment),
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NewBank {
    pub bank: String,
    pub bank_id: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SendTrade {
    pub partnership_id: String,
    pub bank_id: String,
    pub trade: TradeDetails,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SettleTrade {
    pub partnership_id: String,
    pub trade_id: String,
    pub deal_status: DealStatus,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ConfirmPayment {
    pub partnership_id: String,
    pub bank_id: String,
    pub trade_id: String,
    pub confirmation: PaymentConfirmation,
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
            RtpEventKind::NewBank(_) => {
                formatter.write_fmt(format_args!("{}: new_partnership", "event".bright_cyan()))?;
            }
            RtpEventKind::SendTrade(_) => {
                formatter.write_fmt(format_args!("{}: send_trade", "event".bright_cyan()))?;
            }
            RtpEventKind::SettleTrade(_) => {
                formatter.write_fmt(format_args!("{}: settle_trade", "event".bright_cyan()))?;
            }
            RtpEventKind::ConfirmPayment(_) => {
                formatter.write_fmt(format_args!("{}: confirm_payment", "event".bright_cyan()))?;
            }
        }
        formatter.write_fmt(format_args!("\n{}: rtp", "standard".bright_cyan(),))?;
        formatter.write_fmt(format_args!(
            "\n{}: {}",
            "version".bright_cyan(),
            self.version
        ))?;
        match &self.event_kind {
            RtpEventKind::NewBank(data) => {
                formatter.write_fmt(format_args!("\n{}: {:?}", "data".bright_cyan(), data))?;
            }
            RtpEventKind::SendTrade(data) => {
                formatter.write_fmt(format_args!("\n{}: {:?}", "data".bright_cyan(), data))?;
            }
            RtpEventKind::SettleTrade(data) => {
                formatter.write_fmt(format_args!("\n{}: {:?}", "data".bright_cyan(), data))?;
            }
            RtpEventKind::ConfirmPayment(data) => {
                formatter.write_fmt(format_args!("\n{}: {:?}", "data".bright_cyan(), data))?;
            }
        }
        Ok(())
    }
}
