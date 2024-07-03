use serde::{Deserialize, Serialize};

use crate::{account::Account, currency::Currency};

#[derive(Clone, Serialize, Deserialize)]
pub struct Wallet {
    pub accounts: Vec<Account>,
}

impl Wallet {
    pub fn new() -> Wallet {
        Wallet {
            accounts: vec!(
                // Account::new("Cash".into(), 0_f64, Currency::PLN)
            ),
        }
    }

    pub fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }

    pub fn accounts(&self) -> &Vec<Account> {
        &self.accounts
    }

    pub fn get_sum_for_currency(&self, currency: Currency) -> f64 {
        let mut sum = 0_f64;
        for account in &self.accounts {
            if account.currency == currency {
                sum += account.balance;
            }
        }
        sum
    }

    pub fn get_used_currencies(&self) -> std::collections::HashSet<Currency> {
        let mut set = std::collections::HashSet::new();
        for account in &self.accounts {
            set.insert(account.currency);
        }
        set
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    fn is_valid_uuid(s: &str) -> bool {
        let uuid_regex = Regex::new(r"^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$").unwrap();
        uuid_regex.is_match(s)
    }    

    #[test]
    fn test_wallet_new() {
        let wallet = Wallet::new();
        assert_eq!(wallet.accounts.len(), 1);
        assert_eq!(wallet.accounts[0].name, "Cash");
        assert_eq!(wallet.accounts[0].balance, 0_f64);
        assert_eq!(wallet.accounts[0].currency, Currency::PLN);
        assert!(is_valid_uuid(wallet.accounts[0].id.as_str()));
    }

    #[test]
    fn test_wallet_add_account() {
        let mut wallet = Wallet::new();
        wallet.add_account(Account::new("Bank account #1".into(), 123.45, Currency::PLN.into()));
        assert_eq!(wallet.accounts.len(), 2);
        assert_eq!(wallet.accounts[1].name, "Bank account #1");
        assert_eq!(wallet.accounts[1].balance, 123.45);
        assert_eq!(wallet.accounts[1].currency, Currency::PLN);
        assert!(is_valid_uuid(wallet.accounts[1].id.as_str()));
    }

    #[test]
    fn test_wallet_get_sum_for_currency() {
        let mut wallet = Wallet::new();
        wallet.add_account(Account::new("Account #1".into(), 123.45, Currency::PLN.into()));
        wallet.add_account(Account::new("Account #2".into(), 67.89, Currency::PLN.into()));
        wallet.add_account(Account::new("Account #2".into(), 782.9, Currency::PLN.into()));
        wallet.add_account(Account::new("Account #2".into(), 12.97, Currency::PLN.into()));
        wallet.add_account(Account::new("Account #2".into(), 123.97, Currency::GBP.into()));
        wallet.add_account(Account::new("Account #2".into(), 60.00, Currency::GBP.into()));

        assert_eq!(wallet.get_sum_for_currency(Currency::PLN), 987.21);
        assert_eq!(wallet.get_sum_for_currency(Currency::GBP), 183.97);
    }

    #[test]
    fn test_wallet_get_used_currencies() {
        let mut wallet = Wallet::new();
        wallet.add_account(Account::new("Account #1".into(), 123.45, Currency::USD.into()));
        wallet.add_account(Account::new("Account #2".into(), 67.89, Currency::PLN.into()));
        wallet.add_account(Account::new("Account #3".into(), 782.9, Currency::PLN.into()));
        wallet.add_account(Account::new("Account #4".into(), 12.97, Currency::PLN.into()));
        wallet.add_account(Account::new("Account #5".into(), 123.97, Currency::GBP.into()));
        wallet.add_account(Account::new("Account #6".into(), 60.00, Currency::GBP.into()));
        wallet.add_account(Account::new("Account #7".into(), 60.00, Currency::GBP.into()));
        wallet.add_account(Account::new("Account #8".into(), 60.00, Currency::USD.into()));
        wallet.add_account(Account::new("Account #9".into(), 60.00, Currency::GBP.into()));
        wallet.add_account(Account::new("Account #9".into(), 60.00, Currency::NOK.into()));

        let used_currencies = wallet.get_used_currencies();
        assert_eq!(used_currencies.len(), 4);

        assert!(used_currencies.contains(&Currency::USD));
        assert!(used_currencies.contains(&Currency::PLN));
        assert!(used_currencies.contains(&Currency::GBP));
        assert!(used_currencies.contains(&Currency::NOK));
    }
}
