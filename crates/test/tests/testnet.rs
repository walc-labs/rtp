#![allow(unused)]

mod config;
mod util;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Credentials {
    pub account_id: String,
    pub public_key: String,
    pub private_key: String,
}

#[cfg(feature = "testnet")]
mod testnet {
    use crate::{config::Config, print_log, util::*, Credentials};
    use near_workspaces::{
        network::{Custom, Testnet, TopLevelAccountCreator},
        types::{KeyType, NearToken, SecretKey},
        Account, AccountId, Contract, DevNetwork, Worker,
    };
    use owo_colors::OwoColorize;
    use rtp_contract_common::{
        MatchingStatus, PaymentStatus, Product, Settlement, Side, TradeDetails,
    };
    use serde_json::Value;
    use std::{
        env,
        path::PathBuf,
        thread,
        time::{Duration, SystemTime, UNIX_EPOCH},
    };
    use test_case::test_case;
    use tokio::{fs::File, io::AsyncWriteExt};

    const RTP_FACTORY_WASM: &[u8] = include_bytes!("../../../res/rtp_factory.wasm");
    const RTP_WASM: &[u8] = include_bytes!("../../../res/rtp.wasm");

    #[test_case(TradeDetails {
        product: Product::Spot,
        agreement: "Bilateral".to_string(),
        ..Default::default()
    }; "spot")]
    #[test_case(TradeDetails {
        product: Product::Ndf,
        agreement: "Bilateral".to_string(),
        ..Default::default()
    }; "ndf")]
    #[test_case(TradeDetails {
        product: Product::Fwd,
        agreement: "Market".to_string(),
        ..Default::default()
    }; "fwd")]
    #[test_case(TradeDetails {
        product: Product::Swap,
        agreement: "Market".to_string(),
        ..TradeDetails::default_swap()
    }; "swap")]
    #[tokio::test]
    async fn test_settle_trade_basic(mut trade_details: TradeDetails) -> anyhow::Result<()> {
        dotenv::dotenv();

        // let worker = near_workspaces::testnet_with_rpc("https://near-testnet.api.pagoda.co/rpc/v1/", &env::var("RPC_API_KEY").unwrap()).await?;
        let worker = near_workspaces::testnet().await?;
        let config = Config::new();

        let factory = deploy_contract(&worker, &config).await?;

        let bank_a = "Deutsche Bank".to_string();
        let bank_b = "Sparkasse".to_string();

        call::store_contract(&factory, factory.as_account(), RTP_WASM.to_vec()).await?;

        let storage_cost = view::get_bank_storage_cost(&factory).await?;
        call::create_bank(&factory, &bank_a, NearToken::from_yoctonear(storage_cost)).await?;
        call::create_bank(&factory, &bank_b, NearToken::from_yoctonear(storage_cost)).await?;
        let bank_a_id = view::get_bank_id(&factory, &bank_a).await?;
        let bank_b_id = view::get_bank_id(&factory, &bank_b).await?;
        let account_a_id: AccountId = format!("{bank_a_id}.{}", factory.id()).parse()?;
        let account_b_id: AccountId = format!("{bank_b_id}.{}", factory.id()).parse()?;

        thread::sleep(Duration::from_secs(5));

        trade_details.side = Side::Buy;
        trade_details.counterparty = bank_b;
        call::perform_trade(&factory, &bank_a_id, &trade_details).await?;
        trade_details.side = Side::Sell;
        trade_details.counterparty = bank_a;
        call::perform_trade(&factory, &bank_b_id, &trade_details).await?;

        thread::sleep(Duration::from_secs(15));

        assert_trade_matching_status(
            &worker,
            &account_a_id,
            "trade_id",
            &MatchingStatus::Confirmed("".to_string()),
        )
        .await?;
        assert_trade_matching_status(
            &worker,
            &account_b_id,
            "trade_id",
            &MatchingStatus::Confirmed("".to_string()),
        )
        .await?;

        call::confirm_payment(&factory, &bank_a_id, &bank_b_id, "trade_id").await?;
        call::confirm_payment(&factory, &bank_b_id, &bank_a_id, "trade_id").await?;

        thread::sleep(Duration::from_secs(15));

        assert_trade_payment_status(
            &worker,
            &account_a_id,
            "trade_id",
            &PaymentStatus::Confirmed("".to_string()),
        )
        .await?;
        assert_trade_payment_status(
            &worker,
            &account_b_id,
            "trade_id",
            &PaymentStatus::Confirmed("".to_string()),
        )
        .await?;

        Ok(())
    }

    #[test_case(TradeDetails {
        product: Product::Spot,
        agreement: "Bilateral".to_string(),
        ..Default::default()
    }; "spot")]
    #[test_case(TradeDetails {
        product: Product::Ndf,
        agreement: "Bilateral".to_string(),
        ..Default::default()
    }; "ndf")]
    #[test_case(TradeDetails {
        product: Product::Fwd,
        agreement: "Market".to_string(),
        ..Default::default()
    }; "fwd")]
    #[test_case(TradeDetails {
        product: Product::Swap,
        agreement: "Market".to_string(),
        ..TradeDetails::default_swap()
    }; "swap")]
    #[tokio::test]
    async fn test_settle_trade_complex(mut trade_details: TradeDetails) -> anyhow::Result<()> {
        dotenv::dotenv();

        // let worker = near_workspaces::testnet_with_rpc("https://near-testnet.api.pagoda.co/rpc/v1/", API_KEY).await?;
        let worker = near_workspaces::testnet().await?;
        let config = Config::new();

        let factory = deploy_contract(&worker, &config).await?;

        let bank_a = "Deutsche Bank".to_string();
        let bank_b = "Sparkasse".to_string();
        let bank_c = "JPMorgan".to_string();
        let bank_d = "Wells Fargo".to_string();

        call::store_contract(&factory, factory.as_account(), RTP_WASM.to_vec()).await?;

        let storage_cost = view::get_bank_storage_cost(&factory).await?;
        call::create_bank(&factory, &bank_a, NearToken::from_yoctonear(storage_cost)).await?;
        call::create_bank(&factory, &bank_b, NearToken::from_yoctonear(storage_cost)).await?;
        call::create_bank(&factory, &bank_c, NearToken::from_yoctonear(storage_cost)).await?;
        call::create_bank(&factory, &bank_d, NearToken::from_yoctonear(storage_cost)).await?;
        let bank_a_id = view::get_bank_id(&factory, &bank_a).await?;
        let bank_b_id = view::get_bank_id(&factory, &bank_b).await?;
        let bank_c_id = view::get_bank_id(&factory, &bank_c).await?;
        let bank_d_id = view::get_bank_id(&factory, &bank_d).await?;
        let account_a_id: AccountId = format!("{bank_a_id}.{}", factory.id()).parse()?;
        let account_b_id: AccountId = format!("{bank_b_id}.{}", factory.id()).parse()?;
        let account_c_id: AccountId = format!("{bank_c_id}.{}", factory.id()).parse()?;
        let account_d_id: AccountId = format!("{bank_d_id}.{}", factory.id()).parse()?;

        thread::sleep(Duration::from_secs(5));

        {
            let bank_a = bank_a.clone();
            let bank_b = bank_b.clone();
            trade_details.trade_id = format!("{bank_a}:{bank_b}");
            trade_details.side = Side::Buy;
            trade_details.counterparty = bank_b;
            call::perform_trade(&factory, &bank_a_id, &trade_details).await?;
            trade_details.side = Side::Sell;
            trade_details.counterparty = bank_a;
            call::perform_trade(&factory, &bank_b_id, &trade_details).await?;
        }
        {
            let bank_a = bank_a.clone();
            let bank_c = bank_c.clone();
            trade_details.trade_id = format!("{bank_a}:{bank_c}");
            trade_details.side = Side::Buy;
            trade_details.counterparty = bank_c;
            call::perform_trade(&factory, &bank_a_id, &trade_details).await?;
            trade_details.side = Side::Sell;
            trade_details.counterparty = bank_a;
            call::perform_trade(&factory, &bank_c_id, &trade_details).await?;
        }
        {
            let bank_a = bank_a.clone();
            let bank_d = bank_d.clone();
            trade_details.trade_id = format!("{bank_a}:{bank_d}");
            trade_details.side = Side::Buy;
            trade_details.counterparty = bank_d;
            call::perform_trade(&factory, &bank_a_id, &trade_details).await?;
            trade_details.side = Side::Sell;
            trade_details.counterparty = bank_a;
            call::perform_trade(&factory, &bank_d_id, &trade_details).await?;
        }
        {
            let bank_b = bank_b.clone();
            let bank_c = bank_c.clone();
            trade_details.trade_id = format!("{bank_b}:{bank_c}");
            trade_details.side = Side::Buy;
            trade_details.counterparty = bank_c;
            call::perform_trade(&factory, &bank_b_id, &trade_details).await?;
            trade_details.side = Side::Sell;
            trade_details.counterparty = bank_b;
            call::perform_trade(&factory, &bank_c_id, &trade_details).await?;
        }
        {
            let bank_b = bank_b.clone();
            let bank_d = bank_d.clone();
            trade_details.trade_id = format!("{bank_b}:{bank_d}");
            trade_details.side = Side::Buy;
            trade_details.counterparty = bank_d;
            call::perform_trade(&factory, &bank_b_id, &trade_details).await?;
            trade_details.side = Side::Sell;
            trade_details.counterparty = bank_b;
            call::perform_trade(&factory, &bank_d_id, &trade_details).await?;
        }
        {
            let bank_c = bank_c.clone();
            let bank_d = bank_d.clone();
            trade_details.trade_id = format!("{bank_c}:{bank_d}");
            trade_details.side = Side::Buy;
            trade_details.counterparty = bank_d;
            call::perform_trade(&factory, &bank_c_id, &trade_details).await?;
            trade_details.side = Side::Sell;
            trade_details.counterparty = bank_c;
            call::perform_trade(&factory, &bank_d_id, &trade_details).await?;
        }

        thread::sleep(Duration::from_secs(15));

        {
            let trade_id = format!("{bank_a}:{bank_b}");
            assert_trade_matching_status(
                &worker,
                &account_a_id,
                &trade_id,
                &MatchingStatus::Confirmed("".to_string()),
            )
            .await?;
            assert_trade_matching_status(
                &worker,
                &account_b_id,
                &trade_id,
                &MatchingStatus::Confirmed("".to_string()),
            )
            .await?;

            call::confirm_payment(&factory, &bank_a_id, &bank_b_id, &trade_id).await?;
            call::confirm_payment(&factory, &bank_b_id, &bank_a_id, &trade_id).await?;
        }
        {
            let trade_id = format!("{bank_a}:{bank_c}");
            assert_trade_matching_status(
                &worker,
                &account_a_id,
                &trade_id,
                &MatchingStatus::Confirmed("".to_string()),
            )
            .await?;
            assert_trade_matching_status(
                &worker,
                &account_c_id,
                &trade_id,
                &MatchingStatus::Confirmed("".to_string()),
            )
            .await?;

            call::confirm_payment(&factory, &bank_a_id, &bank_c_id, &trade_id).await?;
            call::confirm_payment(&factory, &bank_c_id, &bank_a_id, &trade_id).await?;
        }
        {
            let trade_id = format!("{bank_a}:{bank_d}");
            assert_trade_matching_status(
                &worker,
                &account_a_id,
                &trade_id,
                &MatchingStatus::Confirmed("".to_string()),
            )
            .await?;
            assert_trade_matching_status(
                &worker,
                &account_d_id,
                &trade_id,
                &MatchingStatus::Confirmed("".to_string()),
            )
            .await?;

            call::confirm_payment(&factory, &bank_a_id, &bank_d_id, &trade_id).await?;
            call::confirm_payment(&factory, &bank_d_id, &bank_a_id, &trade_id).await?;
        }
        {
            let trade_id = format!("{bank_b}:{bank_c}");
            assert_trade_matching_status(
                &worker,
                &account_b_id,
                &trade_id,
                &MatchingStatus::Confirmed("".to_string()),
            )
            .await?;
            assert_trade_matching_status(
                &worker,
                &account_c_id,
                &trade_id,
                &MatchingStatus::Confirmed("".to_string()),
            )
            .await?;

            call::confirm_payment(&factory, &bank_b_id, &bank_c_id, &trade_id).await?;
            call::confirm_payment(&factory, &bank_c_id, &bank_b_id, &trade_id).await?;
        }
        {
            let trade_id = format!("{bank_b}:{bank_d}");
            assert_trade_matching_status(
                &worker,
                &account_b_id,
                &trade_id,
                &MatchingStatus::Confirmed("".to_string()),
            )
            .await?;
            assert_trade_matching_status(
                &worker,
                &account_d_id,
                &trade_id,
                &MatchingStatus::Confirmed("".to_string()),
            )
            .await?;

            call::confirm_payment(&factory, &bank_b_id, &bank_d_id, &trade_id).await?;
            call::confirm_payment(&factory, &bank_d_id, &bank_b_id, &trade_id).await?;
        }
        {
            let trade_id = format!("{bank_c}:{bank_d}");
            assert_trade_matching_status(
                &worker,
                &account_c_id,
                &trade_id,
                &MatchingStatus::Confirmed("".to_string()),
            )
            .await?;
            assert_trade_matching_status(
                &worker,
                &account_d_id,
                &trade_id,
                &MatchingStatus::Confirmed("".to_string()),
            )
            .await?;

            call::confirm_payment(&factory, &bank_c_id, &bank_d_id, &trade_id).await?;
            call::confirm_payment(&factory, &bank_d_id, &bank_c_id, &trade_id).await?;
        }

        thread::sleep(Duration::from_secs(15));

        {
            let trade_id = format!("{bank_a}:{bank_b}");
            assert_trade_payment_status(
                &worker,
                &account_a_id,
                &trade_id,
                &PaymentStatus::Confirmed("".to_string()),
            )
            .await?;
            assert_trade_payment_status(
                &worker,
                &account_b_id,
                &trade_id,
                &PaymentStatus::Confirmed("".to_string()),
            )
            .await?;
        }
        {
            let trade_id = format!("{bank_a}:{bank_c}");
            assert_trade_payment_status(
                &worker,
                &account_a_id,
                &trade_id,
                &PaymentStatus::Confirmed("".to_string()),
            )
            .await?;
            assert_trade_payment_status(
                &worker,
                &account_c_id,
                &trade_id,
                &PaymentStatus::Confirmed("".to_string()),
            )
            .await?;
        }
        {
            let trade_id = format!("{bank_a}:{bank_d}");
            assert_trade_payment_status(
                &worker,
                &account_a_id,
                &trade_id,
                &PaymentStatus::Confirmed("".to_string()),
            )
            .await?;
            assert_trade_payment_status(
                &worker,
                &account_d_id,
                &trade_id,
                &PaymentStatus::Confirmed("".to_string()),
            )
            .await?;
        }
        {
            let trade_id = format!("{bank_b}:{bank_c}");
            assert_trade_payment_status(
                &worker,
                &account_b_id,
                &trade_id,
                &PaymentStatus::Confirmed("".to_string()),
            )
            .await?;
            assert_trade_payment_status(
                &worker,
                &account_c_id,
                &trade_id,
                &PaymentStatus::Confirmed("".to_string()),
            )
            .await?;
        }
        {
            let trade_id = format!("{bank_b}:{bank_d}");
            assert_trade_payment_status(
                &worker,
                &account_b_id,
                &trade_id,
                &PaymentStatus::Confirmed("".to_string()),
            )
            .await?;
            assert_trade_payment_status(
                &worker,
                &account_d_id,
                &trade_id,
                &PaymentStatus::Confirmed("".to_string()),
            )
            .await?;
        }
        {
            let trade_id = format!("{bank_c}:{bank_d}");
            assert_trade_payment_status(
                &worker,
                &account_c_id,
                &trade_id,
                &PaymentStatus::Confirmed("".to_string()),
            )
            .await?;
            assert_trade_payment_status(
                &worker,
                &account_d_id,
                &trade_id,
                &PaymentStatus::Confirmed("".to_string()),
            )
            .await?;
        }

        Ok(())
    }

    #[test_case(TradeDetails {
        product: Product::Spot,
        agreement: "Bilateral".to_string(),
        ..Default::default()
    }; "spot")]
    #[test_case(TradeDetails {
        product: Product::Ndf,
        agreement: "Bilateral".to_string(),
        ..Default::default()
    }; "ndf")]
    #[test_case(TradeDetails {
        product: Product::Fwd,
        agreement: "Market".to_string(),
        ..Default::default()
    }; "fwd")]
    #[test_case(TradeDetails {
        product: Product::Swap,
        agreement: "Market".to_string(),
        ..TradeDetails::default_swap()
    }; "swap")]
    #[tokio::test]
    async fn test_settle_trade_fail_match(mut trade_details: TradeDetails) -> anyhow::Result<()> {
        dotenv::dotenv();

        // let worker = near_workspaces::testnet_with_rpc("https://near-testnet.api.pagoda.co/rpc/v1/", API_KEY).await?;
        let worker = near_workspaces::testnet().await?;
        let config = Config::new();

        let factory = deploy_contract(&worker, &config).await?;

        let bank_a = "Deutsche Bank".to_string();
        let bank_b = "Sparkasse".to_string();

        call::store_contract(&factory, factory.as_account(), RTP_WASM.to_vec()).await?;

        let storage_cost = view::get_bank_storage_cost(&factory).await?;
        call::create_bank(&factory, &bank_a, NearToken::from_yoctonear(storage_cost)).await?;
        call::create_bank(&factory, &bank_b, NearToken::from_yoctonear(storage_cost)).await?;
        let bank_a_id = view::get_bank_id(&factory, &bank_a).await?;
        let bank_b_id = view::get_bank_id(&factory, &bank_b).await?;
        let account_a_id: AccountId = format!("{bank_a_id}.{}", factory.id()).parse()?;
        let account_b_id: AccountId = format!("{bank_b_id}.{}", factory.id()).parse()?;

        thread::sleep(Duration::from_secs(5));

        trade_details.side = Side::Buy;
        trade_details.counterparty = bank_b;
        call::perform_trade(&factory, &bank_a_id, &trade_details).await?;
        trade_details.side = Side::Sell;
        trade_details.counterparty = bank_a;
        // changing this value will make the trade fail
        trade_details.price = 3.;
        call::perform_trade(&factory, &bank_b_id, &trade_details).await?;

        thread::sleep(Duration::from_secs(15));

        assert_trade_matching_status(
            &worker,
            &account_a_id,
            "trade_id",
            &MatchingStatus::Rejected("".to_string()),
        )
        .await?;
        assert_trade_matching_status(
            &worker,
            &account_b_id,
            "trade_id",
            &MatchingStatus::Rejected("".to_string()),
        )
        .await?;

        call::confirm_payment(&factory, &bank_a_id, &bank_b_id, "trade_id").await?;
        call::confirm_payment(&factory, &bank_b_id, &bank_a_id, "trade_id").await?;

        thread::sleep(Duration::from_secs(15));

        assert_trade_payment_status(&worker, &account_a_id, "trade_id", &PaymentStatus::Pending)
            .await?;
        assert_trade_payment_status(&worker, &account_b_id, "trade_id", &PaymentStatus::Pending)
            .await?;

        Ok(())
    }

    #[test_case(TradeDetails {
        product: Product::Spot,
        agreement: "Bilateral".to_string(),
        ..Default::default()
    }; "spot")]
    #[test_case(TradeDetails {
        product: Product::Ndf,
        agreement: "Bilateral".to_string(),
        ..Default::default()
    }; "ndf")]
    #[test_case(TradeDetails {
        product: Product::Fwd,
        agreement: "Market".to_string(),
        ..Default::default()
    }; "fwd")]
    #[test_case(TradeDetails {
        product: Product::Swap,
        agreement: "Market".to_string(),
        ..TradeDetails::default_swap()
    }; "swap")]
    #[tokio::test]
    async fn test_settle_trade_fail_payment(mut trade_details: TradeDetails) -> anyhow::Result<()> {
        dotenv::dotenv();

        // let worker = near_workspaces::testnet_with_rpc("https://near-testnet.api.pagoda.co/rpc/v1/", API_KEY).await?;
        let worker = near_workspaces::testnet().await?;
        let config = Config::new();

        let factory = deploy_contract(&worker, &config).await?;

        let bank_a = "Deutsche Bank".to_string();
        let bank_b = "Sparkasse".to_string();

        call::store_contract(&factory, factory.as_account(), RTP_WASM.to_vec()).await?;

        let storage_cost = view::get_bank_storage_cost(&factory).await?;
        call::create_bank(&factory, &bank_a, NearToken::from_yoctonear(storage_cost)).await?;
        call::create_bank(&factory, &bank_b, NearToken::from_yoctonear(storage_cost)).await?;
        let bank_a_id = view::get_bank_id(&factory, &bank_a).await?;
        let bank_b_id = view::get_bank_id(&factory, &bank_b).await?;
        let account_a_id: AccountId = format!("{bank_a_id}.{}", factory.id()).parse()?;
        let account_b_id: AccountId = format!("{bank_b_id}.{}", factory.id()).parse()?;

        thread::sleep(Duration::from_secs(5));

        trade_details.side = Side::Buy;
        trade_details.counterparty = bank_b;
        call::perform_trade(&factory, &bank_a_id, &trade_details).await?;
        trade_details.side = Side::Sell;
        trade_details.counterparty = bank_a;
        call::perform_trade(&factory, &bank_b_id, &trade_details).await?;

        thread::sleep(Duration::from_secs(15));

        assert_trade_matching_status(
            &worker,
            &account_a_id,
            "trade_id",
            &MatchingStatus::Confirmed("".to_string()),
        )
        .await?;
        assert_trade_matching_status(
            &worker,
            &account_b_id,
            "trade_id",
            &MatchingStatus::Confirmed("".to_string()),
        )
        .await?;

        call::confirm_payment(&factory, &bank_a_id, &bank_b_id, "trade_id").await?;
        // 2nd party doesn't confirm payment
        // call::confirm_payment(&factory, &bank_b_id, &bank_a_id, "trade_id").await?;

        thread::sleep(Duration::from_secs(15));

        assert_trade_payment_status(&worker, &account_a_id, "trade_id", &PaymentStatus::Pending)
            .await?;
        assert_trade_payment_status(&worker, &account_b_id, "trade_id", &PaymentStatus::Pending)
            .await?;

        Ok(())
    }

    #[test_case(TradeDetails {
        product: Product::Spot,
        agreement: "Bilateral".to_string(),
        ..Default::default()
    }; "spot")]
    #[test_case(TradeDetails {
        product: Product::Ndf,
        agreement: "Bilateral".to_string(),
        ..Default::default()
    }; "ndf")]
    #[test_case(TradeDetails {
        product: Product::Fwd,
        agreement: "Market".to_string(),
        ..Default::default()
    }; "fwd")]
    #[test_case(TradeDetails {
        product: Product::Swap,
        agreement: "Market".to_string(),
        ..TradeDetails::default_swap()
    }; "swap")]
    #[tokio::test]
    async fn test_trade_timeout(mut trade_details: TradeDetails) -> anyhow::Result<()> {
        dotenv::dotenv();

        // let worker = near_workspaces::testnet_with_rpc("https://near-testnet.api.pagoda.co/rpc/v1/", API_KEY).await?;
        let worker = near_workspaces::testnet().await?;
        let config = Config::new();

        let factory = deploy_contract(&worker, &config).await?;

        let bank_a = "Deutsche Bank".to_string();
        let bank_b = "Sparkasse".to_string();

        call::store_contract(&factory, factory.as_account(), RTP_WASM.to_vec()).await?;

        let storage_cost = view::get_bank_storage_cost(&factory).await?;
        call::create_bank(&factory, &bank_a, NearToken::from_yoctonear(storage_cost)).await?;
        call::create_bank(&factory, &bank_b, NearToken::from_yoctonear(storage_cost)).await?;
        let bank_a_id = view::get_bank_id(&factory, &bank_a).await?;
        let bank_b_id = view::get_bank_id(&factory, &bank_b).await?;
        let account_a_id: AccountId = format!("{bank_a_id}.{}", factory.id()).parse()?;
        let account_b_id: AccountId = format!("{bank_b_id}.{}", factory.id()).parse()?;

        thread::sleep(Duration::from_secs(5));

        trade_details.side = Side::Buy;
        trade_details.counterparty = bank_b;
        trade_details.event_timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        call::perform_trade(&factory, &bank_a_id, &trade_details).await?;

        // 1 minute timeout configured in API
        thread::sleep(Duration::from_secs(65));
        trade_details.side = Side::Sell;
        trade_details.counterparty = bank_a;
        trade_details.event_timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        call::perform_trade(&factory, &bank_b_id, &trade_details).await?;

        thread::sleep(Duration::from_secs(15));

        assert_trade_matching_status(
            &worker,
            &account_a_id,
            "trade_id",
            &MatchingStatus::Rejected("".to_string()),
        )
        .await?;
        assert_trade_matching_status(
            &worker,
            &account_b_id,
            "trade_id",
            &MatchingStatus::Rejected("".to_string()),
        )
        .await?;

        Ok(())
    }

    async fn deploy_contract<T>(worker: &Worker<T>, config: &Config) -> anyhow::Result<Contract>
    where
        T: DevNetwork + TopLevelAccountCreator + 'static,
    {
        let factory_account_id: AccountId = format!(
            "{}.{}",
            config.factory_sub_account.as_str(),
            config.master_account_id.as_str()
        )
        .parse()?;
        let factory_path: PathBuf = ["..", "..", ".near", factory_account_id.as_str()]
            .iter()
            .collect();
        let master_account = Account::from_secret_key(
            config.master_account_id.clone(),
            config.master_secret_key.clone(),
            worker,
        );
        let factory_account = if let Some(factory_secret_key) = config.factory_secret_key.as_ref() {
            Some(Account::from_secret_key(
                factory_account_id.clone(),
                factory_secret_key.clone(),
                worker,
            ))
        } else {
            Account::from_file(&factory_path, worker).ok()
        };
        let key = if let Some(factory_account) = factory_account {
            let key = factory_account.secret_key().clone();

            print_log!(
                "Cleaning old contract storage for {}",
                factory_account_id.as_str().yellow()
            );

            let factory = factory_account
                .deploy(RTP_FACTORY_WASM)
                .await?
                .into_result()?;
            let bank_ids = view::get_bank_ids(&factory, None, None).await?;
            for bank_id in bank_ids {
                call::remove_bank(&factory, &bank_id).await?;
            }
            call::clear_storage(&factory, factory.as_account()).await?;

            print_log!("Deleting account {}", factory_account_id.as_str().yellow());
            if let Err(err) = factory_account
                .delete_account(&config.master_account_id)
                .await?
                .into_result()
            {
                dbg!(err);
            }
            key
        } else {
            let key = SecretKey::from_random(KeyType::ED25519);
            let credentials = Credentials {
                account_id: factory_account_id.to_string(),
                public_key: serde_json::to_string(&key.public_key())?
                    .strip_prefix('\"')
                    .unwrap()
                    .strip_suffix('\"')
                    .unwrap()
                    .to_string(),
                private_key: serde_json::to_string(&key)?
                    .strip_prefix('\"')
                    .unwrap()
                    .strip_suffix('\"')
                    .unwrap()
                    .to_string(),
            };
            let mut file = File::create(&factory_path).await?;
            file.write_all(serde_json::to_string(&credentials)?.as_bytes())
                .await?;
            print_log!(
                "Created new account {}",
                factory_account_id.as_str().yellow()
            );
            key
        };

        let factory = master_account
            .create_subaccount(&config.factory_sub_account)
            .keys(key)
            .transact()
            .await?
            .into_result()?;
        print_log!("Created subaccount {}", factory.id().as_str().yellow());

        master_account
            .transfer_near(factory.id(), NearToken::from_near(30))
            .await?
            .into_result()?;
        print_log!("Funded subaccount {}", factory.id().as_str().yellow());

        let factory = factory.deploy(RTP_FACTORY_WASM).await?.into_result()?;
        print_log!(
            "Deployed factory contract {}",
            factory.id().as_str().yellow()
        );

        call::new(&factory, factory.as_account()).await?;

        Ok(factory)
    }
}
