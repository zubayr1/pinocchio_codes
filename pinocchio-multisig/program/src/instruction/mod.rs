use pinocchio::program_error::ProgramError;

// pub mod multisig_create;
pub mod multisig_config;
pub mod multisig_create;
pub mod update_members;

// pub use multisig_create::*;
pub use multisig_config::*;
pub use multisig_create::*;
pub use update_members::*;

#[repr(u8)]
pub enum MultisigInstruction {
    InitializeConfig,
    CreateMultisig,
    UpdateMembers,
}

impl TryFrom<&u8> for MultisigInstruction {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match *value {
            0 => Ok(MultisigInstruction::InitializeConfig),
            1 => Ok(MultisigInstruction::CreateMultisig),
            2 => Ok(MultisigInstruction::UpdateMembers),
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
