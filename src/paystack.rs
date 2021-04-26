pub mod customers;
pub mod transactions;
pub mod transactions_split;

use transactions::Transaction;
use transactions_split::TransactionSplit;
#[derive(Default)]
pub struct Paystack {
    pub transaction: Transaction,
    pub transaction_split: TransactionSplit,
}

impl Paystack {
    pub fn new(key: String) -> Paystack {
        let formatted_bearer = format!("Bearer {}", key);
        Paystack {
            transaction: Transaction {
                bearer_auth: formatted_bearer.to_string(),
                ..Default::default()
            },
            transaction_split: TransactionSplit {
                bearer_auth: formatted_bearer.to_string(),
            },
        }
    }
}
