use owo_colors::OwoColorize;
use rtp_contract_common::{MatchingStatus, PaymentConfirmation, PaymentStatus, TradeDetails};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "standard")]
#[serde(rename_all = "kebab-case")]
pub enum ContractEvent {
    Rtp(RtpEvent),
}

pub const KNOWN_EVENT_KINDS: [&str; 5] = [
    "new_bank",
    "send_trade",
    "set_matching_status",
    "confirm_payment",
    "set_payment_status",
];

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
    SetMatchingStatus(SetMatchingStatus),
    ConfirmPayment(ConfirmPayment),
    SetPaymentStatus(SetPaymentStatus),
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
pub struct SetMatchingStatus {
    pub partnership_id: String,
    pub trade_id: String,
    pub matching_status: MatchingStatus,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ConfirmPayment {
    pub partnership_id: String,
    pub bank_id: String,
    pub trade_id: String,
    pub confirmation: PaymentConfirmation,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SetPaymentStatus {
    pub partnership_id: String,
    pub trade_id: String,
    pub payment_status: PaymentStatus,
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
            RtpEventKind::SetMatchingStatus(_) => {
                formatter.write_fmt(format_args!(
                    "{}: set_matching_status",
                    "event".bright_cyan()
                ))?;
            }
            RtpEventKind::SetPaymentStatus(_) => {
                formatter.write_fmt(format_args!(
                    "{}: set_payment_status",
                    "event".bright_cyan()
                ))?;
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
                formatter.write_fmt(format_args!("\n{}: {:#?}", "data".bright_cyan(), data))?;
            }
            RtpEventKind::SendTrade(data) => {
                formatter.write_fmt(format_args!("\n{}: {:#?}", "data".bright_cyan(), data))?;
            }
            RtpEventKind::SetMatchingStatus(data) => {
                formatter.write_fmt(format_args!("\n{}: {:#?}", "data".bright_cyan(), data))?;
            }
            RtpEventKind::SetPaymentStatus(data) => {
                formatter.write_fmt(format_args!("\n{}: {:#?}", "data".bright_cyan(), data))?;
            }
            RtpEventKind::ConfirmPayment(data) => {
                formatter.write_fmt(format_args!("\n{}: {:#?}", "data".bright_cyan(), data))?;
            }
        }
        Ok(())
    }
}
