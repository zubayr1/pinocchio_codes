use super::utils::DataLen;
use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};

use crate::{
    error::MyProgramError,
    instruction::MultisigConfigIxData,
    state::try_from_account_info_mut,
};

#[repr(C)] //keeps the struct layout the same across different architectures
#[derive(Clone, Copy, Debug, PartialEq, shank::ShankAccount)]
pub struct MultisigConfig {
    pub config_authority: Pubkey,
    pub multisig_creation_fee: u64,
    pub treasury: Pubkey,
    pub mint: Pubkey,
    pub bump: u8,
}

impl DataLen for MultisigConfig {
    const LEN: usize = core::mem::size_of::<MultisigConfig>();
}

impl MultisigConfig {
    pub const SEED: &'static str = "multisig_config";

    pub fn validate_pda(bump: u8, pda: &Pubkey, owner: &Pubkey) -> Result<(), ProgramError> {
        let seeds = &[Self::SEED.as_bytes(), owner];
        let derived = pinocchio_pubkey::derive_address(seeds, Some(bump), &crate::ID);
        if derived != *pda {
            return Err(MyProgramError::PdaMismatch.into());
        }
        Ok(())
    }

    pub fn initialize( 
        config_acc: &AccountInfo,
        ix_data: &MultisigConfigIxData,
        mint_acc: &AccountInfo,
        bump: u8,
    ) -> ProgramResult {
        let multisig_config = unsafe { try_from_account_info_mut::<MultisigConfig>(config_acc) }?;

        multisig_config.config_authority = ix_data.config_authority;
        multisig_config.multisig_creation_fee = ix_data.multisig_creation_fee;
        multisig_config.treasury = ix_data.treasury;
        multisig_config.mint = *mint_acc.key();
        multisig_config.bump = bump;

        Ok(())
    }
}
