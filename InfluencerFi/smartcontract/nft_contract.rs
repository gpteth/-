// Import necessary dependencies for a Rust smart contract
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::Vector;
use near_sdk::env;
use near_sdk::near_bindgen;

// Define the data structure for an NFT
#[derive(BorshDeserialize, BorshSerialize)]
struct NFT {
    owner_id: String,
    token_id: u64,
    // Add other NFT data fields here
}

// Define the data structure for the NFT contract
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
struct NFTContract {
    owner_id: String,
    nfts: Vector<NFT>,
}

// Implement methods for the NFT contract
#[near_bindgen]
impl NFTContract {
    // Constructor to initialize the NFT contract
    pub fn new(owner_id: String) -> Self {
        Self {
            owner_id,
            nfts: Vector::new(b"n".to_vec()),
        }
    }

    // Mint a new NFT and assign it to the caller
    pub fn mint_nft(&mut self) {
        let token_id = self.nfts.len() as u64;
        let owner_id = env::predecessor_account_id();
        let nft = NFT { owner_id, token_id };
        self.nfts.push(&nft);
    }
}

// Entry points for the smart contract
#[cfg(target_arch = "wasm32")]
#[near_bindgen]
impl NFTContract {
    // Method to get the total number of NFTs
    pub fn total_nfts(&self) -> u64 {
        self.nfts.len() as u64
    }
}
