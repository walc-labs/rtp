mod util;

use rtp_common::RtpEvent;
use util::*;

const RTP_WASM: &[u8] = include_bytes!("../../../res/rtp.wasm");

#[tokio::test]
async fn test_store_contract() -> anyhow::Result<()> {
    let (_, contract) = initialize_contracts().await?;

    call::store_contract(&contract, contract.as_account(), RTP_WASM.to_vec()).await?;

    Ok(())
}

#[tokio::test]
async fn test_create_partnership() -> anyhow::Result<()> {
    let (_, contract) = initialize_contracts().await?;
    let bank_a = "Deutsche Bank".to_string();
    let bank_b = "Sparkasse".to_string();
    let factory_account_id = contract.id();

    call::store_contract(&contract, contract.as_account(), RTP_WASM.to_vec()).await?;

    let storage_cost = view::get_partnership_storage_cost(&contract).await?;
    let (_, events) = call::create_partnership(&contract, &bank_a, &bank_b, storage_cost).await?;
    let partnership_id = view::get_partnership_id(&contract, &bank_a, &bank_b).await?;
    let partnership_id = format!("{partnership_id}.{factory_account_id}").parse()?;
    assert_event_emits(events, vec![RtpEvent::NewPartnership { partnership_id }])?;

    Ok(())
}
