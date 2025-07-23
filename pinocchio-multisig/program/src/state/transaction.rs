use super::utils::DataLen;
use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, ProgramResult,
    instruction::Instruction,
};

use crate::{
    error::MyProgramError,
    state::try_from_account_info_mut,
};
use pinocchio::instruction::AccountMeta;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TransactionStatus {
    Pending,
    Approved,
    Executed,
    Rejected,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CompiledInstruction {
    pub program_id_index: u8,
    pub account_indexes: [u8; 10],
    pub data: [u8; 32],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, shank::ShankAccount)]
pub struct Transaction {
    pub multisig_id: u64,
    pub transaction_index: u64,
    pub creator: Pubkey,
    pub status: TransactionStatus,
    pub account_keys: [Pubkey; 10],
    pub instructions: [CompiledInstruction; 20],
    pub num_account_keys: u8,
    pub num_instructions: u8,
    pub bump: u8,
}

impl DataLen for Transaction {
    const LEN: usize = core::mem::size_of::<Transaction>();
}

impl Transaction {
    pub const SEED: &'static str = "transaction";

    pub fn validate_pda(bump: u8, pda: &Pubkey, multisig_id: u64, transaction_index: u64) -> Result<(), ProgramError> {
        let seeds = &[
            Self::SEED.as_bytes(), 
            &multisig_id.to_le_bytes(),
            &transaction_index.to_le_bytes()
        ];
        let derived = pinocchio_pubkey::derive_address(seeds, Some(bump), &crate::ID);
        if derived != *pda {
            return Err(MyProgramError::PdaMismatch.into());
        }
        Ok(())
    }

    pub fn initialize(
        transaction_acc: &AccountInfo,
        multisig_id: u64,
        transaction_index: u64,
        creator: Pubkey,
        account_keys: [Pubkey; 10],
        instructions: [CompiledInstruction; 20],
        num_account_keys: u8,
        num_instructions: u8,
        bump: u8,
    ) -> ProgramResult {
        let transaction_state = unsafe { try_from_account_info_mut::<Transaction>(transaction_acc) }?;

        transaction_state.multisig_id = multisig_id;
        transaction_state.transaction_index = transaction_index;
        transaction_state.creator = creator;
        transaction_state.status = TransactionStatus::Pending;
        transaction_state.account_keys = account_keys;
        transaction_state.instructions = instructions;
        transaction_state.num_account_keys = num_account_keys;
        transaction_state.num_instructions = num_instructions;
        transaction_state.bump = bump;

        Ok(())
    }

    pub fn approve(transaction_acc: &AccountInfo) -> ProgramResult {
        let transaction_state = unsafe { try_from_account_info_mut::<Transaction>(transaction_acc) }?;
        transaction_state.status = TransactionStatus::Approved;
        Ok(())
    }

    pub fn execute(transaction_acc: &AccountInfo) -> ProgramResult {
        let transaction_state = unsafe { try_from_account_info_mut::<Transaction>(transaction_acc) }?;

        for i in 0..transaction_state.num_instructions as usize {
            let instruction = &transaction_state.instructions[i];
            
            let program_id = transaction_state.account_keys[instruction.program_id_index as usize];
            
            // let mut account_metas = [AccountMeta::new(&Pubkey::default(), false, true); 10];
            
            // for j in 0..transaction_state.num_account_keys as usize {
            //     let actual_account = transaction_state.account_keys[j];
            //     account_metas[j] = AccountMeta::new(&actual_account, false, true);
            // }
            
            let instruction_data = instruction.data;
            
            match program_id {
                // System Program - for SOL transfers
                pinocchio_system::ID => {
                    // let system_instruction = Instruction {
                    //     program_id: &program_id,
                    //     accounts: &account_metas[..transaction_state.num_account_keys as usize],
                    //     data: &instruction_data[..],
                    // };
                },
                
                // Token Program - for SPL token transfers  
                pinocchio_token::ID => {
                    // let token_instruction = Instruction {
                    //     program_id: &program_id,
                    //     accounts: &account_metas[..transaction_state.num_account_keys as usize],
                    //     data: &instruction_data[..],
                    // };
                },
                
                // Any other program
                _ => {
                    // todo
                }
            }
        }
        
        transaction_state.status = TransactionStatus::Executed;
        Ok(())
    }

    pub fn reject(transaction_acc: &AccountInfo) -> ProgramResult {
        let transaction_state = unsafe { try_from_account_info_mut::<Transaction>(transaction_acc) }?;
        transaction_state.status = TransactionStatus::Rejected;
        Ok(())
    }
}
