use pinocchio::program_error::ProgramError;

// pub mod multisig_create;
pub mod multisig_config;
pub mod multisig_create;
pub mod update_members;
pub mod create_proposal;
pub mod create_transaction;
pub mod vote;
pub mod update_proposal_result;
pub mod approve_transaction;
pub mod execute_transaction;
pub mod stale_transaction_index;

// pub use multisig_create::*;
pub use multisig_config::*;
pub use multisig_create::*;
pub use update_members::*;
pub use create_proposal::*;
pub use create_transaction::*;
pub use vote::*;
pub use update_proposal_result::*;
pub use approve_transaction::*;
pub use execute_transaction::*;
pub use stale_transaction_index::*;

#[repr(u8)]
pub enum MultisigInstruction {
    InitializeConfig,
    CreateMultisig,
    UpdateMembers,
    CreateProposal,
    CreateTransaction,
    Vote,
    UpdateProposalResult,
    ApproveTransaction,
    ExecuteTransaction,
    StaleTransactionIndex,
}

impl TryFrom<&u8> for MultisigInstruction {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match *value {
            0 => Ok(MultisigInstruction::InitializeConfig),
            1 => Ok(MultisigInstruction::CreateMultisig),
            2 => Ok(MultisigInstruction::UpdateMembers),
            3 => Ok(MultisigInstruction::CreateTransaction),
            4 => Ok(MultisigInstruction::CreateProposal),            
            5 => Ok(MultisigInstruction::Vote),
            6 => Ok(MultisigInstruction::UpdateProposalResult),
            7 => Ok(MultisigInstruction::ApproveTransaction),
            8 => Ok(MultisigInstruction::ExecuteTransaction),
            9 => Ok(MultisigInstruction::StaleTransactionIndex),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}

// mod idl_gen {
//     use super::InitializeMyStateV1IxData;

//     #[derive(shank::ShankInstruction)]
//     enum _MyProgramInstruction {
//         #[account(0, writable, signer, name = "payer_acc", desc = "Fee payer account")]
//         #[account(1, writable, name = "state_acc", desc = "New State account")]
//         #[account(2, name = "sysvar_rent_acc", desc = "Sysvar rent account")]
//         #[account(3, name = "system_program_acc", desc = "System program account")]
//         InitializeState(InitializeMyStateV1IxData),
//         #[account(0, writable, signer, name = "payer_acc", desc = "Fee payer account")]
//         #[account(1, writable, name = "state_acc", desc = "State account")]
//         UpdateState,
//     }
// }
