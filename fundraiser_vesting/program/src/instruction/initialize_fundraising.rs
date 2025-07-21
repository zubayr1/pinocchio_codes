use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    sysvars::rent::Rent,
    ProgramResult,
};

use pinocchio_token::{ instructions::TransferChecked, state::{ Mint, TokenAccount } };
use crate::{
    error::MyProgramError,
    state::{
        utils::load_ix_data,
        FundraisingState,
    },
};
use crate::instruction::create_account::{create_account, Owner, InitializeFundraisingStateIxData};


pub fn process_initilaize_fundraising(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [
        initiator_acc,
        vesting_acc,
        vault,
        mint,
        initiator_ata,
        sysvar_rent_acc,
        _system_program,
        _rest @..
    ] = accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !initiator_acc.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if !vesting_acc.data_is_empty() {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    let vault_acc = TokenAccount::from_account_info(vault)?;
    assert_eq!(vault_acc.owner(), vesting_acc.key());

    let initiator_ata_acc = TokenAccount::from_account_info(initiator_ata)?;
    assert_eq!(initiator_ata_acc.owner(), initiator_acc.key());

    let mint_acc = Mint::from_account_info(mint)?;    

    let ix_data = unsafe { load_ix_data::<InitializeFundraisingStateIxData>(data)? };

    if ix_data.initiator.ne(initiator_acc.key()) {
        return Err(MyProgramError::InvalidOwner.into());
    }

    let seeds = &[FundraisingState::SEED.as_bytes(), ix_data.owner()];

    let rent = Rent::from_account_info(sysvar_rent_acc)?;

    let bump = create_account::<FundraisingState, _>(
        seeds,
        initiator_acc,
        vesting_acc,
        ix_data,
        &rent,
    )?;

    FundraisingState::initialize(
        vesting_acc,
        ix_data,
        mint,
        bump,
    )?;
    
    (
        TransferChecked {
            from: initiator_ata,
            to: vault,
            authority: initiator_acc,
            mint: mint,
            amount: ix_data.initiation_amount,
            decimals: mint_acc.decimals(),
        }
    ).invoke()?;

    Ok(())
}
