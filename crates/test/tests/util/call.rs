use super::{event, log_tx_result};
use near_workspaces::{
    result::{ExecutionResult, Value},
    Account, Contract,
};

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
) -> anyhow::Result<ExecutionResult<Value>> {
    let (res, _) = log_tx_result(
        Some("store_contract"),
        sender
            .call(contract.id(), "store_contract")
            .args(input)
            .max_gas()
            .transact()
            .await?,
    )?;
    Ok(res)
}

pub async fn create_partnership(
    token: &Contract,
    bank_a: &str,
    bank_b: &str,
) -> anyhow::Result<(ExecutionResult<Value>, Vec<event::ContractEvent>)> {
    let (res, events) = log_tx_result(
        None,
        token
            .call("create_partnership")
            .args_json((bank_a, bank_b))
            .transact()
            .await?,
    )?;
    Ok((res, events))
}
