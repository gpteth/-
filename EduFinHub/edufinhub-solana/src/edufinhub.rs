#![cfg(feature = "program")]

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    program_error::ProgramError,
    msg,
    program_pack::{Pack, IsInitialized},
    sysvar::{rent::Rent, Sysvar},
};

// Define the data structures for courses and user courses
#[derive(Debug, Default, PartialEq)]
pub struct Course {
    pub name: String,
    pub description: String,
    pub price: u64,
}

#[derive(Debug, Default, PartialEq)]
pub struct UserCourse {
    pub course_id: u32,
    pub status: String,
}

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    // Get the first account which is expected to be the user's account
    let user_account = next_account_info(accounts_iter)?;

    // Ensure the user's account is initialized
    if !user_account.is_initialized() {
        return Err(ProgramError::UninitializedAccount);
    }

    // Deserialize the instruction data to identify the action
    let action = match instruction_data.get(0) {
        Some(&action) => action,
        None => {
            msg!("No action provided in the instruction");
            return Err(ProgramError::InvalidInstructionData);
        }
    };

    match action {
        // Action 1: Enroll in a course
        1 => {
            // Parse the course ID from the instruction data (assuming it's a u32)
            let course_id_bytes = instruction_data[1..5].try_into().expect("Invalid course ID length");
            let course_id = u32::from_le_bytes(course_id_bytes);

            // Load the course account based on the course ID
            let course_account_info = next_account_info(accounts_iter)?;
            let course_data = Course::unpack_from_slice(&course_account_info.data.borrow())?;
            
            // Implement the enrollment logic (e.g., check if the user can enroll, deduct funds, update status)
            // ...

            msg!("Enrollment successful");
        }

        // Add more actions as needed for course management, payments, etc.
        _ => {
            msg!("Invalid action");
            return Err(ProgramError::InvalidInstructionData);
        }
    }

    Ok(())
}
