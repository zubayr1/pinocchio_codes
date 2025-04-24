use pinocchio::{
    account_info::AccountInfo, instruction::{Seed, Signer}, program_error::ProgramError,
    sysvars::{ clock::Clock, Sysvar }, ProgramResult,
};
use pinocchio_log::log;

use crate::state::{StakeAccount, StakeConfig, UserAccount};

pub fn process_unstake_instruction(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [
        user,
        user_account,
        config,
        user_token,
        stake_account,
        vault,
        _token_program,
        _remaining @..
        ] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys)
    };

    if data.len() < 8 {
        return Err(ProgramError::InvalidInstructionData)
    }

    let amount = unsafe { *data.as_ptr() };

    let stake_config = unsafe { StakeConfig::from_account_info_unchecked(config) };
    let user_account = unsafe { UserAccount::from_account_info_unchecked(user_account) };
    let stake_acc = unsafe { StakeAccount::from_account_info_unchecked(stake_account) };

    // Verify ownership
    assert_eq!(stake_acc.owner, *user.key());

    // Check freeze period
    let current_time = Clock::get()?.unix_timestamp;
    if current_time - stake_acc.staked_at < stake_config.freeze_period as i64 {
        return Err(ProgramError::InvalidAccountData);
    }

    // Transfer token back to user
    let bump = [stake_config.bump.to_le()];
    let seed = [Seed::from(b"config"), Seed::from(&bump)];
    let signer = Signer::from(&seed);

    unsafe {
        log!("Transferring token back to user");
        pinocchio_token::instructions::Transfer {
            from: vault,
            to: user_token,
            authority: config,
            amount: amount.into(),
        }.invoke_signed(&[signer.clone()])?;

        // Update user account
        user_account.amount_stake -= amount;

        if user_account.amount_stake == 0 {
            // Close stake account
            pinocchio_token::instructions::CloseAccount {
                account: stake_account,
                destination: user,
                authority: config,
            }.invoke_signed(&[signer.clone()])?;

            // Close vault account
            pinocchio_token::instructions::CloseAccount {
                account: vault,
                destination: user,
                authority: config,
            }.invoke_signed(&[signer.clone()])?;

            // Transfer stake account lamports to user
            *user.borrow_mut_lamports_unchecked() += *stake_account.borrow_lamports_unchecked();
            *stake_account.borrow_mut_lamports_unchecked() = 0;
        }
    }

    Ok(())
}
