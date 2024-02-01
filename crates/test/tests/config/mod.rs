use near_workspaces::{types::SecretKey, AccountId};
use std::{env, str::FromStr};

pub struct Config {
    pub master_account_id: AccountId,
    pub master_secret_key: SecretKey,
    pub factory_sub_account: AccountId,
    pub factory_secret_key: Option<SecretKey>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            master_account_id: env::var("MASTER_ACCOUNT_ID").unwrap().parse().unwrap(),
            master_secret_key: env::var("MASTER_SECRET_KEY")
                .ok()
                .map(|sk| SecretKey::from_str(&sk).unwrap())
                .unwrap(),
            factory_sub_account: env::var("FACTORY_SUB_ACCOUNT").unwrap().parse().unwrap(),
            factory_secret_key: env::var("FACTORY_SECRET_KEY")
                .ok()
                .map(|sk| SecretKey::from_str(&sk))
                .unwrap()
                .ok(),
        }
    }
}
