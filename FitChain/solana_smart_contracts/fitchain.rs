use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    sysvar::{clock::Clock, rent::Rent, Sysvar},
    borsh::{self, BorshDeserialize, BorshSerialize},
};

// Define the state structure for FitChain Challenge
#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct Challenge {
    pub name: String,
    pub description: String,
    pub reward: u64,
    pub start_time: i64,
    pub end_time: i64,
    pub participants: Vec<Pubkey>,
    pub is_open: bool,
}

impl IsInitialized for Challenge {
    fn is_initialized(&self) -> bool {
        self.is_open
    }
}

impl Sealed for Challenge {}

solana_program::declare_id!("FitChain111111111111111111111111111111111");

// Entry point for initializing the FitChain smart contract
pub fn initialize(_program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    // Implement initialization logic here
    let account_info_iter = &mut accounts.iter();
    let challenge_account = next_account_info(account_info_iter)?;

    let mut challenge_data = Challenge::default();
    challenge_data.is_open = true;

    challenge_data.serialize(&mut &mut challenge_account.data.borrow_mut()[..])?;

    Ok(())
}

// Entry point for creating a fitness challenge
pub fn create_challenge(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    name: String,
    description: String,
    reward: u64,
    start_time: i64,
    end_time: i64,
) -> ProgramResult {
    // Implement challenge creation logic here
    let account_info_iter = &mut accounts.iter();
    let challenge_account = next_account_info(account_info_iter)?;
    let user_account = next_account_info(account_info_iter)?;

    // Check if the challenge account is owned by the program
    if challenge_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Deserialize the challenge account data
    let mut challenge_data = Challenge::deserialize(&challenge_account.data.borrow())?;

    // Check if the challenge is open
    if !challenge_data.is_open {
        return Err(ProgramError::InvalidAccountData);
    }

    // Check if the current time is after the start time
    let current_time = Clock::get()?.unix_timestamp;
    if current_time < start_time {
        return Err(ProgramError::InvalidArgument);
    }

    // Initialize the challenge data
    challenge_data.name = name;
    challenge_data.description = description;
    challenge_data.reward = reward;
    challenge_data.start_time = start_time;
    challenge_data.end_time = end_time;

    // Add the creator to the list of participants
    challenge_data.participants.push(*user_account.key);

    // Serialize and store the updated challenge data
    challenge_data.serialize(&mut &mut challenge_account.data.borrow_mut()[..])?;

    Ok(())
}

// Entry point for user participation in a challenge
pub fn join_challenge(program_id: &Pubkey, accounts: &[AccountInfo], challenge_id: u64) -> ProgramResult {
    // Implement user participation logic here
    let account_info_iter = &mut accounts.iter();
    let challenge_account = next_account_info(account_info_iter)?;
    let user_account = next_account_info(account_info_iter)?;

    // Check if the challenge account is owned by the program
    if challenge_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Deserialize the challenge account data
    let mut challenge_data = Challenge::deserialize(&challenge_account.data.borrow())?;

    // Check if the challenge is open
    if !challenge_data.is_open {
        return Err(ProgramError::InvalidAccountData);
    }

    // Check if the user has already joined the challenge
    if challenge_data.participants.contains(user_account.key) {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    // Check if the current time is within the challenge duration
    let current_time = Clock::get()?.unix_timestamp;
    if current_time < challenge_data.start_time || current_time >= challenge_data.end_time {
        return Err(ProgramError::InvalidArgument);
    }

    // Add the user to the list of participants
    challenge_data.participants.push(*user_account.key);

    // Serialize and store the updated challenge data
    challenge_data.serialize(&mut &mut challenge_account.data.borrow_mut()[..])?;

    Ok(())
}

// Entry point for ending a fitness challenge and rewarding participants
pub fn end_challenge(program_id: &Pubkey, accounts: &[AccountInfo], challenge_id: u64) -> ProgramResult {
    // Implement challenge ending and reward distribution logic here
    let account_info_iter = &mut accounts.iter();
    let challenge_account = next_account_info(account_info_iter)?;

    // Check if the challenge account is owned by the program
    if challenge_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Deserialize the challenge account data
    let mut challenge_data = Challenge::deserialize(&challenge_account.data.borrow())?;

    // Check if the challenge is open
    if !challenge_data.is_open {
        return Err(ProgramError::InvalidAccountData);
    }

    // Check if the current time is after the challenge end time
    let current_time = Clock::get()?.unix_timestamp;
    if current_time < challenge_data.end_time {
        return Err(ProgramError::InvalidArgument);
    }

    // Calculate rewards and distribute to participants
    let total_reward = challenge_data.reward;
    let num_participants = challenge_data.participants.len() as u64;

    // Calculate the reward per participant
    let reward_per_participant = total_reward / num_participants;

    // Distribute rewards to participants (simplified for demonstration)
    for participant in &challenge_data.participants {
        // Implement your token transfer logic here
        // Transfer reward_per_participant to participant's account
        // Use a token program or custom logic for token transfers
    }

    // Mark the challenge as closed
    challenge_data.is_open = false;

    // Serialize and store the updated challenge data
    challenge_data.serialize(&mut &mut challenge_account.data.borrow_mut()[..])?;

    Ok(())
}

// Entry point for retrieving challenge information
pub fn get_challenge(program_id: &Pubkey, accounts: &[AccountInfo], challenge_id: u64) -> ProgramResult {
    // Implement challenge retrieval logic here
    let account_info_iter = &mut accounts.iter();
    let challenge_account = next_account_info(account_info_iter)?;

    // Check if the challenge account is owned by the program
    if challenge_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Deserialize the challenge account data
    let challenge_data = Challenge::deserialize(&challenge
