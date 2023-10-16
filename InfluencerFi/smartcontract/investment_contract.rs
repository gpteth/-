// Import necessary dependencies for a Rust smart contract
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::Vector;
use near_sdk::env;
use near_sdk::near_bindgen;

// Define the data structure for an investment
#[derive(BorshDeserialize, BorshSerialize)]
struct Investment {
    investor_id: String,
    amount: u64,
    // Add other investment data fields here
}

// Define the data structure for the investment contract
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
struct InvestmentContract {
    investments: Vector<Investment>,
}

// Implement methods for the investment contract
#[near_bindgen]
impl InvestmentContract {
    // Constructor to initialize the investment contract
    pub fn new() -> Self {
        Self {
            investments: Vector::new(b"i".to_vec()),
        }
    }

    // Deposit funds into the investment contract
    pub fn deposit(&mut self, amount: u64) {
        let investor_id = env::predecessor_account_id();
        let investment = Investment {
            investor_id: investor_id.clone(),
            amount,
        };
        self.investments.push(&investment);
    }

    // Withdraw funds from the investment contract
    pub fn withdraw(&mut self, amount: u64) {
        let investor_id = env::predecessor_account_id();
        let mut index_to_remove: Option<usize> = None;

        // Find the index of the investment record for the current investor
        for (index, record) in self.investments.iter().enumerate() {
            if &record.investor_id == &investor_id && record.amount >= amount {
                index_to_remove = Some(index);
                break;
            }
        }

        match index_to_remove {
            Some(index) => {
                // Remove the investment record
                self.investments.swap_remove(index);
                let investor_balance = env::account_balance() + amount;
                env::log(format!(
                    "Withdrawal: Account {} withdrew {} NEAR. New balance: {} NEAR",
                    investor_id, amount, investor_balance
                ).as_bytes());
                env::promise::MFTTransfer {
                    receiver_id: investor_id.clone(),
                    amount,
                    msg: "".to_string(),
                }
                .send();
            }
            None => env::panic(b"Invalid withdrawal request"),
        }
    }
}

// Entry points for the smart contract
#[cfg(target_arch = "wasm32")]
#[near_bindgen]
impl InvestmentContract {
    // Method to get the total number of investments
    pub fn total_investments(&self) -> u64 {
        self.investments.len() as u64
    }
}
