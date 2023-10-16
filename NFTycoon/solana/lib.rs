use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    program_pack::{Pack, Sealed},
    sysvar::{rent::Rent, Sysvar},
};

use solana_program::program::{invoke, invoke_signed};
use solana_program::system_instruction;
use solana_program::system_program;

use spl_token::{
    self,
    error::TokenError,
    instruction::{approve, initialize_account, initialize_mint, mint_to, transfer},
    state::{Account, Mint},
};

// Declare the program ID
solana_program::declare_id!("YourProgramID");

// Token program ID
solana_program::declare_id!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

// Token mint account address
let token_mint_address = Pubkey::new_from_array([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]);

// Main entry point of the program
#[entrypoint]
pub fn entry(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("NFTycoon Solana Smart Contract");

    let accounts_iter = &mut accounts.iter();

    // Parse instruction data and dispatch appropriate functions
    match instruction_data[0] {
        // Initialize a new token
        0 => {
            msg!("Initialize Token");
            // Initialize token mint
            initialize_mint(
                accounts_iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?,
                &token_mint_address,
                program_id,
                None,
                0,
            )?;
            Ok(())
        }

        // Mint new tokens
        1 => {
            msg!("Mint Tokens");
            let payer_account = accounts_iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?;
            let mint_account = accounts_iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?;
            let token_account = accounts_iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?;
            let owner_account = accounts_iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?;

            mint_to(
                payer_account.key,
                token_account.key,
                mint_account.key,
                owner_account.key,
                &[],
                1,
            )?;
            Ok(())
        }

        // Transfer tokens between accounts
        2 => {
            msg!("Transfer Tokens");
            let source_account = accounts_iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?;
            let destination_account = accounts_iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?;
            let owner_account = accounts_iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?;

            transfer(
                source_account.key,
                destination_account.key,
                owner_account.key,
                &[],
                1,
            )?;
            Ok(())
        }

        _ => Err(ProgramError::InvalidInstructionData),
    }
}

solana_program::entrypoint!(entry);

