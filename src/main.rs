use std::fmt::{Display, Error, Formatter};
use dialoguer::{theme::ColorfulTheme, Select};
use crossterm::event::read as await_key_press;

mod account;
use account::example_accounts;

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
    let accounts = example_accounts();
    
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
                accounts.iter().for_each(|acc| println!("{}", acc));
            },
            Action::CreateAccount => {
                let id = dialoguer::Input::<String>::new()
                   .with_prompt("Account ID")
                   .interact()
                   .unwrap();

                println!("New Account ID: {}", id);
            }
        }

        await_key_press().unwrap();
    }
}
