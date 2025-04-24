use pinocchio::{
    account_info::AccountInfo, instruction::{Seed, Signer}, program_error::ProgramError,
    sysvars::{rent::Rent, Sysvar}, ProgramResult,
};
use pinocchio_log::log;

use crate::state::StakeConfig;

pub fn process_initialize_config_instruction(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [
        authority,
        config,
        _system_program,
        _remaining @..
        ] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys)
    };

    if data.len() < 8 {
        return Err(ProgramError::InvalidInstructionData)
    }

    let bump = unsafe{ *(data.as_ptr() as *const u8) }.to_le_bytes();
    let points_per_stake = unsafe { *data.as_ptr().add(1) };
    let max_stake = unsafe { *data.as_ptr().add(2) };
    let freeze_period = unsafe { *(data.as_ptr().add(3) as *const u32) };
    let reward_bump = unsafe { *data.as_ptr().add(7) };

    let seed = [Seed::from(b"config"), Seed::from(&bump)];
    let seeds = Signer::from(&seed);

    unsafe {
        if config.owner() != &crate::ID {
            log!("Creating Config Account");
            pinocchio_system::instructions::CreateAccount {
                from: authority,
                to: config,
                lamports: Rent::get()?.minimum_balance(StakeConfig::SIZE),
                space: StakeConfig::SIZE as u64,
                owner: &crate::ID,
            }.invoke_signed(&[seeds])?;

            let config_account = StakeConfig::from_account_info_unchecked(config);
            config_account.points_per_stake = points_per_stake;
            config_account.max_stake = max_stake;
            config_account.freeze_period = freeze_period;
            config_account.reward_bump = reward_bump;
            config_account.bump = *data.as_ptr();
        } else {
            return Err(ProgramError::AccountAlreadyInitialized);
        }
    }

    Ok(())
}
