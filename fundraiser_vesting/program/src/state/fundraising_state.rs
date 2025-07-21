use super::utils::StateDefinition;
use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};

use crate::{
    error::MyProgramError,
    instruction::{InitializeFundraisingStateIxData, DepositIxData},
    state::try_from_account_info_mut,
};


#[repr(C)] //keeps the struct layout the same across different architectures
#[derive(Clone, Copy, Debug, PartialEq, shank::ShankAccount)]
pub struct FundraisingState {
    pub initiator: Pubkey,
    pub mint: Pubkey,
    pub required_amount: u64,
    pub receive_amount: u64,
    pub end_time: u64,
    pub bump: u8,
}

#[repr(C)] //keeps the struct layout the same across different architectures
#[derive(Clone, Copy, Debug, PartialEq, shank::ShankAccount)]
pub struct DepositState {
    pub payer: Pubkey,
    pub amount: u64,
    pub bump: u8,
}

impl StateDefinition for FundraisingState {
    const LEN: usize = core::mem::size_of::<FundraisingState>();
    const SEED: &'static str = FundraisingState::SEED;
}

impl StateDefinition for DepositState {
    const LEN: usize = core::mem::size_of::<DepositState>();
    const SEED: &'static str = DepositState::SEED;
}

impl FundraisingState {
    pub const SEED: &'static str = "fundraising_state";

    pub fn validate_pda(bump: u8, pda: &Pubkey, owner: &Pubkey) -> Result<(), ProgramError> {
        let seeds = &[Self::SEED.as_bytes(), owner];
        let derived = pinocchio_pubkey::derive_address(seeds, Some(bump), &crate::ID);
        if derived != *pda {
            return Err(MyProgramError::PdaMismatch.into());
        }
        Ok(())
    }

    pub fn initialize(
        fundraising_state_acc: &AccountInfo,
        ix_data: &InitializeFundraisingStateIxData,
        mint: &AccountInfo,
        bump: u8,
    ) -> ProgramResult {
        let fundraising_state = unsafe { try_from_account_info_mut::<FundraisingState>(fundraising_state_acc) }?;

        fundraising_state.initiator = ix_data.initiator;
        fundraising_state.mint = *mint.key();
        fundraising_state.required_amount = ix_data.required_amount;
        fundraising_state.receive_amount = ix_data.initiation_amount;
        fundraising_state.end_time = ix_data.end_time;
        fundraising_state.bump = bump;

        Ok(())
    }

    pub fn deposit(&mut self, ix_data: &DepositIxData) -> ProgramResult {
        self.receive_amount += ix_data.amount;

        Ok(())
    }
}

impl DepositState {
    pub const SEED: &'static str = "deposit_state";

    pub fn validate_pda(bump: u8, pda: &Pubkey, owner: &Pubkey) -> Result<(), ProgramError> {
        let seeds = &[Self::SEED.as_bytes(), owner];
        let derived = pinocchio_pubkey::derive_address(seeds, Some(bump), &crate::ID);
        if derived != *pda {
            return Err(MyProgramError::PdaMismatch.into());
        }
        Ok(())
    }

    pub fn set_deposit_state(
        deposit_state_acc: &AccountInfo,
        payer: &AccountInfo,
        amount: u64,
        bump: u8,
    ) -> ProgramResult {
        let deposit_state = unsafe { try_from_account_info_mut::<DepositState>(deposit_state_acc) }?;   

        deposit_state.payer = *payer.key();
        deposit_state.amount = amount;
        deposit_state.bump = bump;

        Ok(())
    }
}