#![allow(unexpected_cfgs)]

use crate::instruction::{self, MultisigInstruction};
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

    match MultisigInstruction::try_from(ix_disc)? {
        MultisigInstruction::InitializeConfig => {
            msg!("Ix:0");
            instruction::process_initialize_config(accounts, instruction_data)?;
            Ok(())

        }
        MultisigInstruction::CreateMultisig => {
            msg!("Ix:1");
            instruction::process_create_multisig(accounts, instruction_data)?;
            Ok(())
        }
        MultisigInstruction::UpdateMembers => {
            msg!("Ix:2");
            instruction::process_update_members(accounts, instruction_data)?;
            Ok(())
        }
        MultisigInstruction::CreateTransaction => {
            msg!("Ix:3");
            instruction::process_create_transaction(accounts, instruction_data)?;
            Ok(())
        }
        MultisigInstruction::CreateProposal => {
            msg!("Ix:4");
            instruction::process_create_proposal(accounts, instruction_data)?;
            Ok(())
        }
        MultisigInstruction::Vote => {
            msg!("Ix:5");
            instruction::process_vote(accounts, instruction_data)?;
            Ok(())
        }
        MultisigInstruction::UpdateProposalResult => {
            msg!("Ix:6");
            instruction::process_update_proposal_result(accounts, instruction_data)?;
            Ok(())
        }
        MultisigInstruction::ApproveTransaction => {
            msg!("Ix:7");
            instruction::process_approve_transaction(accounts, instruction_data)?;
            Ok(())
        }
        MultisigInstruction::ExecuteTransaction => {
            msg!("Ix:8");
            instruction::process_execute_transaction(accounts, instruction_data)?;
            Ok(())
        }
        MultisigInstruction::StaleTransactionIndex => {
            msg!("Ix:9");
            instruction::process_stale_transaction_index(accounts, instruction_data)?;
            Ok(())
        }
        // MultisigInstruction::UpdateConfig => {
        //     msg!("Ix:1");
        //     instruction::process_update_state_v1(accounts, instruction_data)?;
        //     instruction::process_update_state_v2(accounts, instruction_data)?;
        //     Ok(())
        // }
    }
}
