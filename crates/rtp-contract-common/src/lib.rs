mod event;
mod trade;

pub use event::*;
pub use trade::*;

use std::{
    cmp::Ordering,
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

pub fn get_bank_id(bank: &str) -> String {
    let mut hasher = DefaultHasher::new();
    bank.hash(&mut hasher);
    format!("{:x}", hasher.finish()).parse().unwrap()
}

pub fn get_partnership_id(
    #[allow(unused_mut)] mut bank_a: String,
    #[allow(unused_mut)] mut bank_b: String,
) -> String {
    let mut hasher = DefaultHasher::new();
    match bank_a.cmp(&bank_b) {
        Ordering::Less => {}
        Ordering::Greater => std::mem::swap(&mut bank_a, &mut bank_b),
        Ordering::Equal => panic!("bank_a and bank_b must not be the same"),
    }
    (&bank_a, &bank_b).hash(&mut hasher);
    format!("{:x}", hasher.finish()).parse().unwrap()
}
