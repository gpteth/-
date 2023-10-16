// src/p2p_rentals.rs

use crate::models::{Rental, User};
use solana_program::pubkey::Pubkey;

pub struct P2PRentals {
    rentals: Vec<Rental>,
    users: Vec<User>,
}

impl P2PRentals {
    pub fn new() -> Self {
        P2PRentals {
            rentals: Vec::new(),
            users: Vec::new(),
        }
    }

    pub fn create_rental(&mut self, id: u64, owner: Pubkey, item_description: String) {
        let rental = Rental::new(id, owner, item_description);
        self.rentals.push(rental);
    }

    pub fn get_rental(&self, id: u64) -> Option<&Rental> {
        self.rentals.iter().find(|&r| r.id == id)
    }

    pub fn rent_item(&mut self, rental_id: u64, renter: Pubkey) -> Result<(), &'static str> {
        if let Some(rental) = self.rentals.iter_mut().find(|r| r.id == rental_id) {
            if !rental.is_rented {
                rental.is_rented = true;
                rental.renter = renter;
                Ok(())
            } else {
                Err("Item is already rented.")
            }
        } else {
            Err("Rental not found.")
        }
    }

    pub fn return_item(&mut self, rental_id: u64) -> Result<(), &'static str> {
        if let Some(rental) = self.rentals.iter_mut().find(|r| r.id == rental_id) {
            if rental.is_rented {
                rental.is_rented = false;
                rental.renter = Pubkey::default();
                Ok(())
            } else {
                Err("Item is not currently rented.")
            }
        } else {
            Err("Rental not found.")
        }
    }

    pub fn register_user(&mut self, public_key: Pubkey, username: String) {
        let user = User {
            public_key,
            username,
            // Other user information
        };
        self.users.push(user);
    }

    pub fn get_user(&self, public_key: Pubkey) -> Option<&User> {
        self.users.iter().find(|&user| user.public_key == public_key)
    }
}
