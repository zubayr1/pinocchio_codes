use instructions::StakingInstructions;
use pinocchio::{
    account_info::AccountInfo, entrypoint, program_error::ProgramError, pubkey::Pubkey,
    ProgramResult,
};

mod instructions;
mod state;

entrypoint!(process_instruction);

const ID: Pubkey = five8_const::decode_32_const("88888888888888888888888888888888888888888888");

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    assert_eq!(program_id, &ID);

    let (discriminator, data) = instruction_data.split_first().ok_or(ProgramError::InvalidAccountData)?;

    match StakingInstructions::try_from(*discriminator)? {
        StakingInstructions::InitializeConfig => instructions::process_initialize_config_instruction(accounts, data)?,
        StakingInstructions::InitializeUser => instructions::process_initialize_user_instruction(accounts, data)?,
        StakingInstructions::Stake => instructions::process_stake_instruction(accounts, data)?,
        StakingInstructions::Unstake => instructions::process_unstake_instruction(accounts, data)?,
    }
    Ok(())
}

