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
        network::Testnet,
        types::{KeyType, NearToken, SecretKey},
        Account, AccountId, Contract, Worker,
    };
    use owo_colors::OwoColorize;
    use rtp_contract_common::{
        DealType, MatchingStatus, PaymentStatus, Product, Settlement, Side, TradeDetails,
    };
    use serde_json::Value;
    use std::{env, path::PathBuf, thread, time::Duration};
    use tokio::{fs::File, io::AsyncWriteExt};

    const RTP_FACTORY_WASM: &[u8] = include_bytes!("../../../res/rtp_factory.wasm");
    const RTP_WASM: &[u8] = include_bytes!("../../../res/rtp.wasm");

    #[tokio::test]
    async fn test_settle_trade_basic() -> anyhow::Result<()> {
        dotenv::dotenv();

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

        let mut trade_details = TradeDetails {
            trade_id: "trade_id".to_string(),
            timestamp: 0,
            deal_type: DealType::FxDeal,
            product: Product::Spot,
            contract: "contract".to_string(),
            counterparty: bank_b.clone(),
            amount: "1".to_string(),
            price: "2".to_string(),
            side: Side::Buy,
            settlement: Settlement::RealTime,
            delivery_date: 0,
            payment_calendars: "payment_calendars".to_string(),
            contract_number: "contract_number".to_string(),
        };

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

    #[tokio::test]
    async fn test_settle_trade_complex() -> anyhow::Result<()> {
        dotenv::dotenv();

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

        let mut trade_details = TradeDetails {
            trade_id: "trade_id".to_string(),
            timestamp: 0,
            deal_type: DealType::FxDeal,
            product: Product::Spot,
            contract: "contract".to_string(),
            counterparty: bank_b.clone(),
            amount: "1".to_string(),
            price: "2".to_string(),
            side: Side::Buy,
            settlement: Settlement::RealTime,
            delivery_date: 0,
            payment_calendars: "payment_calendars".to_string(),
            contract_number: "contract_number".to_string(),
        };

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

    // TODO test case with failing match
    // TODO test failing payment
    // TODO test timeout

    async fn deploy_contract(
        worker: &Worker<Testnet>,
        config: &Config,
    ) -> anyhow::Result<Contract> {
        let factory_path: PathBuf = ["..", "..", ".near", config.factory_account_id.as_str()]
            .iter()
            .collect();
        let key = if let Ok(account) = Account::from_file(&factory_path, worker) {
            let key = account.secret_key().clone();

            print_log!(
                "Cleaning old contract storage for {}",
                config.factory_account_id.as_str().yellow()
            );

            let factory = account.deploy(RTP_FACTORY_WASM).await?.into_result()?;
            let bank_ids = view::get_bank_ids(&factory, None, None).await?;
            for bank_id in bank_ids {
                call::remove_bank(&factory, &bank_id).await?;
            }
            call::clear_storage(&factory, factory.as_account()).await?;

            print_log!(
                "Deleting account {}",
                config.factory_account_id.as_str().yellow()
            );
            if let Err(err) = account
                .delete_account(&config.beneficiary_account_id)
                .await?
                .into_result()
            {
                dbg!(err);
            }
            key
        } else {
            let key = SecretKey::from_random(KeyType::ED25519);
            let credentials = Credentials {
                account_id: config.factory_account_id.to_string(),
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
                config.factory_account_id.as_str().yellow()
            );
            key
        };

        let factory = worker
            .create_tla_and_deploy(config.factory_account_id.clone(), key, RTP_FACTORY_WASM)
            .await?
            .into_result()?;
        print_log!(
            "Deployed factory contract {}",
            config.factory_account_id.as_str().yellow()
        );

        call::new(&factory, factory.as_account()).await?;

        Ok(factory)
    }
}
