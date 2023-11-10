use crate::{
    rtp, ContractError, StorageKey, CREATE_CALL_GAS, ON_CREATE_CALL_GAS,
    REPRESENTATIVE_DEPOSIT_TO_COVER_GAS,
};
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env, near_bindgen,
    store::{Lazy, UnorderedSet},
    Balance, PanicOnDefault, Promise, PromiseError,
};
use rtp_contract_common::{DealStatus, RtpEventBindgen, TradeDetails};
use serde_json::json;
use std::{
    cmp::Ordering,
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Contract {
    pub partnership_contracts: UnorderedSet<String>,
    pub contract_code: Lazy<Vec<u8>>,
}

#[near_bindgen]
impl Contract {
    #[init]
    #[handle_result]
    pub fn new() -> Result<Self, ContractError> {
        let input = env::input().ok_or(ContractError::NoInput)?;
        Ok(Self {
            partnership_contracts: UnorderedSet::new(StorageKey::PartnershipContracts),
            contract_code: Lazy::new(StorageKey::ContractCode, input),
        })
    }

    /// Store contract from input.
    #[private]
    #[handle_result]
    pub fn store_contract(&mut self) -> Result<(), ContractError> {
        let input = env::input().ok_or(ContractError::NoInput)?;
        self.contract_code.set(input);

        Ok(())
    }

    /// Clear storage for testing.
    #[private]
    #[handle_result]
    pub fn clear_storage(&mut self) {
        self.partnership_contracts.clear();
        self.contract_code.set(vec![]);
    }

    #[private]
    pub fn remove_partnership(&mut self, partnership_id: String) -> Promise {
        let factory_account_id = env::current_account_id();
        let partnership_contract_id = format!("{partnership_id}.{factory_account_id}")
            .parse()
            .unwrap();

        rtp::ext(partnership_contract_id)
            .remove_partnership()
            .then(Self::ext(env::current_account_id()).on_remove_partnership(partnership_id))
    }

    #[private]
    pub fn on_remove_partnership(
        &mut self,
        partnership_id: String,
        #[callback_result] callback_res: Result<(), PromiseError>,
    ) {
        callback_res.unwrap();
        self.partnership_contracts.remove(&partnership_id);
    }

    #[private]
    #[handle_result]
    #[payable]
    // FIXME unused_mut
    pub fn create_partnership(
        &mut self,
        #[allow(unused_mut)] mut bank_a: String,
        #[allow(unused_mut)] mut bank_b: String,
    ) -> Result<(), ContractError> {
        let attached_deposit = env::attached_deposit();
        let factory_account_id = env::current_account_id();

        let code = self.contract_code.get();
        let code_len = code.len();
        let storage_cost = ((code_len + 32) as Balance) * env::storage_byte_cost()
            + REPRESENTATIVE_DEPOSIT_TO_COVER_GAS;
        if attached_deposit < storage_cost {
            return Err(ContractError::NotEnoughDeposit(
                storage_cost,
                attached_deposit,
            ));
        }

        let mut hasher = DefaultHasher::new();
        match bank_a.cmp(&bank_b) {
            Ordering::Less => {}
            Ordering::Greater => std::mem::swap(&mut bank_a, &mut bank_b),
            Ordering::Equal => return Err(ContractError::InvalidBankInput),
        }
        (&bank_a, &bank_b).hash(&mut hasher);

        let partnership_id = format!("{:x}", hasher.finish());

        if self.partnership_contracts.contains(&partnership_id) {
            return Err(ContractError::PartnershipAlreadyExists);
        }

        // Schedule a Promise tx to account_id.
        let partnership_account_id = format!("{partnership_id}.{factory_account_id}")
            .parse()
            .unwrap();
        let promise_id = env::promise_batch_create(&partnership_account_id);

        // Create account first.
        env::promise_batch_action_create_account(promise_id);

        // Transfer attached deposit.
        env::promise_batch_action_transfer(promise_id, attached_deposit);

        // Deploy contract.
        env::promise_batch_action_deploy_contract(promise_id, code);

        // call `new` with given arguments.
        let args = json!({
            "factory": factory_account_id,
            "partnership_id": partnership_id,
            "bank_a": bank_a,
            "bank_b": bank_b
        });
        let args = args.to_string();
        env::promise_batch_action_function_call(
            promise_id,
            "new",
            args.as_bytes(),
            0,
            CREATE_CALL_GAS,
        );

        // attach callback to the factory.
        let args = json!({
            "partnership_id": partnership_id
        });
        let args = args.to_string();
        let _ = env::promise_then(
            promise_id,
            factory_account_id,
            "on_create_partnership",
            args.as_bytes(),
            0,
            ON_CREATE_CALL_GAS,
        );
        env::promise_return(promise_id);

        Ok(())
    }

    #[private]
    pub fn on_create_partnership(
        &mut self,
        partnership_id: String,
        #[callback_result] callback_res: Result<(), PromiseError>,
    ) {
        callback_res.unwrap();
        self.partnership_contracts
            .insert(partnership_id.to_string());

        let event = RtpEventBindgen::NewPartnership { partnership_id };
        event.emit();
    }

    #[private]
    #[handle_result]
    pub fn perform_trade(
        &mut self,
        bank: String,
        partnership_id: String,
        trade_details: TradeDetails,
    ) -> Result<Promise, ContractError> {
        if !self.partnership_contracts.contains(&partnership_id) {
            return Err(ContractError::PartnershipNotYetExists);
        }

        let factory_account_id = env::current_account_id();
        let partnership_id = format!("{partnership_id}.{factory_account_id}")
            .parse()
            .unwrap();

        Ok(rtp::ext(partnership_id)
            .with_unused_gas_weight(1)
            .perform_trade(bank, trade_details))
    }

    #[private]
    #[handle_result]
    pub fn settle_trade(
        &mut self,
        partnership_id: String,
        trade_id: String,
        deal_status: DealStatus,
    ) -> Result<Promise, ContractError> {
        if !self.partnership_contracts.contains(&partnership_id) {
            return Err(ContractError::PartnershipNotYetExists);
        }

        let factory_account_id = env::current_account_id();
        let partnership_id = format!("{partnership_id}.{factory_account_id}")
            .parse()
            .unwrap();

        Ok(rtp::ext(partnership_id)
            .with_unused_gas_weight(1)
            .settle_trade(trade_id, deal_status))
    }
}
