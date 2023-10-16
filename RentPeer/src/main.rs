// src/reviews.rs

use solana_program::pubkey::Pubkey;

pub struct Review {
    pub rental_id: u64,
    pub reviewer: Pubkey,
    pub rating: u32,
    pub comment: String,
}

impl Review {
    pub fn new(rental_id: u64, reviewer: Pubkey, rating: u32, comment: String) -> Self {
        Review {
            rental_id,
            reviewer,
            rating,
            comment,
        }
    }

    // Implement functions to create and retrieve reviews.
}
