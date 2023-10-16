// lib.rs

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
    program_pack::Pack,
    clock::Clock,
};

use borsh::{BorshDeserialize, BorshSerialize};

// Define the data structure for audits
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Audit {
    pub name: String,
    pub status: String,
    pub owner: Pubkey,
    pub created_at: u64,
}

// Define the program ID
pub const PROGRAM_ID: Pubkey = Pubkey::new_from_array([0u8; 32]);

// Define the instruction for creating a new audit
pub struct CreateAudit<'a> {
    pub name: &'a str,
    pub status: &'a str,
}

// Define the error type for the program
#[derive(Debug)]
pub enum Error {
    InvalidInstruction,
    NotRentExempt,
    AccountAlreadyInitialized,
    OwnerMismatch,
    InsufficientFunds,
}

impl From<Error> for ProgramError {
    fn from(e: Error) -> Self {
        ProgramError::Custom(e as u32)
    }
}

// Create and initialize a new audit
pub fn create_audit(
    accounts: &[AccountInfo],
    data: CreateAudit,
) -> ProgramResult {
    // Verify that the accounts provided are correct
    let accounts_iter = &mut accounts.iter();
    let audit_account = next_account_info(accounts_iter)?;
    let owner_account = next_account_info(accounts_iter)?;
    let system_program_account = next_account_info(accounts_iter)?;
    let rent_sysvar_account = next_account_info(accounts_iter)?;
    let clock_sysvar_account = next_account_info(accounts_iter)?;

    // Ensure that the owner of the audit account matches the caller
    if audit_account.owner != PROGRAM_ID {
        return Err(Error::InvalidInstruction.into());
    }

    // Check if the audit account is rent-exempt
    let rent = Rent::from_account_info(rent_sysvar_account)?;
    if !rent.is_exempt(audit_account.lamports(), audit_account.data_len()) {
        return Err(Error::NotRentExempt.into());
    }

    // Deserialize the audit account data
    let mut audit_data = Audit::try_from_slice(&audit_account.data.borrow())?;

    // Verify that the audit account has not been initialized
    if audit_data.created_at != 0 {
        return Err(Error::AccountAlreadyInitialized.into());
    }

    // Verify that the owner of the audit matches the caller
    if *owner_account.key != audit_data.owner {
        return Err(Error::OwnerMismatch.into());
    }

    // Verify that the caller has enough funds to create the audit account
    if audit_account.lamports() < rent.minimum_balance(audit_account.data_len()) {
        return Err(Error::InsufficientFunds.into());
    }

    // Initialize the audit account with the provided data
    audit_data = Audit {
        name: data.name.to_string(),
        status: data.status.to_string(),
        owner: *owner_account.key,
        created_at: Clock::from_account_info(clock_sysvar_account)?.unix_timestamp,
    };

    // Serialize and store the updated audit data
    audit_data.serialize(&mut &mut audit_account.data.borrow_mut()[..])?;

    // Send lamports to the system program to create the account
    **audit_account.lamports.borrow_mut() -= rent.minimum_balance(audit_account.data_len());
    **system_program_account.lamports.borrow_mut() += rent.minimum_balance(audit_account.data_len());

    Ok(())
}

// Entry point for the smart contract
entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if *program_id != PROGRAM_ID {
        return Err(Error::InvalidInstruction.into());
    }

    // Parse the instruction data
    let instruction = CreateAudit::try_from_slice(instruction_data)?;

    // Dispatch the instruction
    match instruction {
        CreateAudit { name, status } => {
            create_audit(accounts, CreateAudit { name, status })?;
        }
    }

    Ok(())
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::{
        clock::Epoch,
        program_pack::Pack,
        sysvar::{clock::Clock, rent::Rent, Sysvar},
    };
    use solana_program_test::*;
    use solana_sdk::{account::Account, signature::Signer, transaction::Transaction};

    #[tokio::test]
    async fn test_create_audit() {
        let program_id = Pubkey::new_unique();
        let (mut banks_client, payer, recent_blockhash) = ProgramTest::new(
            "solana_audit_program",
            program_id,
            processor!(process_instruction),
        )
        .start()
        .await;

        let audit_owner = Keypair::new();
        let audit_account = Keypair::new();

        // Create the audit account
        let mut transaction = Transaction::new_with_payer(
            &[system_instruction::create_account(
                &payer.pubkey(),
                &audit_account.pubkey(),
                1_000_000, // lamports
                1024,      // data_len
                &program_id,
            )],
            Some(&payer.pubkey()),
        );

        transaction.sign(&[&payer, &audit_account], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Initialize the audit account
        let instruction_data = CreateAudit {
            name: "Audit 1",
            status: "Pending",
        }
        .try_to_vec()
        .unwrap();

        let transaction = Transaction::new_with_payer(
            &[Instruction::new_with_bincode(
                program_id,
                &instruction_data,
                vec![
                    AccountMeta::new(audit_account.pubkey(), false),
                    AccountMeta::new(audit_owner.pubkey(), false),
                    AccountMeta::new(system_program::id(), false),
                    AccountMeta::new(rent::id(), false),
                    AccountMeta::new(clock::id(), false),
                ],
            )],
            Some(&payer.pubkey()),
        );

        banks_client.process_transaction(transaction).await.unwrap();

        // Retrieve and deserialize the audit account data
        let audit_account_info = banks_client.get_account(&audit_account.pubkey()).await.unwrap().unwrap();
        let audit_data = Audit::try_from_slice(&audit_account_info.data).unwrap();

        // Verify that the audit data has been correctly initialized
        assert_eq!(audit_data.name, "Audit 1");
        assert_eq!(audit_data.status, "Pending");
        assert_eq!(audit_data.owner, audit_owner.pubkey());
        assert_ne!(audit_data.created_at, 0);
    }
}
