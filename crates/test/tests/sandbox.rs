#![allow(unused)]

mod util;

#[cfg(not(feature = "testnet"))]
mod sandbox {
    use crate::util::*;
    use near_workspaces::types::NearToken;
    use rtp_contract_common::{
        DealStatus, DealType, Outcome, RtpEventBindgen, Settlement, Side, Speed, Trade,
    };

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

        call::store_contract(&contract, contract.as_account(), RTP_WASM.to_vec()).await?;

        let storage_cost = view::get_partnership_storage_cost(&contract).await?;
        let (_, events) = call::create_partnership(
            &contract,
            &bank_a,
            &bank_b,
            NearToken::from_yoctonear(storage_cost),
        )
        .await?;
        let partnership_id = view::get_partnership_id(&contract, &bank_a, &bank_b).await?;
        assert_event_emits(
            events,
            vec![RtpEventBindgen::NewPartnership { partnership_id }],
        )?;

        Ok(())
    }

    #[tokio::test]
    async fn test_perform_trade_success() -> anyhow::Result<()> {
        let (_, contract) = initialize_contracts().await?;
        let bank_a = "Deutsche Bank".to_string();
        let bank_b = "Sparkasse".to_string();

        call::store_contract(&contract, contract.as_account(), RTP_WASM.to_vec()).await?;

        let storage_cost = view::get_partnership_storage_cost(&contract).await?;
        call::create_partnership(
            &contract,
            &bank_a,
            &bank_b,
            NearToken::from_yoctonear(storage_cost),
        )
        .await?;
        let partnership_id = view::get_partnership_id(&contract, &bank_a, &bank_b).await?;

        let trade = Trade {
            timestamp: 0,
            deal_type: DealType::FxDeal,
            speed: Speed::RealTime,
            contract: "contract".to_string(),
            side: Side::Buy,
            settlement: Settlement::RealTime,
            delivery_date: 0,
            payment_calendars: "payment_calendars".to_string(),
            deal_status: DealStatus::Pending,
            contract_number: "contract_number".to_string(),
            trade_id: "trade_id".to_string(),
        };
        let (_, events) = call::perform_trade(&contract, &bank_a, &partnership_id, &trade).await?;
        assert_event_emits(
            events,
            vec![RtpEventBindgen::SendTrade {
                bank: bank_a,
                trade: trade.clone(),
            }],
        )?;

        let (_, events) = call::perform_trade(&contract, &bank_b, &partnership_id, &trade).await?;
        assert_event_emits(
            events,
            vec![RtpEventBindgen::SendTrade {
                bank: bank_b,
                trade,
            }],
        )?;

        Ok(())
    }

    #[tokio::test]
    async fn test_settle_trade_success() -> anyhow::Result<()> {
        let (_, contract) = initialize_contracts().await?;
        let bank_a = "Deutsche Bank".to_string();
        let bank_b = "Sparkasse".to_string();

        call::store_contract(&contract, contract.as_account(), RTP_WASM.to_vec()).await?;

        let storage_cost = view::get_partnership_storage_cost(&contract).await?;
        call::create_partnership(
            &contract,
            &bank_a,
            &bank_b,
            NearToken::from_yoctonear(storage_cost),
        )
        .await?;
        let partnership_id = view::get_partnership_id(&contract, &bank_a, &bank_b).await?;

        let (_, events) = call::settle_trade(
            &contract,
            &partnership_id,
            "trade_id",
            &Outcome::Success("Trade successfull".to_string()),
        )
        .await?;
        assert_event_emits(
            events,
            vec![RtpEventBindgen::SettleTrade {
                trade_id: "trade_id".to_string(),
                outcome: Outcome::Success("Trade successfull".to_string()),
            }],
        )?;

        Ok(())
    }
}
