use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvars::rent::Rent,
    ProgramResult,
};
use pinocchio_token::instructions::CloseAccount;

use pinocchio_token::{ instructions::TransferChecked, state::{ Mint, TokenAccount } };

use crate::{
    error::MyProgramError,
    state::{
        utils::load_acc_mut_unchecked,
        utils::{load_ix_data, DataLen},
        EscrowState,
    },
};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TakeEscrowIxData {
    pub taker: Pubkey,
    pub bump: u8,
}

impl DataLen for TakeEscrowIxData {
    const LEN: usize = core::mem::size_of::<TakeEscrowIxData>(); // 32 bytes for data
}

pub fn process_take_escrow(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [
        taker, 
        maker, 
        escrow_acc, 
        vault, 
        mint_a, 
        mint_b, 
        taker_ata_a, 
        taker_ata_b, 
        maker_ata_b, 
        _system_program, 
        _token_program, 
        _rest @..
    ] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !taker.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let escrow_state = EscrowState::from_account_info(escrow_acc)?;

    let vault_acc = TokenAccount::from_account_info(vault)?;
    assert_eq!(vault_acc.owner(), escrow_acc.key());

    let taker_ata_a_acc = TokenAccount::from_account_info(taker_ata_a)?;
    assert_eq!(taker_ata_a_acc.owner(), taker.key());

    let taker_ata_b_acc = TokenAccount::from_account_info(taker_ata_b)?;
    assert_eq!(taker_ata_b_acc.owner(), taker.key());

    let maker_ata_b_acc = TokenAccount::from_account_info(maker_ata_b)?;
    assert_eq!(maker_ata_b_acc.owner(), maker.key());

    let mint_a_acc = Mint::from_account_info(mint_a)?;
    let mint_b_acc = Mint::from_account_info(mint_b)?;

    let ix_data = unsafe { load_ix_data::<TakeEscrowIxData>(data)? };

    if ix_data.taker.ne(taker.key()) {
        return Err(MyProgramError::InvalidOwner.into());
    }

    // Validate PDA
    EscrowState::validate_pda(ix_data.bump, escrow_acc.key(), &maker.key())?;

    TransferChecked {
        from: taker_ata_b,
        to: maker_ata_b,
        authority: taker,
        mint: mint_b,
        amount: escrow_state.receive_amount,
        decimals: mint_b_acc.decimals(),
    }.invoke()?;

    let pda_bump_bytes = [ix_data.bump];

    let signer_seeds = [
        Seed::from(EscrowState::SEED.as_bytes()),
        Seed::from(maker.key()),
        Seed::from(&pda_bump_bytes[..]),
    ];
    let signers = [Signer::from(&signer_seeds[..])];

    TransferChecked {
        from: vault,
        to: taker_ata_a,
        authority: escrow_acc,
        mint: mint_a,
        amount: escrow_state.receive_amount,
        decimals: mint_a_acc.decimals(),
    }.invoke_signed(&signers)?;

    CloseAccount {
        account: escrow_acc,
        authority: escrow_acc,
        destination: taker,
    }.invoke_signed(&signers)?;

    EscrowState::take(escrow_acc, ix_data)?;

    Ok(())
}
