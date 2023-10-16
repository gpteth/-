// Import necessary Solana libraries
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    msg,
    pubkey::Pubkey,
    program_pack::{Pack, Sealed},
    sysvar::{rent::Rent, Sysvar},
    rent::Rent as RentSysvar,
    system_instruction,
};
use borsh::{BorshDeserialize, BorshSerialize};
use thiserror::Error;

// Define the program state structure
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Strategy {
    pub owner: Pubkey,
    pub name: String,
    pub description: String,
    pub performance: u32,
    pub total_investment: u64,
}

// Define errors
#[derive(Error, Debug, Copy, Clone)]
pub enum ErrorCode {
    #[error("Invalid instruction")]
    InvalidInstruction,
}

impl From<ErrorCode> for ProgramError {
    fn from(e: ErrorCode) -> Self {
        ProgramError::Custom(e as u32)
    }
}

// Define the program ID
solana_program::declare_id!("YourProgramIdHere");

// Define the entry point for processing instructions
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = Instruction::try_from_slice(instruction_data)?;

    match instruction {
        Instruction::CreateStrategy { name, description, performance } => {
            create_strategy(accounts, name, description, performance, program_id)
        }
        Instruction::Invest { strategy_index, amount } => {
            invest(accounts, strategy_index, amount, program_id)
        }
        Instruction::CheckBalance { strategy_index } => {
            check_balance(accounts, strategy_index, program_id)
        }
    }
}

// Define the instruction data structure
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum Instruction {
    CreateStrategy {
        name: String,
        description: String,
        performance: u32,
    },
    Invest {
        strategy_index: u8,
        amount: u64,
    },
    CheckBalance {
        strategy_index: u8,
    },
}

// Define the create_strategy function
fn create_strategy(
    accounts: &[AccountInfo],
    name: String,
    description: String,
    performance: u32,
    program_id: &Pubkey,
) -> ProgramResult {
    // Check that the first account is owned by the program
    let account_info = &accounts[0];
    if account_info.owner != program_id {
        return Err(ErrorCode::InvalidInstruction.into());
    }

    // Verify that the account is rent exempt
    let rent = Rent::from_account_info(&accounts[1])?;
    if !rent.is_exempt(account_info.lamports(), account_info.data_len()) {
        return Err(ErrorCode::InvalidInstruction.into());
    }

    // Deserialize the strategy account and verify that it's not already initialized
    let mut strategy_account = Strategy::try_from_slice(&account_info.data.borrow())?;
    if strategy_account.owner != Pubkey::default() {
        return Err(ErrorCode::InvalidInstruction.into());
    }

    // Initialize the strategy account
    strategy_account.owner = *account_info.key;
    strategy_account.name = name;
    strategy_account.description = description;
    strategy_account.performance = performance;
    strategy_account.total_investment = 0;

    // Serialize and save the updated strategy account
    strategy_account.serialize(&mut &mut account_info.data.borrow_mut()[..])?;
    Ok(())
}

// Define the invest function
fn invest(
    accounts: &[AccountInfo],
    strategy_index: u8,
    amount: u64,
    program_id: &Pubkey,
) -> ProgramResult {
    // Check that the first account is owned by the program
    let account_info = &accounts[0];
    if account_info.owner != program_id {
        return Err(ErrorCode::InvalidInstruction.into());
    }

    // Deserialize the strategy account
    let mut strategy_account = Strategy::try_from_slice(&account_info.data.borrow())?;

    // Ensure that the strategy account is owned by the program and is initialized
    if strategy_account.owner != *account_info.key {
        return Err(ErrorCode::InvalidInstruction.into());
    }

    // Update the total investment in the strategy
    strategy_account.total_investment += amount;

    // Serialize and save the updated strategy account
    strategy_account.serialize(&mut &mut account_info.data.borrow_mut()[..])?;

    // Transfer SOL tokens from the user's account to the strategy account
    let source_info = next_account_info(accounts)?;
    let destination_info = next_account_info(accounts)?;

    let instruction = system_instruction::transfer(
        source_info.key,
        destination_info.key,
        amount,
    );
    msg!("Transferring {} SOL tokens to the strategy account", amount);
    solana_program::program::invoke(
        &instruction,
        accounts,
        &[source_info.clone(), destination_info.clone()],
    )?;

    Ok(())
}

// Define the check_balance function
fn check_balance(
    accounts: &[AccountInfo],
    strategy_index: u8,
    program_id: &Pubkey,
) -> ProgramResult {
    // Check that the first account is owned by the program
    let account_info = &accounts[0];
    if account
