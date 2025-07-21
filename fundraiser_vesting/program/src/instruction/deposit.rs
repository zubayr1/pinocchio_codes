use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    sysvars::rent::Rent,
    ProgramResult,
};
use pinocchio::sysvars::Sysvar;
use pinocchio::sysvars::clock::Clock;
use pinocchio_token::{ instructions::TransferChecked, state::{ Mint, TokenAccount } };
use crate::{
    error::MyProgramError,
    state::{
        utils::load_ix_data,
        FundraisingState, DepositState,
    },
};
use crate::state::try_from_account_info_mut;
use crate::instruction::create_account::{create_account, Owner, DepositIxData};

pub fn process_deposit(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [
        depositor_acc,
        initiator_acc,
        vesting_acc,
        deposit_state_acc,
        vault,
        mint,
        depositor_ata,
        sysvar_rent_acc,
        _system_program,
        _rest @..
    ] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !depositor_acc.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let fundraising_state = unsafe { try_from_account_info_mut::<FundraisingState>(vesting_acc)? };

    // Validate PDA
    FundraisingState::validate_pda(fundraising_state.bump, vesting_acc.key(), &initiator_acc.key())?;

    if fundraising_state.initiator.ne(initiator_acc.key()) {
        return Err(MyProgramError::InvalidOwner.into());
    }

    if fundraising_state.end_time < Clock::get()?.unix_timestamp as u64 {
        return Err(MyProgramError::FundraisingEnded.into());
    }

    let depositor_ata_acc = TokenAccount::from_account_info(depositor_ata)?;
    assert_eq!(depositor_ata_acc.owner(), depositor_acc.key());

    let vault_acc = TokenAccount::from_account_info(vault)?;
    assert_eq!(vault_acc.owner(), deposit_state_acc.key());

    let mint_acc = Mint::from_account_info(mint)?;
    assert_eq!(*mint.key(), fundraising_state.mint);

    let ix_data = unsafe { load_ix_data::<DepositIxData>(data)? };

    if ix_data.payer.ne(depositor_acc.key()) {
        return Err(MyProgramError::InvalidOwner.into());
    }

    let seeds = &[DepositState::SEED.as_bytes(), ix_data.owner()];

    let rent = Rent::from_account_info(sysvar_rent_acc)?;

    let bump = create_account::<DepositState, _>(
        seeds,
        depositor_acc,
        deposit_state_acc,
        ix_data,
        &rent,
    )?;

    DepositState::set_deposit_state(
        deposit_state_acc,
        depositor_acc,
        ix_data.amount,
        bump,
    )?;

    (
        TransferChecked {
            from: depositor_ata,
            to: vault,
            authority: depositor_acc,
            mint: mint,
            amount: ix_data.amount,
            decimals: mint_acc.decimals(),
        }
    ).invoke()?;

    fundraising_state.deposit(ix_data)?;

    Ok(())
}
