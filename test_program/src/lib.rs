use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    ProgramResult,
    program_error::ProgramError,
    program_pack::{Pack, PackDeserialize, PackSerialize},
    pubkey::Pubkey,
};

use pinocchio_pubkey::declare_id;

pub mod states;
pub mod instructions;

pub use states::*;
pub use instructions::*;

entrypoint!(process_instruction);

declare_id!("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8");

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let [ix_disc, instruction_data] = instruction_data.split_first().ok_or(ProgramError::InvalidInstructionData)?;

    match ix_disc {
        0 => {
            let ix = Mint::unpack(instruction_data)?;
        }
        _ => return Err(ProgramError::InvalidInstructionData),
    }

    Ok(())
}
