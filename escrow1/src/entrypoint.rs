use crate::instruction::{self, EscrowProgramInstrution};
use pinocchio::{
    account_info::AccountInfo, no_allocator, nostd_panic_handler, program_entrypoint,
    program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};
use pinocchio_log::log;

// This is the entrypoint for the program.
program_entrypoint!(process_instruction);
//Do not allocate memory.
no_allocator!();
// Use the no_std panic handler.
nostd_panic_handler!();

#[inline(always)]
fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (ix_disc, instruction_data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match EscrowProgramInstrution::try_from(ix_disc)? {
        EscrowProgramInstrution::Make => {
            log!("Ix:0");
            instruction::process_make_escrow(accounts, instruction_data)
        }
        EscrowProgramInstrution::Take => {
            log!("Ix:1");
            instruction::process_take_escrow(accounts, instruction_data)
        }
    }
}
