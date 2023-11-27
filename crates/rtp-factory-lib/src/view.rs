use crate::{
    Contract, ContractExt, BANK_DEPOSIT_COVER_ADDITIONAL_BYTES, BANK_DEPOSIT_TO_COVER_GAS,
};
use near_sdk::{env, near_bindgen, Balance};

#[near_bindgen]
impl Contract {
    pub fn get_bank_ids(&self, skip: Option<u32>, limit: Option<u32>) -> Vec<String> {
        self.bank_ids
            .iter()
            .skip(skip.unwrap_or_default() as usize)
            .enumerate()
            .take_while(|(index, _)| *index < limit.unwrap_or(20) as usize)
            .map(|bank| bank.1.clone())
            .collect()
    }

    pub fn get_bank_id(&self, bank: String) -> String {
        rtp_contract_common::get_bank_id(&bank)
    }

    pub fn get_partnership_id(
        &self,
        #[allow(unused_mut)] mut bank_a: String,
        #[allow(unused_mut)] mut bank_b: String,
    ) -> String {
        rtp_contract_common::get_partnership_id(bank_a, bank_b)
    }

    pub fn get_bank_storage_cost(&self) -> Balance {
        let code = self.contract_code.get();
        let code_len: usize = code.len();
        ((code_len + BANK_DEPOSIT_COVER_ADDITIONAL_BYTES) as Balance) * env::storage_byte_cost()
            + BANK_DEPOSIT_TO_COVER_GAS
    }
}
