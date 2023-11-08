use super::log_view_result;
use near_sdk::Balance;
use near_workspaces::{AccountId, Contract};

pub async fn get_partnership_storage_cost(contract: &Contract) -> anyhow::Result<Balance> {
    let res = log_view_result(
        contract
            .call("get_partnership_storage_cost")
            .max_gas()
            .view()
            .await?,
    )?;
    Ok(res.json()?)
}

pub async fn get_partnerships(
    contract: &Contract,
    skip: Option<u32>,
    limit: Option<u32>,
) -> anyhow::Result<Vec<String>> {
    let res = log_view_result(
        contract
            .call("get_partnerships")
            .args_json((skip, limit))
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
