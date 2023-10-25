use super::{event, log_tx_result};
use near_workspaces::{
    result::{ExecutionResult, Value},
    types::Balance,
    Account, Contract,
};
use rtp_common::{Outcome, Trade};

pub async fn new(contract: &Contract, sender: &Account) -> anyhow::Result<ExecutionResult<Value>> {
    let (res, _): (ExecutionResult<Value>, Vec<event::ContractEvent>) = log_tx_result(
        Some("new"),
        sender
            .call(contract.id(), "new")
            .max_gas()
            .transact()
            .await?,
    )?;
    Ok(res)
}

pub async fn store_contract(
    contract: &Contract,
    sender: &Account,
    input: Vec<u8>,
) -> anyhow::Result<(ExecutionResult<Value>, Vec<event::ContractEvent>)> {
    let (res, events) = log_tx_result(
        Some("store_contract"),
        sender
            .call(contract.id(), "store_contract")
            .args(input)
            .max_gas()
            .transact()
            .await?,
    )?;
    Ok((res, events))
}

pub async fn create_partnership(
    contract: &Contract,
    bank_a: &str,
    bank_b: &str,
    storage_cost: Balance,
) -> anyhow::Result<(ExecutionResult<Value>, Vec<event::ContractEvent>)> {
    let (res, events) = log_tx_result(
        None,
        contract
            .call("create_partnership")
            .args_json((bank_a, bank_b))
            .deposit(storage_cost)
            .max_gas()
            .transact()
            .await?,
    )?;
    Ok((res, events))
}

pub async fn perform_trade(
    contract: &Contract,
    bank: &str,
    partnership_id: &str,
    trade: &Trade,
) -> anyhow::Result<(ExecutionResult<Value>, Vec<event::ContractEvent>)> {
    let (res, events) = log_tx_result(
        None,
        contract
            .call("perform_trade")
            .args_json((bank, partnership_id, trade))
            .max_gas()
            .transact()
            .await?,
    )?;
    Ok((res, events))
}

pub async fn settle_trade(
    contract: &Contract,
    partnership_id: &str,
    trade_id: &str,
    outcome: &Outcome,
) -> anyhow::Result<(ExecutionResult<Value>, Vec<event::ContractEvent>)> {
    let (res, events) = log_tx_result(
        None,
        contract
            .call("settle_trade")
            .args_json((partnership_id, trade_id, outcome))
            .max_gas()
            .transact()
            .await?,
    )?;
    Ok((res, events))
}
