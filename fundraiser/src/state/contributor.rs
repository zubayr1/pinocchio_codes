use super::utils::{load_acc_mut_unchecked, DataLen, Initialized};
use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::{self, Pubkey},
    ProgramResult,
};

#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Contributor {
    is_initialized: bool,
    amount: u64,
    bump: u8,
}

impl DataLen for Contributor {
    const LEN: usize = core::mem::size_of::<Contributor>();
}

impl Initialized for Contributor {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Contributor {
    pub const SEED: &'static str = "contributor";

    pub fn initialize(
        amount: u64,
        bump: u8,
    ) -> ProgramResult {
        let contributor =
            unsafe { load_acc_mut_unchecked::<Contributor>(self.borrow_mut_data_unchecked()) }?;

        contributor.is_initialized = true;
        contributor.amount = amount;
        contributor.bump = bump;

        Ok(())
    }
}
