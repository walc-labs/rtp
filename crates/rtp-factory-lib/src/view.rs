use crate::{Contract, ContractError, ContractExt};
use near_sdk::{near_bindgen, AccountId};
use std::{
    cmp::Ordering,
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

#[near_bindgen]
impl Contract {
    pub fn get_partnerships(&self, skip: Option<u32>, limit: Option<u32>) -> Vec<String> {
        self.partnership_contracts
            .iter()
            .skip(skip.unwrap_or_default() as usize)
            .enumerate()
            .take_while(|(index, _)| *index < limit.unwrap_or(20) as usize)
            .map(|partnership| partnership.1.clone())
            .collect()
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
