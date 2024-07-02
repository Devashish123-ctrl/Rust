use ic_cdk::export::candid::{CandidType, Deserialize};
use std::collections::HashMap;
use std::cell::RefCell;

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Token {
    symbol: String,
    total_supply: u64,
    balances: RefCell<HashMap<String, u64>>,
}

impl Token {
    pub fn new(symbol: String, total_supply: u64) -> Self {
        let mut balances = HashMap::new();
        balances.insert("creator".to_string(), total_supply);
        Self {
            symbol,
            total_supply,
            balances: RefCell::new(balances),
        }
    }

    pub fn balance_of(&self, account: String) -> u64 {
        *self.balances.borrow().get(&account).unwrap_or(&0)
    }

    pub fn transfer(&self, from: String, to: String, amount: u64) -> Result<(), String> {
        let mut balances = self.balances.borrow_mut();
        let from_balance = balances.get(&from).unwrap_or(&0);
        if *from_balance < amount {
            return Err("Insufficient balance".to_string());
        }
        *balances.entry(from).or_insert(0) -= amount;
        *balances.entry(to).or_insert(0) += amount;
        Ok(())
    }
}
