use crate::ContractError;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env, near_bindgen, AccountId, PanicOnDefault,
};
use rtp_common::{RtpEvent, Trade};

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

        let event = RtpEvent::SendTrade { bank, trade };
        event.emit();

        Ok(())
    }
}
