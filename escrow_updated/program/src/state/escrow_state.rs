use super::utils::{load_acc_mut_unchecked, load_acc_unchecked, DataLen};
use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::{self, Pubkey},
    ProgramResult,
};

use crate::{
    error::MyProgramError,
    instruction::{MakeEscrowIxData, TakeEscrowIxData},
};

#[repr(C)] //keeps the struct layout the same across different architectures
#[derive(Clone, Copy, Debug, PartialEq, shank::ShankAccount)]
pub struct EscrowState {
    pub maker: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub receive_amount: u64,
    pub bump: u8,
}

impl DataLen for EscrowState {
    const LEN: usize = core::mem::size_of::<EscrowState>();
}

// impl Initialized for MyState {
//     fn is_initialized(&self) -> bool {
//         self.is_initialized > 0
//     }
// }

impl EscrowState {
    pub const SEED: &'static str = "escrow";

    pub fn from_account_info(escrow_acc: &AccountInfo) -> Result<Self, ProgramError> {
        let data = escrow_acc.try_borrow_data()?;
        let escrow_state: &Self = unsafe { load_acc_unchecked(&data)? };

        Ok(*escrow_state)
    }

    pub fn validate_pda(bump: u8, pda: &Pubkey, owner: &Pubkey) -> Result<(), ProgramError> {
        let seed_with_bump = &[Self::SEED.as_bytes(), owner, &[bump]];
        let (derived, _) = pubkey::find_program_address(seed_with_bump, &crate::ID);
        if derived != *pda {
            return Err(MyProgramError::PdaMismatch.into());
        }
        Ok(())
    }

    pub fn make(
        escrow_acc: &AccountInfo,
        ix_data: &MakeEscrowIxData,
    ) -> ProgramResult {
        let escrow_state = unsafe { load_acc_mut_unchecked::<EscrowState>(escrow_acc.borrow_mut_data_unchecked()) }?;

        escrow_state.maker = ix_data.maker;
        escrow_state.receive_amount = ix_data.receive_amount;
        escrow_state.bump = ix_data.bump;

        Ok(())
    }

    pub fn take(
        escrow_acc: &AccountInfo,
        _ix_data: &TakeEscrowIxData,
    ) -> ProgramResult {
        // let escrow_state = unsafe { load_acc_mut_unchecked::<EscrowState>(escrow_acc.borrow_mut_data_unchecked()) }?;

        let mut data = escrow_acc.try_borrow_mut_data()?;
        data.fill(0);

        Ok(())
    }
}
