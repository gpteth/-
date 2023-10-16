// SPDX-License-Identifier: Apache-2.0
#![allow(unused_attributes)]

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::rent::Rent,
};

// Import the Marketplace module
mod marketplace;

// Declare the GreenToken program ID
solana_program::declare_id!("GreenTokn11111111111111111111111111111111");

struct GreenToken;

impl GreenToken {
    fn mint(program_id: &Pubkey, accounts: &[AccountInfo], amount: u64) -> ProgramResult {
        let account_iter = &mut accounts.iter();
        let token_account = next_account_info(account_iter)?;
        let mint_authority = next_account_info(account_iter)?;

        // Ensure the mint_authority is correct (you can implement your own authorization logic)

        // Mint new tokens
        let mut token_data = token_account.try_borrow_mut_data()?;
        let mut token_balance = u64::from_le_bytes(*token_data);

        token_balance = token_balance.checked_add(amount).ok_or(ProgramError::Overflow)?;
        token_data.copy_from_slice(&token_balance.to_le_bytes());

        msg!("GreenTokens minted successfully!");
        Ok(())
    }

    fn transfer(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        from_amount: u64,
        to_account: &Pubkey,
    ) -> ProgramResult {
        let account_iter = &mut accounts.iter();
        let from_account = next_account_info(account_iter)?;
        let to_account_info = next_account_info(account_iter)?;

        // Ensure the correct transfer logic based on your requirements
        // Check balances, authorizations, and perform the transfer

        // Deduct from sender's balance
        let mut from_balance_data = from_account.try_borrow_mut_data()?;
        let mut from_balance = u64::from_le_bytes(*from_balance_data);

        from_balance = from_balance.checked_sub(from_amount).ok_or(ProgramError::InsufficientFunds)?;
        from_balance_data.copy_from_slice(&from_balance.to_le_bytes());

        // Add to receiver's balance
        let mut to_balance_data = to_account_info.try_borrow_mut_data()?;
        let mut to_balance = u64::from_le_bytes(*to_balance_data);

        to_balance = to_balance.checked_add(from_amount).ok_or(ProgramError::Overflow)?;
        to_balance_data.copy_from_slice(&to_balance.to_le_bytes());

        msg!("GreenTokens transferred successfully!");
        Ok(())
    }
}

// Define the entry point for processing instructions
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = instruction_data[0]; // The first byte indicates the instruction type

    match instruction {
        0 => GreenToken::mint(program_id, accounts, 0), // Implement mint instruction
        1 => {
            let from_amount = u64::from_le_bytes(instruction_data[1..9].try_into().unwrap());
            let to_account_bytes = instruction_data[9..41].try_into().unwrap();
            let to_account = Pubkey::new(&to_account_bytes);
            GreenToken::transfer(program_id, accounts, from_amount, &to_account) // Implement transfer instruction
        }
        2 => {
            // Call the list_for_sale function from the marketplace module
            marketplace::Marketplace::list_for_sale(/* pass necessary arguments */)
        }
        3 => {
            // Call the buy function from the marketplace module
            marketplace::Marketplace::buy(/* pass necessary arguments */)
        }
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

entrypoint!(process_instruction);
