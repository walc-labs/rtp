use near_workspaces::AccountId;
use std::env;

pub struct Config {
    pub factory_account_id: AccountId,
    pub beneficiary_account_id: AccountId,
}

impl Config {
    pub fn new() -> Self {
        Self {
            factory_account_id: env::var("FACTORY_ACCOUNT_ID").unwrap().parse().unwrap(),
            beneficiary_account_id: env::var("BENEFICIARY_ACCOUNT_ID").unwrap().parse().unwrap(),
        }
    }
}
