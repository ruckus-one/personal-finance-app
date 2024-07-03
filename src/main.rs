use std::fmt::{Display, Error, Formatter};
use currency::Currency;
use dialoguer::{theme::ColorfulTheme, Select};
use crossterm::event::read as await_key_press;
use std::path::PathBuf;

mod history;
mod snapshot;
mod account;
mod wallet;
mod currency;
use account::Account;
use snapshot::Snapshot;
use structopt::StructOpt;
use uuid::Uuid;
use wallet::Wallet;

enum Action {
    ShowBalances,
    CreateAccount,
    UpdateBalances,
    ShowAccounts,
    SaveHistory,
    // LoadHistory,
    Quit,
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Action::ShowBalances => write!(f, "Show balances"),
            Action::CreateAccount => write!(f, "Create account"),
            Action::UpdateBalances => write!(f, "Update balances"),
            Action::ShowAccounts => write!(f, "List accounts"),
            Action::SaveHistory => write!(f, "Save history"),
            Action::Quit => write!(f, "Quit"),
        }
    }
}

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(short, long, parse(from_os_str), default_value = "")]
    filename: PathBuf,
}

fn main() {
    let mut opt = Opt::from_args();
    let mut history = history::History::new();

    if opt.filename.to_str().unwrap() == "" {
        opt.filename = PathBuf::from(format!("balances_{}.json", Uuid::new_v4()));
    } else {
        println!("Reading file: {}", opt.filename.to_str().unwrap());
        
        // read file content
        let file_content = std::fs::read_to_string(&opt.filename).unwrap();

        // deserialize
        history = serde_json::from_str::<history::History>(&file_content).unwrap();
    }

    let items = [
        Action::ShowBalances,
        Action::CreateAccount,
        Action::UpdateBalances,
        Action::ShowAccounts,
        Action::SaveHistory,
        Action::Quit,
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
                let snapshot = history.get_latest_snapshot();

                match snapshot {
                    Some(snapshot) => {
                        println!("Latest snapshot is from {}", snapshot.formatted_timestamp());
                        println!("---");

                        let currencies = snapshot.wallet.get_used_currencies();
                        
                        for currency in currencies {
                            let total = snapshot.wallet.get_sum_for_currency(currency);
                            println!("{}: {}", currency, total);
                        }                            
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
                            println!("{} -> {} {}", account.name, account.balance, account.currency);
                        }
                    },
                    None => {
                        println!("No snapshot found");
                    },
                }
            },
            Action::UpdateBalances => {
                println!("Update balances");

                match history.get_latest_snapshot() {
                    Some(snapshot) => {
                        let mut blank_wallet = snapshot.wallet().clone();

                        // populate wallet balances
                        blank_wallet.accounts.iter_mut().for_each(|account| {
                            let current_balance = dialoguer::Input::<f64>::new()
                                .with_prompt(format!("Enter the balance for: {}. Previous value was: {} [{}]", account.name, account.balance, account.currency))
                                .interact()
                                .unwrap();

                            account.set_balance(current_balance);
                            println!("All done!");
                        });

                        let new_snapshot = Snapshot::new(blank_wallet);
                        history.add_snapshot(new_snapshot);
                    },
                    None => {
                        println!("No snapshot found, need to initialize the history first.");
                        return;
                    }
                };
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

            },
            Action::SaveHistory => {
                println!("Save history");

                let result = serde_json::to_string(&history).unwrap();
                std::fs::write(&opt.filename.to_str().unwrap(), result).unwrap();

                println!("Done! Saved to {}", opt.filename.to_str().unwrap());
            },
            Action::Quit => {
                break;
            },
        }

        await_key_press().unwrap();
    }
}
