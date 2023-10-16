// marketplace.rs

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

// Define a structure to represent a marketplace item
#[derive(Debug)]
pub struct MarketplaceItem {
    pub seller: Pubkey,
    pub item_id: u64,
    pub price: u64,
    pub is_sold: bool,
}

pub struct Marketplace;

impl Marketplace {
    pub fn list_for_sale(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        item_id: u64,
        price: u64,
    ) -> ProgramResult {
        // Ensure the correct accounts are provided
        let account_iter = &mut accounts.iter();
        let seller_account = next_account_info(account_iter)?;

        // Load or create an account to store the marketplace item
        let item_account = next_account_info(account_iter)?;

        // Deserialize the data to check if the item already exists
        let mut item_data = item_account.try_borrow_mut_data()?;
        let mut marketplace_item = MarketplaceItem::deserialize(&item_data);

        // Check if the item is already listed or sold
        if marketplace_item.is_sold {
            return Err(ProgramError::InvalidInstructionData);
        }

        // Update the marketplace item with the new listing
        marketplace_item = MarketplaceItem {
            seller: *seller_account.key,
            item_id,
            price,
            is_sold: false,
        };

        // Serialize and save the updated item back to the account data
        item_data.copy_from_slice(&marketplace_item.serialize());

        Ok(())
    }

    pub fn buy(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        item_id: u64,
    ) -> ProgramResult {
        // Ensure the correct accounts are provided
        let account_iter = &mut accounts.iter();
        let buyer_account = next_account_info(account_iter)?;
        let seller_account = next_account_info(account_iter)?;

        // Load the marketplace item account
        let item_account = next_account_info(account_iter)?;

        // Deserialize the data to retrieve the marketplace item
        let item_data = item_account.try_borrow_mut_data()?;
        let mut marketplace_item = MarketplaceItem::deserialize(&item_data);

        // Check if the item is already sold or if the buyer is the seller
        if marketplace_item.is_sold || *buyer_account.key == marketplace_item.seller {
            return Err(ProgramError::InvalidInstructionData);
        }

        // Transfer funds from the buyer to the seller
        // In a real-world scenario, you would need to implement the token transfer logic

        // Mark the item as sold
        marketplace_item.is_sold = true;

        // Serialize and update the item back to the account data
        item_data.copy_from_slice(&marketplace_item.serialize());

        Ok(())
    }

    pub fn get_item_status(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        // Ensure the correct accounts are provided
        let account_iter = &mut accounts.iter();
        let item_account = next_account_info(account_iter)?;

        // Deserialize the data to retrieve the marketplace item
        let item_data = item_account.try_borrow_mut_data()?;
        let marketplace_item = MarketplaceItem::deserialize(&item_data);

        // Return the status of the item (listed or sold)
        if marketplace_item.is_sold {
            msg!("Item is sold.");
        } else {
            msg!("Item is listed for sale.");
        }

        Ok(())
    }
}

impl MarketplaceItem {
    // Serialize the marketplace item into bytes
    fn serialize(&self) -> [u8; 24] {
        let mut data = [0u8; 24];
        data[..8].copy_from_slice(&self.seller.to_bytes());
        data[8..16].copy_from_slice(&self.item_id.to_le_bytes());
        data[16..24].copy_from_slice(&self.price.to_le_bytes());
        data
    }

    // Deserialize bytes into a marketplace item
    fn deserialize(data: &[u8]) -> Self {
        let seller = Pubkey::new(&data[..8]);
        let item_id = u64::from_le_bytes(data[8..16].try_into().unwrap());
        let price = u64::from_le_bytes(data[16..24].try_into().unwrap());
        let is_sold = false; // Initialize as not sold

        MarketplaceItem {
            seller,
            item_id,
            price,
            is_sold,
        }
    }
}
