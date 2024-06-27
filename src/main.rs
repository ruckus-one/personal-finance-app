use std::fmt::{Display, Error, Formatter};
use currency::Currency;
use dialoguer::{theme::ColorfulTheme, Select};
use crossterm::event::read as await_key_press;

mod history;
mod snapshot;
mod account;
mod wallet;
mod currency;
use account::Account;
use wallet::Wallet;

enum Action {
    ShowBalances,
    CreateAccount,
    ShowAccounts,
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Action::ShowBalances => write!(f, "Show balances"),
            Action::CreateAccount => write!(f, "Create account"),
            Action::ShowAccounts => write!(f, "List accounts"),
        }
    }
}

fn main() {
    let mut history = history::History::new();

    let items = vec![
        Action::ShowBalances,
        Action::ShowAccounts,
        Action::CreateAccount,
    ];

    loop {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose an option")
            .default(0)
            .items(&items)
            .interact()
            .unwrap();
    
        let choice = &items[selection];
        
        history.get_latest_snapshot().unwrap().wallet.accounts[0].balance = 100.into();

        match choice {
            Action::ShowBalances => {
                let snapshot = history.get_latest_snapshot();

                match snapshot {
                    Some(snapshot) => {
                        println!("Latest snapshot is from {}", snapshot.formatted_timestamp());
                        println!("---");

                        let foo = snapshot.wallet.get_sum_for_currency(Currency::PLN);
                        println!("Balance for PLN -> {}", foo);
                    },
                    None => {
                        println!("No snapshot found");
                    }
                }
            },
            Action::ShowAccounts => {
                let snapshot = history.get_latest_snapshot();

                match snapshot {
                    Some(snapshot) => {
                        println!("Latest snapshot is from {}", snapshot.formatted_timestamp());
                        println!("---");
                        let wallet = snapshot.wallet();
                        
                        for account in wallet.accounts() {
                            println!("{} -> {}", account.name, account.balance);
                        }
                    },
                    None => {
                        println!("No snapshot found");
                    },
                }
            },
            Action::CreateAccount => {
                let name = dialoguer::Input::<String>::new()
                   .with_prompt("Name the new account")
                   .interact()
                   .unwrap();

                let snapshot = history.get_latest_snapshot();

                match snapshot {
                    Some(snapshot) => {
                        let wallet = snapshot.wallet();
                        wallet.add_account(Account::new(name.clone(), 0.into(), Currency::PLN));
                        println!("New account has been created! -> {}", name);   
                    },
                    None => {
                        println!("No snapshot found");
                    }
                }
            }
        }

        await_key_press().unwrap();
    }
}
