use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::Pubkey,
    ProgramResult,
};

// use pinocchio_system::instructions::CreateAccount;
use crate::instruction::create_account_checked::CreateAccountChecked;

use pinocchio_token::{ instructions::TransferChecked, state::{ Mint, TokenAccount } };

use crate::{
    error::MyProgramError,
    state::{
        utils::{load_ix_data, DataLen},
        EscrowState,
    },
};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, shank::ShankType)]
pub struct MakeEscrowIxData {
    pub maker: Pubkey,
    pub receive_amount: u64,
    pub bump: u8,
}

impl DataLen for MakeEscrowIxData {
    const LEN: usize = core::mem::size_of::<MakeEscrowIxData>(); // 32 bytes for Pubkey + 32 bytes for data
}

pub fn process_make_escrow(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [
        maker,
        mint_a,
        escrow_acc,
        vault,
        maker_ata_a,
        sysvar_rent_acc,
        _system_program,
        _rest @..
    ] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !maker.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // if !escrow_acc.data_is_empty() {
    //     return Err(ProgramError::AccountAlreadyInitialized);
    // }

    let vault_acc = TokenAccount::from_account_info(vault)?;
    assert_eq!(vault_acc.owner(), escrow_acc.key());

    let maker_ata_a_acc = TokenAccount::from_account_info(maker_ata_a)?;
    assert_eq!(maker_ata_a_acc.owner(), maker.key());

    let mint_a_acc = Mint::from_account_info(mint_a)?;

    // let rent = Rent::from_account_info(sysvar_rent_acc)?;

    let ix_data = unsafe { load_ix_data::<MakeEscrowIxData>(data)? };

    if ix_data.maker.ne(maker.key()) {
        return Err(MyProgramError::InvalidOwner.into());
    }

    let pda_bump_bytes = [ix_data.bump];

    EscrowState::validate_pda(ix_data.bump, escrow_acc.key(), &ix_data.maker)?;

    // Signer seeds
    let signer_seeds = [
        Seed::from(EscrowState::SEED.as_bytes()),
        Seed::from(&ix_data.maker),
        Seed::from(&pda_bump_bytes[..]),
    ];
    let signers = [Signer::from(&signer_seeds[..])];
    // Create the governance config account
    CreateAccountChecked {
        from: maker,
        to: escrow_acc,
        space: EscrowState::LEN as u64,
        owner: &crate::ID,
        sysvar_rent_acc,
        // lamports: rent.minimum_balance(MyState::LEN),
    }
    .invoke_signed(&signers)?;

    EscrowState::make(escrow_acc, ix_data)?;

    (TransferChecked {
        from: maker_ata_a,
        to: vault,
        authority: maker,
        mint: mint_a,
        amount: ix_data.receive_amount,
        decimals: mint_a_acc.decimals(),
    }).invoke()?;

    Ok(())
}
