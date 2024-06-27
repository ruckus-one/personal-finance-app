use std::fmt::{Display, Error, Formatter};
use dialoguer::{theme::ColorfulTheme, Select};
use crossterm::event::read as await_key_press;

mod account;
mod wallet;
use account::{example_accounts, Account};
use wallet::Wallet;

enum Action {
    ShowBalances,
    CreateAccount,
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Action::ShowBalances => write!(f, "Show balances"),
            Action::CreateAccount => write!(f, "Create account"),
        }
    }
}

fn main() {
    // let accounts = example_accounts();
    let mut wallet = Wallet::new();

    let items = vec![
        Action::ShowBalances,
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
        
        match choice {
            Action::ShowBalances => {
                wallet.accounts.iter().for_each(|acc| println!("{}", acc));
            },
            Action::CreateAccount => {
                let name = dialoguer::Input::<String>::new()
                   .with_prompt("Name the new account")
                   .interact()
                   .unwrap();

                wallet.add_account(Account::new(name, 0.into(), "PLN".into()))
            }
        }

        await_key_press().unwrap();
    }
}
