use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::{self, Pubkey},
    sysvars::rent::Rent,
    ProgramResult,
};

use bytemuck::{Pod, Zeroable};

use pinocchio_system::instructions::CreateAccount;

use pinocchio_token::{ instructions::TransferChecked, state::{ Mint, TokenAccount } };

use crate::{
    error::MyProgramError,
    state::{
        utils::{load_ix_data, DataLen},
        MultisigState, MultisigConfig,
        try_from_account_info_mut,
    },
};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, shank::ShankType)]
pub struct MultisigCreateIxData {
    pub multisig_id: u64,
    pub config_authority: Pubkey,
    pub threshold: u8,
}

impl DataLen for MultisigCreateIxData {
    const LEN: usize = core::mem::size_of::<MultisigCreateIxData>();
}

pub fn process_create_multisig(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [
        payer_acc, 
        multisig_acc, 
        config_acc, 
        treasury_acc,
        mint,
        sysvar_rent_acc, 
        _system_program
    ] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !payer_acc.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if !multisig_acc.data_is_empty() {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    let payer_ata_acc = TokenAccount::from_account_info(payer_acc)?;
    assert_eq!(payer_ata_acc.owner(), payer_acc.key());

    let rent = Rent::from_account_info(sysvar_rent_acc)?;

    let ix_data = unsafe { load_ix_data::<MultisigCreateIxData>(data)? };

    if ix_data.config_authority.ne(payer_acc.key()) {
        return Err(MyProgramError::InvalidOwner.into());
    }

    let multisig_config = unsafe { try_from_account_info_mut::<MultisigConfig>(config_acc)? };

    if treasury_acc.key().ne(&multisig_config.treasury) {
        return Err(MyProgramError::InvalidTreasury.into());
    }

    if multisig_config.mint.ne(mint.key()) {
        return Err(MyProgramError::InvalidMint.into());
    }

    let mint_acc = Mint::from_account_info(mint)?;

    let seeds = &[MultisigState::SEED.as_bytes(), &ix_data.config_authority];
    // derive the canonical bump during account init
    let (derived_multisig_state_pda, bump) = pubkey::find_program_address(seeds, &crate::ID);
    if derived_multisig_state_pda.ne(multisig_acc.key()) {
        return Err(ProgramError::InvalidAccountOwner);
    }

    let bump_binding = [bump];
    //Signer Seeds
    let signer_seeds = [
        Seed::from(MultisigState::SEED.as_bytes()),
        Seed::from(&ix_data.config_authority),
        Seed::from(&bump_binding),
    ];
    let signers = [Signer::from(&signer_seeds[..])];

    (
        TransferChecked {
            from: payer_acc,
            to: treasury_acc,
            authority: payer_acc,
            mint: mint,
            amount: multisig_config.multisig_creation_fee,
            decimals: mint_acc.decimals(),
        }
    ).invoke()?;

    CreateAccount {
        from: payer_acc,
        to: multisig_acc,
        space: MultisigState::LEN as u64,
        owner: &crate::ID,
        lamports: rent.minimum_balance(MultisigState::LEN),
    }
    .invoke_signed(&signers)?;

    MultisigState::initialize(multisig_acc, &ix_data, bump)?;

    Ok(())
}
