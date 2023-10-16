// SPDX-License-Identifier: Apache-2.0

//! A simple Solana smart contract for AutoChain

use solana_program::{
    account_info::next_account_info,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    program_error::ProgramError,
    program_pack::Pack,
    sysvar::{rent::Rent, Sysvar},
};

use borsh::{BorshDeserialize, BorshSerialize};

/// Define the AutoChain data structure
#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct AutoChain {
    pub owner: Pubkey,
    pub make: String,
    pub model: String,
    pub year: u16,
    pub price: u64,
    pub is_sold: bool,
}

/// Entry point function to create a new AutoChain smart contract
pub fn create_autochain(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    make: String,
    model: String,
    year: u16,
    price: u64,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let owner_info = next_account_info(accounts_iter)?;
    let autochain_account = next_account_info(accounts_iter)?;

    // Check if the owner of the smart contract is the caller
    if owner_info.key != &autochain_account.owner {
        msg!("Caller is not the owner");
        return Err(ProgramError::InvalidAccountData);
    }

    // Check if the AutoChain account already exists
    if autochain_account.data.borrow().len() != 0 {
        msg!("AutoChain account already exists");
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    // Create a new AutoChain instance and serialize it
    let autochain_data = AutoChain {
        owner: *owner_info.key,
        make,
        model,
        year,
        price,
        is_sold: false,
    };

    // Serialize and save the AutoChain data to the account
    autochain_data.serialize(&mut &mut autochain_account.data.borrow_mut()[..])?;

    Ok(())
}

/// Entry point function to buy a vehicle from the AutoChain
pub fn buy_autochain(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let buyer_info = next_account_info(accounts_iter)?;
    let seller_info = next_account_info(accounts_iter)?;
    let autochain_account = next_account_info(accounts_iter)?;

    // Check if the buyer is different from the seller
    if buyer_info.key == seller_info.key {
        msg!("Buyer and seller cannot be the same");
        return Err(ProgramError::InvalidAccountData);
    }

    // Deserialize the AutoChain data from the account
    let mut autochain_data = AutoChain::try_from_slice(&autochain_account.data.borrow())?;

    // Check if the vehicle is already sold
    if autochain_data.is_sold {
        msg!("Vehicle is already sold");
        return Err(ProgramError::InvalidAccountData);
    }

    // Transfer ownership
    autochain_data.is_sold = true;
    autochain_data.owner = *buyer_info.key;

    // Serialize and update the AutoChain data in the account
    autochain_data.serialize(&mut &mut autochain_account.data.borrow_mut()[..])?;

    Ok(())
}

entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    match instruction_data[0] {
        // Create a new AutoChain
        0 => {
            let make = String::from_utf8(instruction_data[1..33].to_vec()).unwrap();
            let model = String::from_utf8(instruction_data[33..65].to_vec()).unwrap();
            let year = u16::from_le_bytes([instruction_data[65], instruction_data[66]]);
            let price = u64::from_le_bytes([
                instruction_data[67],
                instruction_data[68],
                instruction_data[69],
                instruction_data[70],
                instruction_data[71],
                instruction_data[72],
                instruction_data[73],
                instruction_data[74],
            ]);

            create_autochain(program_id, accounts, make, model, year, price)
        }
        // Buy a vehicle from AutoChain
        1 => {
            buy_autochain(program_id, accounts)
        }
        _ => {
            msg!("Invalid instruction");
            Err(ProgramError::InvalidInstructionData)
        }
    }
}
