use std::{fmt::{Display, Error, Formatter}, rc::Rc};
use uuid::Uuid;

pub struct Account {
    pub id: String,
    pub name: String,
    pub balance: f64,
    pub currency: String,
}

impl Account {
    pub fn new(name: String, opening_balance: f64, currency: String) -> Account {
        let id = Uuid::new_v4().to_string();

        Account {
            id,
            name,
            balance: opening_balance,
            currency,
        }
    }
}

impl Display for Account {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "Account: {} - {} - {} - {}", self.id, self.name, self.balance, self.currency)
    }
}

pub fn example_accounts() -> Rc<Vec<Account>> {
    Rc::new(vec![
        Account {
            id: String::from("some-acc-1"),
            name: String::from("Cash"),
            balance: 123.45,
            currency: String::from("PLN"),
        },
        Account {
            id: String::from("some-acc-2"),
            name: String::from("Bank account #1"),
            balance: 123.45,
            currency: String::from("PLN"),
        },
        Account {
            id: String::from("some-acc-3"),
            name: String::from("Shares value"),
            balance: 123.45,
            currency: String::from("PLN"),
        },
    ])
}