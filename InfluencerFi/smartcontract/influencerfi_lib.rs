// Import necessary dependencies for a Rust smart contract library
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;

// Define a common data structure for storing user balances
#[derive(BorshDeserialize, BorshSerialize)]
pub struct UserBalance {
    pub balance: u64,
    // Add other user balance data fields here
}

// Define a common data structure for managing allowances
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Allowance {
    pub spender_id: String,
    pub amount: u64,
    // Add other allowance data fields here
}

// Define common utility functions for handling balances and allowances
pub fn get_user_balance(
    balances: &UnorderedMap<String, UserBalance>,
    account_id: &String,
) -> u64 {
    match balances.get(account_id) {
        Some(user_balance) => user_balance.balance,
        None => 0u64,
    }
}

pub fn set_user_balance(
    balances: &mut UnorderedMap<String, UserBalance>,
    account_id: String,
    balance: u64,
) {
    let user_balance = UserBalance { balance };
    balances.insert(&account_id, &user_balance);
}

pub fn get_allowance(
    allowances: &UnorderedMap<String, Allowance>,
    owner_id: &String,
    spender_id: &String,
) -> u64 {
    match allowances.get(owner_id) {
        Some(allowance) if &allowance.spender_id == spender_id => allowance.amount,
        _ => 0u64,
    }
}

pub fn set_allowance(
    allowances: &mut UnorderedMap<String, Allowance>,
    owner_id: String,
    spender_id: String,
    amount: u64,
) {
    let allowance = Allowance {
        spender_id,
        amount,
    };
    allowances.insert(&owner_id, &allowance);
}
