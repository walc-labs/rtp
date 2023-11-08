use crate::ContractError;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env, near_bindgen, AccountId, PanicOnDefault, Promise,
};
use rtp_contract_common::{Outcome, RtpEventBindgen, Trade};

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Contract {
    factory: AccountId,
    bank_a: String,
    bank_b: String,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(factory: AccountId, bank_a: String, bank_b: String) -> Self {
        Self {
            factory,
            bank_a,
            bank_b,
        }
    }

    #[handle_result]
    pub fn perform_trade(&mut self, bank: String, trade: Trade) -> Result<(), ContractError> {
        if env::predecessor_account_id() != self.factory {
            return Err(ContractError::NotFactory);
        }

        if bank != self.bank_a && bank != self.bank_b {
            return Err(ContractError::InvalidBank);
        }

        let event: RtpEventBindgen = RtpEventBindgen::SendTrade { bank, trade };
        event.emit();

        Ok(())
    }

    #[handle_result]
    pub fn settle_trade(
        &mut self,
        trade_id: String,
        outcome: Outcome,
    ) -> Result<(), ContractError> {
        if env::predecessor_account_id() != self.factory {
            return Err(ContractError::NotFactory);
        }

        // TODO no on-chain logic for storing/verifying?

        let event = RtpEventBindgen::SettleTrade { trade_id, outcome };
        event.emit();

        Ok(())
    }

    #[handle_result]
    pub fn remove_partnership(&mut self) -> Result<(), ContractError> {
        if env::predecessor_account_id() != self.factory {
            return Err(ContractError::NotFactory);
        }

        Promise::new(env::current_account_id())
            .delete_account(self.factory.clone())
            .as_return();

        Ok(())
    }
}
