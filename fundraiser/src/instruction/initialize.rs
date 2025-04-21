use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvars::rent::Rent,
    ProgramResult,
};

use pinocchio_system::instructions::CreateAccount;

use pinocchio_token::from_account_info;

use crate::{
    state::{
        utils::{load_ix_data, DataLen},
        Fundraiser,
    },
};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InitializeIxData {
    pub maker: Pubkey,
    pub duration: u8,
    pub bump: u8,
}

impl DataLen for InitializeIxData {
    const LEN: usize = core::mem::size_of::<InitializeIxData>();
}

pub fn process_initilaize(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [
        maker,
        mint_to_raise,
        fundraiser,
        vault,
        sysvar_rent_acc,
        _system_program,
        _token_program,
        _rest @..
    ] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !maker.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if !fundraiser.data_is_empty() {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    let vault_acc = TokenAccount::from_account_info(vault)?;

    assert_eq!(vault_acc.owner(), fundraiser.key());
    assert_eq!(vault_acc.mint, mint_to_raise);

    let rent = Rent::from_account_info(sysvar_rent_acc)?;

    let ix_data = unsafe { load_ix_data::<InitializeIxData>(data)? };

    let bump_seed = [ix_data.bump];

    // Signer seeds
    let fundraiser_seeds = [
        Seed::from(Fundraiser::SEED.as_bytes()),
        Seed::from(maker.key().as_ref()),
        Seed::from(&bump_seed[..]),
    ];
    let fundraiser_signer = Signer::from(&fundraiser_seeds[..]);

    CreateAccount {
        from: maker,
        to: fundraiser,
        space: Fundraiser::LEN as u64,
        owner: &crate::ID,
        lamports: rent.minimum_balance(Fundraiser::LEN),
    }
    .invoke_signed(&[fundraiser_signer])?;

    let mut fundraiser_state = (unsafe{
        load_account_unchecked::<Fundraiser>(fundraiser.borrow_mut_data_unchecked())
    })?;

    fundraiser_state.initialize(
        *maker.key,
        *mint_to_raise,
        ix_data.amount,
        ix_data.duration,
        ix_data.bump,
        Clock::get()?.unix_timestamp,
    )?;

    Ok(())
}
