use pinocchio::{
    account_info::AccountInfo, instruction::{Seed, Signer}, program_error::ProgramError,
    sysvars::{rent::Rent, Sysvar}, ProgramResult,
};
use pinocchio_log::log;

use crate::state::UserAccount;

pub fn process_initialize_user_instruction(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [
        user,
        user_account,
        _system_program,
        _remaining @..
        ] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys)
    };

    if data.len() < 1 {
        return Err(ProgramError::InvalidInstructionData)
    }

    let bump = unsafe{ *(data.as_ptr() as *const u8) }.to_le_bytes();
    let seed = [Seed::from(b"user"), Seed::from(user.key()), Seed::from(&bump)];
    let seeds = Signer::from(&seed);

    unsafe {
        if user_account.owner() != &crate::ID {
            log!("Creating User Account");
            pinocchio_system::instructions::CreateAccount {
                from: user,
                to: user_account,
                lamports: Rent::get()?.minimum_balance(UserAccount::SIZE),
                space: UserAccount::SIZE as u64,
                owner: &crate::ID,
            }.invoke_signed(&[seeds])?;

            let user_account = UserAccount::from_account_info_unchecked(user_account);
            user_account.points = 0;
            user_account.amount_stake = 0;
            user_account.bump = *data.as_ptr();
        } else {
            return Err(ProgramError::AccountAlreadyInitialized);
        }
    }

    Ok(())
}
