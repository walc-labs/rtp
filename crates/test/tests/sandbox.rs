#![allow(unused)]

mod util;

#[cfg(not(feature = "testnet"))]
mod sandbox {
    use crate::util::*;
    use near_workspaces::types::NearToken;
    use rtp_contract_common::{
        DealType, MatchingStatus, Product, RtpEvent, Settlement, Side, TradeDetails,
    };

    const RTP_WASM: &[u8] = include_bytes!("../../../res/rtp.wasm");

    #[tokio::test]
    async fn test_store_contract() -> anyhow::Result<()> {
        let (_, contract) = initialize_contracts().await?;

        call::store_contract(&contract, contract.as_account(), RTP_WASM.to_vec()).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_create_bank() -> anyhow::Result<()> {
        let (_, contract) = initialize_contracts().await?;
        let bank_a = "Deutsche Bank".to_string();
        let bank_b = "Sparkasse".to_string();

        call::store_contract(&contract, contract.as_account(), RTP_WASM.to_vec()).await?;

        let storage_cost = view::get_bank_storage_cost(&contract).await?;
        dbg!(&storage_cost);
        let (_, events) =
            call::create_bank(&contract, &bank_a, NearToken::from_yoctonear(storage_cost)).await?;
        let bank_id = view::get_bank_id(&contract, &bank_a).await?;
        assert_event_emits(
            events,
            vec![RtpEvent::NewBank {
                bank: bank_a,
                bank_id,
            }],
        )?;

        Ok(())
    }

    #[tokio::test]
    async fn test_perform_trade_success() -> anyhow::Result<()> {
        let (_, contract) = initialize_contracts().await?;
        let bank_a = "Deutsche Bank".to_string();
        let bank_b = "Sparkasse".to_string();

        call::store_contract(&contract, contract.as_account(), RTP_WASM.to_vec()).await?;

        let storage_cost = view::get_bank_storage_cost(&contract).await?;
        call::create_bank(&contract, &bank_a, NearToken::from_yoctonear(storage_cost)).await?;
        call::create_bank(&contract, &bank_b, NearToken::from_yoctonear(storage_cost)).await?;
        let bank_a_id = view::get_bank_id(&contract, &bank_a).await?;
        let bank_b_id = view::get_bank_id(&contract, &bank_b).await?;
        let partnership_id =
            view::get_partnership_id(&contract, bank_a.as_str(), bank_b.as_str()).await?;

        let mut trade = TradeDetails {
            trade_id: "trade_id".to_string(),
            timestamp: 0,
            deal_type: DealType::FxDeal,
            product: Product::Spot,
            contract: "contract".to_string(),
            counterparty: bank_b.clone(),
            instrument_id: "EUR_USD".to_string(),
            amount: "1".to_string(),
            price: "2".to_string(),
            side: Side::Buy,
            settlement: Settlement::RealTime,
            delivery_date: 0,
            payment_calendars: "payment_calendars".to_string(),
            contract_number: "contract_number".to_string(),
        };
        let (_, events) = call::perform_trade(&contract, &bank_a_id, &trade).await?;
        assert_event_emits(
            events,
            vec![RtpEvent::SendTrade {
                partnership_id: partnership_id.clone(),
                bank_id: bank_a_id,
                trade: trade.clone(),
            }],
        )?;

        trade.counterparty = bank_a;
        let (_, events) = call::perform_trade(&contract, &bank_b_id, &trade).await?;
        assert_event_emits(
            events,
            vec![RtpEvent::SendTrade {
                partnership_id,
                bank_id: bank_b_id,
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

        let storage_cost = view::get_bank_storage_cost(&contract).await?;
        call::create_bank(&contract, &bank_a, NearToken::from_yoctonear(storage_cost)).await?;
        call::create_bank(&contract, &bank_b, NearToken::from_yoctonear(storage_cost)).await?;
        let bank_a_id = view::get_bank_id(&contract, &bank_a).await?;
        let bank_b_id = view::get_bank_id(&contract, &bank_b).await?;
        let partnership_id =
            view::get_partnership_id(&contract, bank_a.as_str(), bank_b.as_str()).await?;

        let mut trade = TradeDetails {
            trade_id: "trade_id".to_string(),
            timestamp: 0,
            deal_type: DealType::FxDeal,
            product: Product::Spot,
            contract: "contract".to_string(),
            counterparty: bank_b,
            instrument_id: "EUR_USD".to_string(),
            amount: "1".to_string(),
            price: "2".to_string(),
            side: Side::Buy,
            settlement: Settlement::RealTime,
            delivery_date: 0,
            payment_calendars: "payment_calendars".to_string(),
            contract_number: "contract_number".to_string(),
        };
        call::perform_trade(&contract, &bank_a_id, &trade).await?;
        trade.counterparty = bank_a;
        call::perform_trade(&contract, &bank_b_id, &trade).await?;

        let (_, events) = call::set_matching_status(
            &contract,
            &partnership_id,
            &bank_a_id,
            &bank_b_id,
            "trade_id",
            &MatchingStatus::Confirmed("Trade successfull".to_string()),
        )
        .await?;
        assert_event_emits(
            events,
            vec![RtpEvent::SetMatchingStatus {
                partnership_id,
                trade_id: "trade_id".to_string(),
                matching_status: MatchingStatus::Confirmed("Trade successfull".to_string()),
            }],
        )?;

        Ok(())
    }

    #[tokio::test]
    async fn test_confirm_payment() -> anyhow::Result<()> {
        let (worker, factory) = initialize_contracts().await?;
        let bank_a = "Deutsche Bank".to_string();
        let bank_b = "Sparkasse".to_string();

        call::store_contract(&factory, factory.as_account(), RTP_WASM.to_vec()).await?;

        let storage_cost = view::get_bank_storage_cost(&factory).await?;
        call::create_bank(&factory, &bank_a, NearToken::from_yoctonear(storage_cost)).await?;
        call::create_bank(&factory, &bank_b, NearToken::from_yoctonear(storage_cost)).await?;
        let bank_a_id = view::get_bank_id(&factory, &bank_a).await?;
        let bank_b_id = view::get_bank_id(&factory, &bank_b).await?;
        let partnership_id =
            view::get_partnership_id(&factory, bank_a.as_str(), bank_b.as_str()).await?;

        let mut trade = TradeDetails {
            trade_id: "trade_id".to_string(),
            timestamp: 0,
            deal_type: DealType::FxDeal,
            product: Product::Spot,
            contract: "contract".to_string(),
            counterparty: bank_b.clone(),
            instrument_id: "EUR_USD".to_string(),
            amount: "1".to_string(),
            price: "2".to_string(),
            side: Side::Buy,
            settlement: Settlement::RealTime,
            delivery_date: 0,
            payment_calendars: "payment_calendars".to_string(),
            contract_number: "contract_number".to_string(),
        };
        call::perform_trade(&factory, &bank_a_id, &trade).await?;
        trade.counterparty = bank_a.clone();
        call::perform_trade(&factory, &bank_b_id, &trade).await?;

        let (_, events) = call::set_matching_status(
            &factory,
            &partnership_id,
            &bank_a_id,
            &bank_b_id,
            "trade_id",
            &MatchingStatus::Confirmed("Trade successfull".to_string()),
        )
        .await?;

        call::confirm_payment(&factory, &bank_a_id, &bank_b_id, "trade_id").await?;
        call::confirm_payment(&factory, &bank_b_id, &bank_a_id, "trade_id").await?;

        let bank_a_id = view::get_bank_id(&factory, &bank_a).await?;
        let account_a_id = format!("{bank_a_id}.{}", factory.id()).parse()?;
        let bank_b_id = view::get_bank_id(&factory, &bank_b).await?;
        let account_b_id = format!("{bank_b_id}.{}", factory.id()).parse()?;

        let trade_a = view::get_trade(&worker, &account_a_id, "trade_id").await?;
        let trade_b = view::get_trade(&worker, &account_b_id, "trade_id").await?;

        assert!(trade_a.payments.credit);
        assert!(trade_a.payments.debit);
        assert!(trade_b.payments.credit);
        assert!(trade_b.payments.debit);

        Ok(())
    }
}
