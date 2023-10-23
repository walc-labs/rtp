pub mod call;
pub mod event;
pub mod view;

use near_workspaces::{
    network::Sandbox,
    result::{ExecutionFinalResult, ExecutionResult, Value, ViewResultDetails},
    types::{KeyType, SecretKey},
    Account, Contract, Worker,
};
use owo_colors::OwoColorize;
use rtp_common::RtpEvent;
use serde::Serialize;
use std::fmt;

#[macro_export]
macro_rules! print_log {
    ( $x:expr, $($y:expr),+ ) => {
        let thread_name = std::thread::current().name().unwrap().to_string();
        if thread_name == "main" {
            println!($x, $($y),+);
        } else {
            let mut s = format!($x, $($y),+);
            s = s.split('\n').map(|s| {
                let mut pre = "    ".to_string();
                pre.push_str(s);
                pre.push('\n');
                pre
            }).collect::<String>();
            println!(
                "{}\n{}",
                thread_name.bold(),
                &s[..s.len() - 1],
            );
        }
    };
}

pub async fn initialize_contracts() -> anyhow::Result<(Worker<Sandbox>, Account, Contract)> {
    let worker = near_workspaces::sandbox().await?;

    let owner = worker.dev_create_account().await?;

    let wasm = include_bytes!("../../../../res/rtp_factory.wasm");

    let key = SecretKey::from_random(KeyType::ED25519);
    let contract = worker
        .create_tla_and_deploy("rtp.test.near".parse()?, key, wasm)
        .await?
        .into_result()?;

    call::new(&contract, contract.as_account()).await?;

    Ok((worker, owner, contract))
}

pub fn log_tx_result(
    ident: Option<&str>,
    res: ExecutionFinalResult,
) -> anyhow::Result<(ExecutionResult<Value>, Vec<event::ContractEvent>)> {
    for failure in res.receipt_failures() {
        print_log!("{:#?}", failure.bright_red());
    }
    let mut events = vec![];
    for outcome in res.receipt_outcomes() {
        if !outcome.logs.is_empty() {
            for log in outcome.logs.iter() {
                if log.starts_with("EVENT_JSON:") {
                    let event: event::ContractEvent =
                        serde_json::from_str(&log.replace("EVENT_JSON:", ""))?;
                    events.push(event.clone());
                    print_log!(
                        "{}: {}\n{}",
                        "account".bright_cyan(),
                        outcome.executor_id,
                        event
                    );
                } else {
                    print_log!("{}", log.bright_yellow());
                }
            }
        }
    }
    if let Some(ident) = ident {
        print_log!(
            "{} gas burnt: {:.3} {}",
            ident.italic(),
            res.total_gas_burnt.as_tgas().bright_magenta().bold(),
            "TGas".bright_magenta().bold()
        );
    }
    Ok((res.into_result()?, events))
}

pub fn log_view_result(res: ViewResultDetails) -> anyhow::Result<ViewResultDetails> {
    if !res.logs.is_empty() {
        for log in res.logs.iter() {
            print_log!("{}", log.bright_yellow());
        }
    }
    Ok(res)
}

pub fn assert_event_emits<T>(actual: T, events: Vec<RtpEvent>) -> anyhow::Result<()>
where
    T: Serialize + fmt::Debug + Clone,
{
    let mut actual = serde_json::to_value(&actual)?;
    actual.as_array_mut().unwrap().retain(|ac| {
        let event_str = ac
            .as_object()
            .unwrap()
            .get("event")
            .unwrap()
            .as_str()
            .unwrap();
        event::KNOWN_EVENT_KINDS.contains(&event_str)
    });
    let mut expected = vec![];
    for event in events {
        let mut expected_event = serde_json::to_value(event)?;
        let ev = expected_event.as_object_mut().unwrap();
        let event_str = ev.get("event").unwrap().as_str().unwrap();
        if !event::KNOWN_EVENT_KINDS.contains(&event_str) {
            continue;
        }
        ev.insert("standard".into(), "rtp".into());
        ev.insert("version".into(), "1.0.0".into());
        expected.push(expected_event);
    }
    assert_eq!(
        &actual,
        &serde_json::to_value(&expected)?,
        "actual and expected events did not match.\nActual: {:#?}\nExpected: {:#?}",
        &actual,
        &expected
    );
    Ok(())
}
