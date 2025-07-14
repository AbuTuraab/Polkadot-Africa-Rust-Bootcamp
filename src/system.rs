use std::collections::BTreeMap;
use num::traits::{One, Zero};
use core::ops::AddAssign;

pub trait Config {
    type AccountId: Ord + Clone;
    type BlockNumber: Zero + One + AddAssign + Copy;
    type Nonce: Zero + One + Copy;
}


#[derive(Debug)]
pub struct Pallet <T:Config> {
    pub block_number: T::BlockNumber,            // 2^32 = 4.5 million
    pub nonce: BTreeMap<T::AccountId, T::Nonce>, // <username, nonce_value> e.g. ("femi", 10)
}

impl<T:Config> Pallet<T>{
    /// Create an instance of the pallet
    pub fn new() -> Self {
        Self {
            block_number: T::BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }

    /// Get the current block number
    pub fn block_number(&self) -> T::BlockNumber {
        self.block_number
    }

    /// Increase the block number by one
    pub fn inc_block_number(&mut self) {
        self.block_number += T::BlockNumber::one();
    }

    /// Increase the nonce value of the caller `who`
    pub fn inc_nonce(&mut self, who: &T::AccountId) {
        // Check for the nonce of `who`, and store. If it does not exist, set nonce to `0`
        // create new nonce => nonce + 1
        // store new nonce, with caller
        let nonce = *self.nonce.get(who).unwrap_or(&T::Nonce::zero());
        let new_nonce = nonce + T::Nonce::one();
        self.nonce.insert(who.clone(), new_nonce);
    }
}

#[cfg(test)]
mod tests {
    
    struct TestConfig;

    impl super::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }
    use super::*;

    #[test]
    fn system_pallet_work() {
        // Arrange
        // create system pallet
        let mut system = Pallet::<TestConfig>::new();

        // Act
        // increase current block number
        system.inc_block_number();
        // increase the nonce of a user - `Temi`
        system.inc_nonce(&"Temi".to_string());

        // Assert
        // Check the block number (i.e. 1)
        assert_eq!(system.block_number(), 1);
        // Check the nonce of Temi (i.e. 1)
        assert_eq!(system.nonce.get("Temi"), Some(&1));
        // Check the nonce of Faithful (i.e. 0)
        assert_eq!(system.nonce.get("Faithful"), None);
    }
}
