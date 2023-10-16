// src/dispute.rs

use solana_program::pubkey::Pubkey;

pub struct Dispute {
    pub rental_id: u64,
    pub participants: [Pubkey; 2],
    pub description: String,
}

impl Dispute {
    pub fn new(rental_id: u64, participant1: Pubkey, participant2: Pubkey, description: String) -> Self {
        Dispute {
            rental_id,
            participants: [participant1, participant2],
            description,
        }
    }

    // Implement functions for dispute resolution logic.
    pub fn initiate_dispute() {
        // Implement logic for initiating a dispute
    }

    pub fn resolve_dispute() {
        // Implement logic for resolving a dispute
    }
}
