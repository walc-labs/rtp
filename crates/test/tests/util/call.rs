use super::log_tx_result;
use near_workspaces::{
    result::{ExecutionResult, Value},
    types::NearToken,
    Account, Contract,
};
use rtp_common::ContractEvent;
use rtp_contract_common::{MatchingStatus, TradeDetails};

pub async fn new(contract: &Contract, sender: &Account) -> anyhow::Result<ExecutionResult<Value>> {
    let (res, _): (ExecutionResult<Value>, Vec<ContractEvent>) =
        log_tx_result("new", sender.call(contract.id(), "new").max_gas()).await?;
    Ok(res)
}

pub async fn store_contract(
    contract: &Contract,
    sender: &Account,
    input: Vec<u8>,
) -> anyhow::Result<(ExecutionResult<Value>, Vec<ContractEvent>)> {
    let (res, events) = log_tx_result(
        "store_contract",
        sender
            .call(contract.id(), "store_contract")
            .args(input)
            .max_gas(),
    )
    .await?;
    Ok((res, events))
}

pub async fn clear_storage(
    contract: &Contract,
    sender: &Account,
) -> anyhow::Result<(ExecutionResult<Value>, Vec<ContractEvent>)> {
    let (res, events) = log_tx_result(
        "clear_storage",
        sender.call(contract.id(), "clear_storage").max_gas(),
    )
    .await?;
    Ok((res, events))
}

pub async fn remove_bank(
    contract: &Contract,
    bank_id: &str,
) -> anyhow::Result<(ExecutionResult<Value>, Vec<ContractEvent>)> {
    let (res, events) = log_tx_result(
        "remove_bank",
        contract.call("remove_bank").args_json((bank_id,)).max_gas(),
    )
    .await?;
    Ok((res, events))
}

pub async fn create_bank(
    contract: &Contract,
    bank: &str,
    storage_cost: NearToken,
) -> anyhow::Result<(ExecutionResult<Value>, Vec<ContractEvent>)> {
    let (res, events) = log_tx_result(
        "create_bank",
        contract
            .call("create_bank")
            .args_json((bank,))
            .deposit(storage_cost)
            .max_gas(),
    )
    .await?;
    Ok((res, events))
}

pub async fn perform_trade(
    contract: &Contract,
    bank_id: &str,
    trade_details: &TradeDetails,
) -> anyhow::Result<(ExecutionResult<Value>, Vec<ContractEvent>)> {
    let (res, events) = log_tx_result(
        "perform_trade",
        contract
            .call("perform_trade")
            .args_json((bank_id, trade_details))
            .max_gas(),
    )
    .await?;
    Ok((res, events))
}

pub async fn set_matching_status(
    contract: &Contract,
    partnership_id: &str,
    bank_a_id: &str,
    bank_b_id: &str,
    trade_id: &str,
    matching_status: &MatchingStatus,
) -> anyhow::Result<(ExecutionResult<Value>, Vec<ContractEvent>)> {
    let (res, events) = log_tx_result(
        "set_matching_status",
        contract
            .call("set_matching_status")
            .args_json((
                partnership_id,
                bank_a_id,
                bank_b_id,
                trade_id,
                matching_status,
            ))
            .max_gas(),
    )
    .await?;
    Ok((res, events))
}

pub async fn confirm_payment(
    contract: &Contract,
    creditor_id: &str,
    debitor_id: &str,
    trade_id: &str,
) -> anyhow::Result<(ExecutionResult<Value>, Vec<ContractEvent>)> {
    let (res, events) = log_tx_result(
        "confirm_payment",
        contract
            .call("confirm_payment")
            .args_json((creditor_id, debitor_id, trade_id))
            .max_gas(),
    )
    .await?;
    Ok((res, events))
}
