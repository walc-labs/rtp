use crate::{Contract, ContractError, ContractExt};
use near_sdk::{near_bindgen, AccountId};
use std::{
    cmp::Ordering,
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

#[near_bindgen]
impl Contract {
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
