use super::utils::{load_acc_mut_unchecked, DataLen, Initialized};
use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::{self, Pubkey},
    ProgramResult,
};

#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Fundraiser {
    is_initialized: bool,
    maker: Pubkey,
    mint_to_raise: Pubkey,
    amount_to_raise: u8,
    current_amount: u8,
    time_started: i64,
    duration: u8,
    bump: u8,
}

impl DataLen for Fundraiser {
    const LEN: usize = core::mem::size_of::<Fundraiser>();
}

impl Initialized for Fundraiser {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Fundraiser {
    pub const SEED: &'static str = "fundraiser";

    pub fn validate_pda(bump: u8, pda: &Pubkey, owner: &Pubkey) -> Result<(), ProgramError> {
        let seed_with_bump = &[Self::SEED.as_bytes(), owner, &[bump]];
        let derived = pubkey::create_program_address(seed_with_bump, &crate::ID)?;
        if derived != *pda {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(())
    }

    pub fn initialize(
        maker: Pubkey,
        mint_to_raise: Pubkey,
        amount_to_raise: u8,
        time_started: i64,
        duration: u8,
        bump: u8,
    ) -> ProgramResult {
        let fundraiser =
            unsafe { load_acc_mut_unchecked::<Fundraiser>(self.borrow_mut_data_unchecked()) }?;

        fundraiser.is_initialized = true;
        fundraiser.maker = maker;
        fundraiser.mint_to_raise = mint_to_raise;
        fundraiser.amount_to_raise = amount_to_raise;
        fundraiser.current_amount = 0;
        fundraiser.time_started = time_started;
        fundraiser.duration = duration;
        fundraiser.bump = bump;

        Ok(())
    }
}
