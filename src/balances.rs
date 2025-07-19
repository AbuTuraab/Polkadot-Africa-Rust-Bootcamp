use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Pallet {
   balances: BTreeMap<String, u128>,
}

// enum Result<T, E>{
//     Ok(T),
//     Err(E)
// }


impl Pallet {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new()
        }
    }

    pub fn set_balance(&mut self, who:&String, amount: u128){
        self.balances.insert(who.clone(), amount);
    }

    pub fn balance(&self, who:&String) -> u128 {
        *self.balances.get(who).unwrap_or(&0)
    }

    pub fn transfer(
        &mut self,
        sender: String,
        receiver: String,
        amount: u128,
    ) -> Result<(), &'static str> {
            let sender_balance = self.balance(&sender);
            let receiver_balance = self.balance(&receiver);

            let new_sender_balance= sender_balance.checked_sub(amount).ok_or("Not enough balance")?;
            let new_receiver_balance = receiver_balance.checked_add(amount).ok_or("Overflow")?;

            self.balances.insert(sender, new_sender_balance);
            self.balances.insert(receiver, new_receiver_balance);
        Ok(())
    }
}

#[cfg(test)]
mod tests{
    #[test]
fn init_balances(){
    let mut balances = super::Pallet::new();

    assert_eq!(balances.balance(&"alice".to_string()), 0);
    balances.set_balance(&"alice".to_string(), 100);
    assert_eq!(balances.balance(&"alice".to_string()), 100);
    assert_eq!(balances.balance(&"bob".to_string()), 0);

}

fn transfer_balance(){
    let mut balances = super::Pallet::new();

    assert_eq!(
        balances.transfer("alice".to_string(), "bob".to_string(), 51),
     Err("Not  enough funds")
    );

    balances.set_balance(&"alice".to_string(), 100);
    assert_eq!(balances.transfer("alice".to_string(), "bob".to_string(), 51), 
Ok(())
);


assert_eq!(balances.balance(&"alice".to_string()), 49);
assert_eq!(balances.balance(&"bob".to_string()), 51);


assert_eq!(
    balances.transfer("alice".to_string(), "bob".to_string(), 51),
    Err("No enough balances")
);
   
}
}
