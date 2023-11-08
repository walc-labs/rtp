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
        Account, Contract, Worker,
    };
    use owo_colors::OwoColorize;
    use rtp_contract_common::{DealStatus, DealType, Settlement, Side, Speed, Trade};
    use std::{env, path::PathBuf};
    use tokio::{fs::File, io::AsyncWriteExt};

    const RTP_FACTORY_WASM: &[u8] = include_bytes!("../../../res/rtp_factory.wasm");
    const RTP_WASM: &[u8] = include_bytes!("../../../res/rtp.wasm");

    #[tokio::test]
    async fn test_settle_trade() -> anyhow::Result<()> {
        dotenv::dotenv();

        let worker = near_workspaces::testnet().await?;
        let config = Config::new();

        let factory = deploy_contract(&worker, &config).await?;

        let bank_a = "Deutsche Bank".to_string();
        let bank_b = "Sparkasse".to_string();

        call::store_contract(&factory, factory.as_account(), RTP_WASM.to_vec()).await?;

        let storage_cost = view::get_partnership_storage_cost(&factory).await?;
        call::create_partnership(
            &factory,
            &bank_a,
            &bank_b,
            NearToken::from_yoctonear(storage_cost),
        )
        .await?;
        let partnership_id = view::get_partnership_id(&factory, &bank_a, &bank_b).await?;

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

        call::perform_trade(&factory, &bank_a, &partnership_id, &trade).await?;
        call::perform_trade(&factory, &bank_b, &partnership_id, &trade).await?;

        Ok(())
    }

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
            let partnerships = view::get_partnerships(&factory, None, None).await?;
            for partnership_id in partnerships {
                call::remove_partnership(&factory, &partnership_id).await?;
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
