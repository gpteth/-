#![cfg(feature = "program")]

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::{self, Sysvar},
    program,
};

program! {
    pub mod racetomars {
        use super::*;

        pub fn create_spacecraft(ctx: Context, name: String) -> ProgramResult {
            let spacecraft = Spacecraft {
                owner: *ctx.accounts.owner.key,
                name,
                parts: 0,
                is_on_mars: false,
            };

            let accounts = &mut ctx.accounts;
            accounts.spacecraft.data = spacecraft.try_to_vec()?;
            Ok(())
        }

        pub fn launch_to_mars(ctx: Context) -> ProgramResult {
            let accounts = &mut ctx.accounts;

            let mut spacecraft = Spacecraft::try_from_slice(&accounts.spacecraft.data.borrow())?;
            if spacecraft.is_on_mars {
                msg!("Spacecraft is already on Mars");
                return Err(ProgramError::InvalidAccountData);
            }

            // Implement logic to deduct parts (not shown in this simplified example)

            spacecraft.is_on_mars = true;
            spacecraft.serialize(&mut &mut accounts.spacecraft.data.borrow_mut()[..])?;
            Ok(())
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct Spacecraft {
    owner: Pubkey,
    name: String,
    parts: u64,
    is_on_mars: bool,
}

impl Spacecraft {
    fn deduct_parts(&mut self, parts_to_deduct: u64) -> Result<(), ProgramError> {
        if self.parts < parts_to_deduct {
            msg!("Not enough parts to deduct");
            return Err(ProgramError::InvalidArgument);
        }
        self.parts -= parts_to_deduct;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Context<'info> {
    #[account(signer)]
    owner: AccountInfo<'info>,
    #[account(mut)]
    spacecraft: AccountInfo<'info>,
    rent: Sysvar<'info, Rent>,
}
