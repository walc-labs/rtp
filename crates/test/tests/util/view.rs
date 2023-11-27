use super::log_view_result;
use near_sdk::Balance;
use near_workspaces::{network::NetworkClient, AccountId, Contract, Worker};
use rtp_contract_common::Trade;

pub async fn get_bank_storage_cost(contract: &Contract) -> anyhow::Result<Balance> {
    let res = log_view_result(
        contract
            .call("get_bank_storage_cost")
            .max_gas()
            .view()
            .await?,
    )?;
    Ok(res.json()?)
}

pub async fn get_bank_ids(
    contract: &Contract,
    skip: Option<u32>,
    limit: Option<u32>,
) -> anyhow::Result<Vec<String>> {
    let res = log_view_result(
        contract
            .call("get_bank_ids")
            .args_json((skip, limit))
            .max_gas()
            .view()
            .await?,
    )?;
    Ok(res.json()?)
}

pub async fn get_bank_id(contract: &Contract, bank: &str) -> anyhow::Result<String> {
    let res = log_view_result(
        contract
            .call("get_bank_id")
            .args_json((bank,))
            .max_gas()
            .view()
            .await?,
    )?;
    Ok(res.json()?)
}

pub async fn get_partnership_id(
    contract: &Contract,
    bank_a: &str,
    bank_b: &str,
) -> anyhow::Result<String> {
    let res = log_view_result(
        contract
            .call("get_partnership_id")
            .args_json((bank_a, bank_b))
            .max_gas()
            .view()
            .await?,
    )?;
    Ok(res.json()?)
}

pub async fn get_trade<T: ?Sized + NetworkClient>(
    worker: &Worker<T>,
    contract_id: &AccountId,
    trade_id: &str,
) -> anyhow::Result<Trade> {
    let res = log_view_result(
        worker
            .view(contract_id, "get_trade")
            .args_json((trade_id,))
            .await?,
    )?;
    Ok(res.json()?)
}
