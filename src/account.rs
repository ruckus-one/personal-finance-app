use std::{fmt::{Display, Error, Formatter}, rc::Rc};
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use crate::currency::Currency;

#[derive(Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub name: String,
    pub balance: f64,
    pub currency: Currency,
}

impl Account {
    pub fn new(name: String, opening_balance: f64, currency: Currency) -> Account {
        let id = Uuid::new_v4().to_string();

        Account {
            id,
            name,
            balance: opening_balance,
            currency,
        }
    }

    pub fn set_balance(&mut self, balance: f64) -> &Account {
        self.balance = balance;
        self
    }
}

impl Display for Account {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "Account: {} - {} - {} - {}", self.id, self.name, self.balance, self.currency)
    }
}

#[allow(dead_code)]
pub fn example_accounts() -> Rc<Vec<Account>> {
    Rc::new(vec![
        Account {
            id: String::from("some-acc-1"),
            name: String::from("Cash"),
            balance: 123.45,
            currency: Currency::PLN,
        },
        Account {
            id: String::from("some-acc-2"),
            name: String::from("Bank account #1"),
            balance: 123.45,
            currency: Currency::GBP,
        },
        Account {
            id: String::from("some-acc-3"),
            name: String::from("Shares value"),
            balance: 123.45,
            currency: Currency::PLN,
        },
    ])
}

mod tests {
    use super::*;

    #[test]
    fn account_balance_can_be_updated() {
        let mut account = Account::new("Cash".into(), 123.45, Currency::PLN);
        assert_eq!(account.balance, 123.45);
        account.set_balance(234.56);
        assert_eq!(account.balance, 234.56);
    }
}