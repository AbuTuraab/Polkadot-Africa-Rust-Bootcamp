use num::traits::{CheckedAdd, CheckedSub, Zero};
use std::collections::BTreeMap;

pub trait Config: crate::system::Config {
    type Balance: CheckedAdd + CheckedSub + Zero + Copy;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    balances: BTreeMap<T::AccountId, T::Balance>,
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    pub fn set_balance(&mut self, who: &T::AccountId, amount: T::Balance) {
        self.balances.insert(who.clone(), amount);
    }

    pub fn balance(&self, who: &T::AccountId) -> T::Balance {
        *self.balances.get(who).unwrap_or(&T::Balance::zero())
    }

    pub fn transfer(
        &mut self,
        sender: T::AccountId,
        receiver: T::AccountId,
        amount: T::Balance,
    ) -> Result<(), &'static str> {
        let sender_balance = self.balance(&sender);
        let receiver_balance = self.balance(&receiver);

        let new_sender_balance = sender_balance
            .checked_sub(&amount)
            .ok_or("Not enough balance")?;
        let new_receiver_balance = receiver_balance.checked_add(&amount).ok_or("Overflow")?;

        self.balances.insert(sender, new_sender_balance);
        self.balances.insert(receiver, new_receiver_balance);
        Ok(())
    }
}

// An enum for calls available in the balance pallet
pub enum Call<T: Config> {
    Transfer {
        to: T::AccountId,
        amount: T::Balance,
    },
}

impl<T: Config> crate::support::Dispatch for Pallet<T> {
    type Call = Call<T>;
    type Caller = T::AccountId;

    fn dispatch(
        &mut self,
        caller: Self::Caller,
        call: Self::Call,
    ) -> crate::support::DispatchResult {
        match call {
            Call::Transfer { to, amount } => self.transfer(caller, to, amount)?,
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    struct TestConfig;

    impl crate::system::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    impl super::Config for TestConfig {
        type Balance = u32;
    }
    #[test]
    fn init_balances() {
        let mut balances = super::Pallet::<TestConfig>::new();

        assert_eq!(balances.balance(&"alice".to_string()), 0);
        balances.set_balance(&"alice".to_string(), 100);
        assert_eq!(balances.balance(&"alice".to_string()), 100);
        assert_eq!(balances.balance(&"bob".to_string()), 0);
    }

    #[test]
    fn transfer_balance() {
        let mut balances = super::Pallet::<TestConfig>::new();

        assert_eq!(
            balances.transfer("alice".to_string(), "bob".to_string(), 51),
            Err("Not enough balance")
        );

        balances.set_balance(&"alice".to_string(), 100);
        assert_eq!(
            balances.transfer("alice".to_string(), "bob".to_string(), 51),
            Ok(())
        );

        assert_eq!(balances.balance(&"alice".to_string()), 49);
        assert_eq!(balances.balance(&"bob".to_string()), 51);

        assert_eq!(
            balances.transfer("alice".to_string(), "bob".to_string(), 51),
            Err("Not enough balance")
        );
    }
}
