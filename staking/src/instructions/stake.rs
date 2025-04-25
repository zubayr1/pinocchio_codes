use pinocchio::{
    account_info::AccountInfo, instruction::{Seed, Signer}, program_error::ProgramError,
    sysvars::{clock::Clock, rent::Rent, Sysvar}, ProgramResult,
};
use pinocchio_log::log;
use pinocchio_token::state::TokenAccount;

use crate::state::{StakeAccount, StakeConfig, UserAccount};

pub fn process_stake_instruction(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [
        user,
        user_account,
        config,
        user_token,
        user_token_ata,
        stake_account,
        vault,
        _system_program,
        _token_program,
        _remaining @..
        ] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys)
    };

    if data.len() < 9 {
        return Err(ProgramError::InvalidInstructionData)
    }

    let bump = unsafe{ *(data.as_ptr() as *const u8) }.to_le_bytes();
    let amount = unsafe { *data.as_ptr().add(1) };
    let config = unsafe { StakeConfig::from_account_info_unchecked(config) };
    let user_account = unsafe { UserAccount::from_account_info_unchecked(user_account) };
    let user_token_ata_acc = unsafe { TokenAccount::from_account_info_unchecked(user_token_ata) };

    // Check if user hasn't exceeded max stake limit
    if user_account.amount_stake >= config.max_stake {
        return Err(ProgramError::InvalidAccountData);
    }

    let binding = user_account.amount_stake.to_le_bytes();
    let seed = [
        Seed::from(b"stake"),
        Seed::from(user.key()),
        Seed::from(&binding),
        Seed::from(&bump),
    ];
    let seeds = Signer::from(&seed);

    unsafe {
        if stake_account.owner() != &crate::ID {
            log!("Creating Stake Account");
            pinocchio_system::instructions::CreateAccount {
                from: user,
                to: stake_account,
                lamports: Rent::get()?.minimum_balance(StakeAccount::SIZE),
                space: StakeAccount::SIZE as u64,
                owner: &crate::ID,
            }.invoke_signed(&[seeds])?;

            let stake_acc = StakeAccount::from_account_info_unchecked(stake_account);
            stake_acc.owner = *user.key();
            stake_acc.mint = *TokenAccount::from_account_info(user_token)?.mint();
            stake_acc.staked_at = Clock::get()?.unix_timestamp;
            stake_acc.bump = *data.as_ptr();

            // Transfer token to vault
            pinocchio_token::instructions::Transfer {
                from: user_token_ata_acc,
                to: vault,
                authority: user,
                amount: amount.into(),
            }.invoke()?;

            // Update user account
            user_account.amount_stake += amount;
            user_account.points += config.points_per_stake as u64;
        } else {
            return Err(ProgramError::AccountAlreadyInitialized);
        }
    }

    Ok(())
}
