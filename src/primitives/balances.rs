use std::collections::HashMap;
use crate::primitives::{Amount, CurencyName};

pub struct Balances(pub HashMap<CurencyName, Amount>);

impl Balances {
    pub fn new(currencies: HashMap<CurencyName, Amount>) -> Balances {
        Balances(currencies)
    }

    pub fn increase_currency_amount(&mut self, currency_name: &CurencyName, amount: Amount) -> Result<(), ()>{
        if let Some(currency) = self.0.get_mut(currency_name) {
            *currency += amount;
        } else { return Err(()) }
        Ok(())
    }

    pub fn reduce_currency_amount(&mut self, currency_name: &CurencyName, amount: Amount) -> Result<(), ()>{
        if let Some(currency) = self.0.get_mut(currency_name) {
            *currency -= amount;
        } else { return Err(()) }
        Ok(())
    }

    pub fn get_currency_amount(&self, currency_name: CurencyName) -> Amount {
        self.0[&currency_name]
    }
}