use std::{fmt::{Display, Error, Formatter}, rc::Rc};

pub struct Account {
    id: String,
    name: String,
    balance: f64,
    currency: String,
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