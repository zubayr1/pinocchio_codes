use super::utils::{load_acc_mut_unchecked, DataLen, Initialized};
use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::{self, Pubkey},
    ProgramResult,
};

use crate::{
    error::MyProgramError,
    instruction::{InitializeMyStateIxData, UpdateMyStateIxData},
};

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum State {
    Uninitialized,
    Initialized,
    Updated,
}

#[repr(C)] //keeps the struct layout the same across different architectures
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MyState {
    pub is_initialized: bool,
    pub owner: Pubkey,
    pub state: State,
    pub data: [u8; 32],
    pub update_count: u32,
}

impl DataLen for MyState {
    const LEN: usize = core::mem::size_of::<MyState>();
}

impl Initialized for MyState {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl MyState {
    pub const SEED: &'static str = "mystate";

    pub fn validate_pda(bump: u8, pda: &Pubkey, owner: &Pubkey) -> Result<(), ProgramError> {
        let seed_with_bump = &[Self::SEED.as_bytes(), owner, &[bump]];
        let derived = pubkey::create_program_address(seed_with_bump, &crate::ID)?;
        if derived != *pda {
            return Err(MyProgramError::PdaMismatch.into());
        }
        Ok(())
    }

    pub fn initialize(
        my_stata_acc: &AccountInfo,
        ix_data: &InitializeMyStateIxData,
    ) -> ProgramResult {
        let my_state =
            unsafe { load_acc_mut_unchecked::<MyState>(my_stata_acc.borrow_mut_data_unchecked()) }?;

        my_state.owner = ix_data.owner;
        my_state.state = State::Initialized;
        my_state.data = ix_data.data;
        my_state.update_count = 0;
        my_state.is_initialized = true;

        Ok(())
    }

    pub fn update(&mut self, ix_data: &UpdateMyStateIxData) -> ProgramResult {
        self.data = ix_data.data;
        if self.state != State::Updated {
            self.state = State::Updated;
        }
        self.update_count += 1;

        Ok(())
    }
}
