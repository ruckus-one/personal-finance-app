use crate::Wallet;
use chrono::prelude::*;

pub struct Snapshot {
    pub wallet: Wallet,
    pub timestamp: i64,
}

impl Snapshot {
    pub fn new(wallet: Wallet) -> Snapshot {
        Snapshot {
            wallet,
            timestamp: Utc::now().timestamp(),
        }
    }

    // get mutable ref to the wallet
    pub fn wallet(&mut self) -> &mut Wallet {
        &mut self.wallet
    }

    // get the formatted timestamp using chrono
    pub fn formatted_timestamp(&self) -> String {
        let datetime = Utc.timestamp_opt(self.timestamp, 0).unwrap();
        datetime.to_rfc3339()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snapshot_new() {
        let wallet = Wallet::new();

        let now = Utc::now().timestamp();
        let snapshot = Snapshot::new(wallet);
        assert_eq!(snapshot.timestamp, now);
    }

    #[test]
    fn test_formatted_timestamp() {
        let expected = Utc.timestamp_opt(Utc::now().timestamp(), 0).unwrap().to_rfc3339();

        let wallet = Wallet::new();
        let snapshot = Snapshot::new(wallet);
        let timestamp_note = snapshot.formatted_timestamp();

        assert!(timestamp_note == expected);
    }
}
