use crate::{snapshot::Snapshot, wallet::Wallet};

pub struct History {
    pub snapshots: Vec<Snapshot>,
}

impl History {
    pub fn new() -> History {
        History {
            snapshots: vec!(
                Snapshot::new(
                    Wallet::new()
                )
            ),
        }
    }

    pub fn add_snapshot(&mut self, snapshot: Snapshot) {
        self.snapshots.push(snapshot)
    }

    // method to get a mutable reference to the latest snapshop
    pub fn get_latest_snapshot(&mut self) -> Option<&mut Snapshot> {
        self.snapshots.last_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_history_new() {
        let history = History::new();
        assert_eq!(history.snapshots.len(), 1);
    }

    #[test]
    fn test_history_add_snapshot() {
        let mut history = History::new();
        history.add_snapshot(
            Snapshot::new(
                Wallet::new()
            )
        );
        assert_eq!(history.snapshots.len(), 2);
    }

    #[test]
    fn test_history_get_latest_snapshot() {
        let mut history = History::new();
        assert_eq!(history.get_latest_snapshot().unwrap().wallet().accounts[0].balance, 0_f64);
    }

    #[test]
    fn test_history_get_latest_snapshot_after_add_snapshot() {
        let mut history = History::new();
        let mut wallet = Wallet::new();
        wallet.accounts[0].balance = 123.45;

        history.add_snapshot(
            Snapshot::new(
                wallet
            )
        
        );
        assert_eq!(history.get_latest_snapshot().unwrap().wallet().accounts[0].balance, 123.45);
    }
}