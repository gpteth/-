// Import necessary dependencies for a Rust smart contract
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env;
use near_sdk::near_bindgen;

// Define the data structure for the influencer token
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
struct InfluencerToken {
    owner_id: String,
    token_name: String,
    total_supply: u64,
    balances: std::collections::HashMap<String, u64>,
}

// Implement methods for the influencer token smart contract
#[near_bindgen]
impl InfluencerToken {
    // Constructor to initialize the token
    pub fn new(owner_id: String, token_name: String, total_supply: u64) -> Self {
        let mut balances = std::collections::HashMap::new();
        balances.insert(owner_id.clone(), total_supply);
        Self {
            owner_id,
            token_name,
            total_supply,
            balances,
        }
    }

    // Transfer tokens from one account to another
    pub fn transfer(&mut self, receiver_id: String, amount: u64) {
        // Check if sender has enough balance
        let sender_id = env::predecessor_account_id();
        let sender_balance = self.balances.get(&sender_id).unwrap_or(&0u64);
        assert!(sender_balance >= &amount, "Insufficient balance");

        // Update sender and receiver balances
        let receiver_balance = self.balances.get(&receiver_id).unwrap_or(&0u64);
        self.balances.insert(sender_id.clone(), sender_balance - amount);
        self.balances.insert(receiver_id.clone(), receiver_balance + amount);
    }
}

// Entry points for the smart contract
#[cfg(target_arch = "wasm32")]
#[near_bindgen]
impl InfluencerToken {
    // Method to get the balance of an account
    pub fn get_balance(&self, account_id: String) -> u64 {
        self.balances.get(&account_id).cloned().unwrap_or(0)
    }
}
