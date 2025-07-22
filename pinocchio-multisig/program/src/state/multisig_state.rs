use super::utils::DataLen;
use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};

use crate::{
    error::MyProgramError,
    instruction::MultisigCreateIxData,
    state::try_from_account_info_mut,
};

use crate::state::Member;

#[repr(C)] //keeps the struct layout the same across different architectures
#[derive(Clone, Copy, Debug, PartialEq, shank::ShankAccount)]
pub struct MultisigState {
    pub multisig_id: u64,
    pub config_authority: Pubkey,
    pub threshold: u8,
    pub members: [Member; 10],
    pub current_index: i8,
    pub roaming_index: i8,
    pub bump: u8,
}

impl DataLen for MultisigState {
    const LEN: usize = core::mem::size_of::<MultisigState>();
}

impl MultisigState {
    pub const SEED: &'static str = "multisig";

    pub fn validate_pda(bump: u8, pda: &Pubkey, owner: &Pubkey) -> Result<(), ProgramError> {
        let seeds = &[Self::SEED.as_bytes(), owner];
        let derived = pinocchio_pubkey::derive_address(seeds, Some(bump), &crate::ID);
        if derived != *pda {
            return Err(MyProgramError::PdaMismatch.into());
        }
        Ok(())
    }

    pub fn initialize( 
        multisig_acc: &AccountInfo,
        ix_data: &MultisigCreateIxData,
        bump: u8,
    ) -> ProgramResult {
        let multisig_state = unsafe { try_from_account_info_mut::<MultisigState>(multisig_acc) }?;

        multisig_state.multisig_id = ix_data.multisig_id;
        multisig_state.config_authority = ix_data.config_authority;
        multisig_state.threshold = ix_data.threshold;
        multisig_state.members = [Member::default(); 10];
        multisig_state.current_index = 0;
        multisig_state.roaming_index = 0;
        multisig_state.bump = bump;

        Ok(())
    }
}
