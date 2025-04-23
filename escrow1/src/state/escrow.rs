use super::utils::{load_acc_mut_unchecked, DataLen, Initialized};
use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::{self, Pubkey},
    ProgramResult,
};

use crate::{
    error::MyProgramError,
    instruction::MakeEscrowIxData,
};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Escrow {
    pub maker: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub receive_amount: u64,
    pub bump: u8,
    pub is_initialized: bool,
}

impl DataLen for Escrow {
    const LEN: usize = core::mem::size_of::<Escrow>();
}

impl Initialized for Escrow {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Escrow {
    pub const SEED: &'static str = "escrow";

    pub fn validate_pda(bump: u8, pda: &Pubkey, owner: &Pubkey) -> Result<(), ProgramError> {
        let seed_with_bump = &[Self::SEED.as_bytes(), owner, &[bump]];
        let derived = pubkey::create_program_address(seed_with_bump, &crate::ID)?;
        if derived != *pda {
            return Err(MyProgramError::PdaMismatch.into());
        }
        Ok(())
    }

    pub fn make(
        escrow_acc: &AccountInfo,
        ix_data: &MakeEscrowIxData,
    ) -> ProgramResult {
        let escrow =
            unsafe { load_acc_mut_unchecked::<Escrow>(escrow_acc.borrow_mut_data_unchecked()) }?;

        escrow.maker = ix_data.maker;
        escrow.mint_a = ix_data.mint_a;
        escrow.mint_b = ix_data.mint_b;
        escrow.receive_amount = ix_data.receive_amount;
        escrow.bump = ix_data.bump;
        escrow.is_initialized = true;

        Ok(())
    }

    pub fn take(
        escrow_acc: &AccountInfo,
        ix_data: &TakeEscrowIxData,
    ) -> ProgramResult {
        let escrow =
            unsafe { load_acc_mut_unchecked::<Escrow>(escrow_acc.borrow_mut_data_unchecked()) }?;


    }
}
