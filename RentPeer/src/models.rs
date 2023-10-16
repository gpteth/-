// src/models.rs

use solana_program::pubkey::Pubkey;

pub struct User {
    pub public_key: Pubkey,
    pub username: String,
    // Other user information
}

pub struct Rental {
    pub id: u64,
    pub owner: Pubkey,
    pub renter: Pubkey,
    pub item_description: String,
    pub is_rented: bool,
    // Other rental information
}

pub struct Review {
    pub rental_id: u64,
    pub reviewer: Pubkey,
    pub rating: u32,
    pub comment: String,
}

pub struct Payment {
    pub rental_id: u64,
    pub payer: Pubkey,
    pub amount: u64,
    // Other payment information
}

pub struct Dispute {
    pub rental_id: u64,
    pub participants: [Pubkey; 2],
    pub description: String,
}
