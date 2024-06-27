use crate::account::Account;

pub struct Wallet {
    pub accounts: Vec<Account>,
}

impl Wallet {
    pub fn new() -> Wallet {
        Wallet {
            accounts: vec!(
                Account::new("Cash".into(), 0_f64, "PLN".into())
            ),
        }
    }

    pub fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }
}

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
        assert_eq!(wallet.accounts[0].currency, "PLN");
        assert!(is_valid_uuid(wallet.accounts[0].id.as_str()));
    }

    #[test]
    fn test_wallet_add_account() {
        let mut wallet = Wallet::new();
        wallet.add_account(Account::new("Bank account #1".into(), 123.45, "PLN".into()));
        assert_eq!(wallet.accounts.len(), 2);
        assert_eq!(wallet.accounts[1].name, "Bank account #1");
        assert_eq!(wallet.accounts[1].balance, 123.45);
        assert_eq!(wallet.accounts[1].currency, "PLN");
        assert!(is_valid_uuid(wallet.accounts[1].id.as_str()));
    }
}
