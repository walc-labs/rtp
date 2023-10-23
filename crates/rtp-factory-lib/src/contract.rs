use crate::{ContractError, StorageKey, CREATE_CALL_GAS, ON_CREATE_CALL_GAS};
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env, near_bindgen,
    store::{Lazy, UnorderedSet},
    AccountId, Balance, PanicOnDefault, PromiseError,
};
use rtp_common::RtpEvent;
use serde_json::json;
use std::{
    cmp::Ordering,
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Contract {
    partnership_contracts: UnorderedSet<AccountId>,
    contract_code: Lazy<Vec<u8>>,
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
    #[handle_result]
    pub fn store_contract(&mut self) -> Result<(), ContractError> {
        let input = env::input().ok_or(ContractError::NoInput)?;
        self.contract_code.set(input);

        Ok(())
    }

    #[private]
    #[handle_result]
    // FIXME unused_mut
    pub fn create_partnership(
        &self,
        #[allow(unused_mut)] mut bank_a: String,
        #[allow(unused_mut)] mut bank_b: String,
    ) -> Result<(), ContractError> {
        let attached_deposit = env::attached_deposit();
        let factory_account_id = env::current_account_id();

        let code = self.contract_code.get();
        let code_len = code.len();
        let storage_cost = ((code_len + 32) as Balance) * env::storage_byte_cost();
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

        let partnership_id = format!("{:x}", hasher.finish()).parse().unwrap();

        if self.partnership_contracts.contains(&partnership_id) {
            return Err(ContractError::PartnershipAlreadyExists);
        }

        // Schedule a Promise tx to account_id.
        let promise_id = env::promise_batch_create(&partnership_id);

        // Create account first.
        env::promise_batch_action_create_account(promise_id);

        // Transfer attached deposit.
        env::promise_batch_action_transfer(promise_id, attached_deposit);

        // Deploy contract.
        env::promise_batch_action_deploy_contract(promise_id, code);

        // call `new` with given arguments.
        let args = json!({
            "factory": factory_account_id,
            "bank_a": bank_a,
            "bank_b": bank_b
        });
        let args = args.as_str().unwrap().as_bytes();
        env::promise_batch_action_function_call(promise_id, "new", args, 0, CREATE_CALL_GAS);

        // attach callback to the factory.
        let args = json!({
            "partnership_id": partnership_id
        });
        let args = args.as_str().unwrap().as_bytes();
        let _ = env::promise_then(
            promise_id,
            factory_account_id,
            "on_create_partnership",
            args,
            0,
            ON_CREATE_CALL_GAS,
        );
        env::promise_return(promise_id);

        Ok(())
    }

    #[private]
    pub fn on_create_partnership(
        &mut self,
        partnership_id: AccountId,
        #[callback_result] callback_res: Result<(), PromiseError>,
    ) {
        callback_res.unwrap();
        self.partnership_contracts.insert(partnership_id.clone());

        let event = RtpEvent::NewPartnership { partnership_id };
        event.emit();
    }

    pub fn get_partnership_storage_cost(&self) -> Balance {
        let code = self.contract_code.get();
        let code_len = code.len();
        ((code_len + 32) as Balance) * env::storage_byte_cost()
    }

    #[handle_result]
    pub fn get_partnership_id(
        &self,
        #[allow(unused_mut)] mut bank_a: String,
        #[allow(unused_mut)] mut bank_b: String,
    ) -> Result<AccountId, ContractError> {
        let mut hasher = DefaultHasher::new();
        match bank_a.cmp(&bank_b) {
            Ordering::Less => {}
            Ordering::Greater => std::mem::swap(&mut bank_a, &mut bank_b),
            Ordering::Equal => return Err(ContractError::InvalidBankInput),
        }
        (&bank_a, &bank_b).hash(&mut hasher);

        Ok(format!("{:x}", hasher.finish()).parse().unwrap())
    }
}
