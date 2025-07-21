#![allow(unexpected_cfgs)]

use crate::instruction::{self, FundraisingVestingInstruction};
use pinocchio::{
    account_info::AccountInfo, default_panic_handler, msg, no_allocator, program_entrypoint,
    program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};

// This is the entrypoint for the program.
program_entrypoint!(process_instruction);
//Do not allocate memory.
no_allocator!();
// Use the no_std panic handler.
default_panic_handler!();

#[inline(always)]
fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (ix_disc, instruction_data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match FundraisingVestingInstruction::try_from(ix_disc)? {
        FundraisingVestingInstruction::InitializeFundraising => {
            msg!("Ix:0");
            instruction::process_initilaize_fundraising(accounts, instruction_data)
        }
        FundraisingVestingInstruction::Deposit => {
            msg!("Ix:1");
            instruction::process_deposit(accounts, instruction_data)
        }
        FundraisingVestingInstruction::Withdraw => {
            msg!("Ix:2");
            instruction::process_withdraw(accounts, instruction_data)
        }
    }
}
