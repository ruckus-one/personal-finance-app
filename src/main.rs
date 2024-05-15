use std::io::{stdin, Read};

struct Account {
    id: String,
    name: String,
    balance: f64,
    currency: String,
}

fn main() {
    let mut accounts = vec![
        Account {
            id: String::from("some-acc-1"),
            name: String::from("Cash"),
            balance: 123.45,
            currency: String::from("PLN"),
        },
        Account {
            id: String::from("some-acc-1"),
            name: String::from("Bank account #1"),
            balance: 123.45,
            currency: String::from("PLN"),
        },
        Account {
            id: String::from("some-acc-1"),
            name: String::from("Shares value"),
            balance: 123.45,
            currency: String::from("PLN"),
        },
    ];

    accounts.iter_mut().for_each(|account| {
        println!(
            "What's the current balance of account <{}>?\n",
            account.name
        );

        let mut buffer = String::new();
        match stdin().read_line(&mut buffer) {
            Ok(_) => {
                let input: f64 = buffer.trim().parse().unwrap();
                account.balance = input;
            }
            Err(_) => println!("Tis not a number!"),
        }
    });

    let mut total = 0_f64;
    for account in accounts {
        total += account.balance;
    }

    println!("Total is: {}", total);
}
