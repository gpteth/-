// SPDX-License-Identifier: MIT
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    program_pack::{Pack, IsInitialized},
};
use std::convert::TryInto;

// Define data structure for Membership
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Membership {
    pub title: String,
    pub description: String,
    pub price: u64,
    pub active: bool,
}

// Define state struct for MembershipContract
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct MembershipContract {
    pub memberships: Vec<Membership>,
}

// Implement state initialization
impl IsInitialized for MembershipContract {
    fn is_initialized(&self) -> bool {
        true
    }
}

// Define program entry point
entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Parse accounts
    let accounts_iter = &mut accounts.iter();
    let membership_account = next_account_info(accounts_iter)?;

    if instruction_data.is_empty() {
        return Err(ProgramError::InvalidInstructionData);
    }

    // Deserialize instruction data
    let instruction: Instruction = Instruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    match instruction {
        Instruction::CreateMembership {
            title,
            description,
            price,
        } => {
            // Check that the membership account is owned by the program
            if membership_account.owner != program_id {
                return Err(ProgramError::IncorrectProgramId);
            }

            // Create a new membership
            let new_membership = Membership {
                title,
                description,
                price,
                active: true,
            };

            // Deserialize the membership account
            let mut membership_data = MembershipContract::try_from_slice(&membership_account.data.borrow())?;

            // Add the new membership to the contract state
            membership_data.memberships.push(new_membership);

            // Serialize and store the updated contract state
            membership_data.serialize(&mut &mut membership_account.data.borrow_mut()[..])?;
        }
        Instruction::UpdateMembershipStatus { membership_id, active } => {
            // Check that the membership account is owned by the program
            if membership_account.owner != program_id {
                return Err(ProgramError::IncorrectProgramId);
            }

            // Deserialize the membership account
            let mut membership_data = MembershipContract::try_from_slice(&membership_account.data.borrow())?;

            // Check if the membership ID is valid
            if membership_id >= membership_data.memberships.len() {
                return Err(ProgramError::InvalidArgument);
            }

            // Update the membership status
            membership_data.memberships[membership_id].active = active;

            // Serialize and store the updated contract state
            membership_data.serialize(&mut &mut membership_account.data.borrow_mut()[..])?;
        }
        Instruction::CheckMembershipStatus { membership_id, user_account } => {
            // Deserialize the membership account
            let membership_data = MembershipContract::try_from_slice(&membership_account.data.borrow())?;

            // Check if the membership ID is valid
            if membership_id >= membership_data.memberships.len() {
                return Err(ProgramError::InvalidArgument);
            }

            // Get the membership status
            let membership_status = membership_data.memberships[membership_id].active;

            // Write the membership status to the user's account
            let mut user_account_data = user_account.data.borrow_mut();
            user_account_data[0] = if membership_status { 1 } else { 0 };
        }
    }

    Ok(())
}

// Define instruction enum
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum Instruction {
    CreateMembership {
        title: String,
        description: String,
        price: u64,
    },
    UpdateMembershipStatus {
        membership_id: u8,
        active: bool,
    },
    CheckMembershipStatus {
        membership_id: u8,
        user_account: AccountInfo,
    },
}
