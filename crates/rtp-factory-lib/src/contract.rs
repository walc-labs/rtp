use crate::{
    rtp, ContractError, StorageKey, BANK_DEPOSIT_COVER_ADDITIONAL_BYTES, BANK_DEPOSIT_TO_COVER_GAS,
    CREATE_CALL_GAS, ON_CREATE_CALL_GAS,
};
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env, near_bindgen,
    store::{Lazy, UnorderedSet},
    AccountId, Balance, PanicOnDefault, Promise, PromiseError,
};
use rtp_contract_common::{DealStatus, PaymentConfirmation, RtpEvent, TradeDetails};
use serde_json::json;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Contract {
    pub bank_ids: UnorderedSet<String>,
    pub contract_code: Lazy<Vec<u8>>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            bank_ids: UnorderedSet::new(StorageKey::BankIds),
            contract_code: Lazy::new(StorageKey::ContractCode, Vec::new()),
        }
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
        self.bank_ids.clear();
        self.contract_code.set(vec![]);
    }

    #[private]
    pub fn remove_bank(&mut self, bank_id: String) -> Promise {
        let factory_account_id = env::current_account_id();
        let bank_account_id = format!("{bank_id}.{factory_account_id}").parse().unwrap();

        rtp::ext(bank_account_id)
            .delete_account()
            .then(Self::ext(env::current_account_id()).on_remove_bank(bank_id))
    }

    #[private]
    pub fn on_remove_bank(
        &mut self,
        bank_id: String,
        #[callback_result] callback_res: Result<(), PromiseError>,
    ) {
        callback_res.unwrap();
        self.bank_ids.remove(&bank_id);
    }

    #[private]
    #[handle_result]
    #[payable]
    pub fn create_bank(&mut self, bank: String) -> Result<(), ContractError> {
        let attached_deposit = env::attached_deposit();
        let factory_account_id = env::current_account_id();

        let code = self.contract_code.get();
        let code_len = code.len();
        let storage_cost = ((code_len + BANK_DEPOSIT_COVER_ADDITIONAL_BYTES) as Balance)
            * env::storage_byte_cost()
            + BANK_DEPOSIT_TO_COVER_GAS;
        if attached_deposit < storage_cost {
            return Err(ContractError::NotEnoughDeposit(
                storage_cost,
                attached_deposit,
            ));
        }

        let mut hasher = DefaultHasher::new();
        bank.hash(&mut hasher);

        let bank_id = format!("{:x}", hasher.finish());

        if self.bank_ids.contains(&bank_id) {
            return Err(ContractError::BankAlreadyExists);
        }

        let account_id = format!("{bank_id}.{factory_account_id}").parse().unwrap();

        let promise_id = env::promise_batch_create(&account_id);
        env::promise_batch_action_create_account(promise_id);
        env::promise_batch_action_transfer(promise_id, attached_deposit / 2);
        env::promise_batch_action_deploy_contract(promise_id, code);
        env::promise_batch_action_function_call(
            promise_id,
            "new",
            json!({
                "factory": factory_account_id,
                "bank": bank,
            })
            .to_string()
            .as_bytes(),
            0,
            CREATE_CALL_GAS,
        );

        let promise_id = env::promise_then(
            promise_id,
            factory_account_id,
            "on_create_bank",
            json!({
                "bank": bank,
                "bank_id": bank_id
            })
            .to_string()
            .as_bytes(),
            0,
            ON_CREATE_CALL_GAS,
        );
        env::promise_return(promise_id);

        Ok(())
    }

    #[private]
    pub fn on_create_bank(
        &mut self,
        bank: String,
        bank_id: String,
        #[callback_result] callback_res: Result<(), PromiseError>,
    ) {
        callback_res.unwrap();
        self.bank_ids.insert(bank_id.clone());

        let event = RtpEvent::NewBank { bank, bank_id };
        event.emit();
    }

    #[private]
    #[handle_result]
    pub fn perform_trade(
        &mut self,
        bank_id: String,
        trade_details: TradeDetails,
    ) -> Result<Promise, ContractError> {
        if !self.bank_ids.contains(&bank_id) {
            return Err(ContractError::BankNotYetExists);
        }

        let factory_account_id = env::current_account_id();
        let account_id = format!("{bank_id}.{factory_account_id}").parse().unwrap();

        Ok(rtp::ext(account_id)
            .with_unused_gas_weight(1)
            .perform_trade(trade_details))
    }

    #[private]
    #[handle_result]
    pub fn settle_trade(
        &mut self,
        partnership_id: String,
        bank_a_id: String,
        bank_b_id: String,
        trade_id: String,
        deal_status: DealStatus,
    ) -> Result<Promise, ContractError> {
        if !self.bank_ids.contains(&bank_a_id) || !self.bank_ids.contains(&bank_b_id) {
            return Err(ContractError::BankNotYetExists);
        }

        let factory_account_id = env::current_account_id();
        let account_a_id: AccountId = format!("{bank_a_id}.{factory_account_id}").parse().unwrap();
        let account_b_id: AccountId = format!("{bank_b_id}.{factory_account_id}").parse().unwrap();

        Ok(rtp::ext(account_a_id.clone())
            .with_unused_gas_weight(1)
            .settle_trade(trade_id.clone(), deal_status.clone())
            .and(
                rtp::ext(account_b_id.clone())
                    .with_unused_gas_weight(1)
                    .settle_trade(trade_id.clone(), deal_status.clone()),
            )
            .then(Self::ext(factory_account_id).on_settle_trade(
                partnership_id,
                account_a_id,
                account_b_id,
                trade_id,
                deal_status,
            )))
    }

    #[allow(clippy::too_many_arguments)]
    #[private]
    pub fn on_settle_trade(
        &mut self,
        partnership_id: String,
        account_a_id: AccountId,
        account_b_id: AccountId,
        trade_id: String,
        deal_status: DealStatus,
        #[callback_result] settlement_a: Result<(), PromiseError>,
        #[callback_result] settlement_b: Result<(), PromiseError>,
    ) {
        let event = RtpEvent::SettleTrade {
            partnership_id,
            trade_id: trade_id.clone(),
            deal_status: if settlement_a.is_err() || settlement_b.is_err() {
                rtp::ext(account_a_id).settle_trade(trade_id.clone(), DealStatus::Error);
                rtp::ext(account_b_id).settle_trade(trade_id, DealStatus::Error);
                DealStatus::Error
            } else {
                deal_status
            },
        };

        event.emit();
    }

    #[private]
    #[handle_result]
    pub fn confirm_payment(
        &mut self,
        creditor_id: String,
        debitor_id: String,
        trade_id: String,
    ) -> Result<Promise, ContractError> {
        if !self.bank_ids.contains(&creditor_id) || !self.bank_ids.contains(&debitor_id) {
            return Err(ContractError::BankNotYetExists);
        }

        let factory_account_id = env::current_account_id();
        let creditor_account_id: AccountId = format!("{creditor_id}.{factory_account_id}")
            .parse()
            .unwrap();
        let debitor_account_id: AccountId = format!("{debitor_id}.{factory_account_id}")
            .parse()
            .unwrap();

        Ok(rtp::ext(creditor_account_id)
            .with_unused_gas_weight(1)
            .confirm_payment(trade_id.clone(), PaymentConfirmation::Credit)
            .and(
                rtp::ext(debitor_account_id)
                    .with_unused_gas_weight(1)
                    .confirm_payment(trade_id.clone(), PaymentConfirmation::Debit),
            )
            .then(Self::ext(factory_account_id).on_confirm_payment()))
    }

    #[private]
    pub fn on_confirm_payment(&mut self, #[callback_unwrap] _a: (), #[callback_unwrap] _b: ()) {}
}
