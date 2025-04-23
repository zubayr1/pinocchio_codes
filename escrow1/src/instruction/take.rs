
use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvars::rent::Rent,
    ProgramResult,
};

use pinocchio_system::instructions::CreateAccount;

use pinocchio_token::{ instructions::TransferChecked, state::{ Mint, TokenAccount } };

use crate::{
    error::MyProgramError,
    state::{
        utils::{load_ix_data, DataLen},
        Escrow,
    },
};

pub fn process_take_escrow(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [
        maker,
        taker,
        mint_a,
        mint_b,
        escrow,
        vault,
        maker_ata_x,
        maker_ata_y,
        taker_ata_x,
        taker_ata_y,
        _sysvar_rent_acc,
        _system_program,
        _rest @..
        ] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !maker.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if !escrow.data_is_empty() {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    let vault_acc = TokenAccount::from_account_info(vault)?;
    assert_eq!(vault_acc.owner(), escrow.key());

    let maker_ata_acc = TokenAccount::from_account_info(maker_ata_x)?;
    assert_eq!(maker_ata_acc.owner(), maker.key());

    let maker_ata_acc = TokenAccount::from_account_info(maker_ata_y)?;
    assert_eq!(maker_ata_acc.owner(), maker.key());

    let taker_ata_acc = TokenAccount::from_account_info(taker_ata_x)?;
    assert_eq!(taker_ata_acc.owner(), taker.key());

    let taker_ata_acc = TokenAccount::from_account_info(taker_ata_y)?;
    assert_eq!(taker_ata_acc.owner(), taker.key());

    let rent = Rent::get()?;

    let ix_data = unsafe { load_ix_data::<MakeEscrowIxData>(data)? };

    if ix_data.maker.ne(maker.key()) {
        return Err(MyProgramError::InvalidOwner.into());
    }

    let pda_bump_bytes = [ix_data.bump];

    Escrow::validate_pda(ix_data.bump, escrow.key(), &ix_data.maker)?;

    // Signer seeds
    let signer_seeds = [
        Seed::from(Escrow::SEED.as_bytes()),
        Seed::from(&ix_data.maker),
        Seed::from(&pda_bump_bytes[..]),
    ];
    let signers = [Signer::from(&signer_seeds[..])];
    // Create the governance config account
    (CreateAccount {
        from: maker,
        to: escrow,
        space: Escrow::LEN as u64,
        owner: &crate::ID,
        lamports: rent.minimum_balance(Escrow::LEN),
    })
    .invoke_signed(&signers)?;

    // Initialize the escrow account
    Escrow::make(escrow, ix_data)?;

    (TransferChecked {
        from: maker_ata,
        to: vault,
        authority: maker,
        mint: mint_a,
        amount: ix_data.receive_amount,
        decimals: mint_a.decimals,
    }).invoke()?;

    Ok(())
}
