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

use crate::{
    error::MyProgramError,
    state::{
        utils::{load_ix_data, DataLen},
        MultisigConfig,
    },
};


#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, shank::ShankType)]
pub struct MultisigConfigIxData {
    pub config_authority: Pubkey,
    pub multisig_creation_fee: u64,
    pub treasury: Pubkey,
}

impl DataLen for MultisigConfigIxData {
    const LEN: usize = core::mem::size_of::<MultisigConfigIxData>();
}

pub fn process_initialize_config(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [
        payer_acc, 
        config_acc, 
        mint_acc,
        sysvar_rent_acc, 
        _system_program
    ] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !payer_acc.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if !config_acc.data_is_empty() {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    let rent = Rent::from_account_info(sysvar_rent_acc)?;

    let ix_data = unsafe { load_ix_data::<MultisigConfigIxData>(data)? };

    if ix_data.config_authority.ne(payer_acc.key()) {
        return Err(MyProgramError::InvalidOwner.into());
    }

    let seeds = &[MultisigConfig::SEED.as_bytes(), &ix_data.config_authority];
    // derive the canonical bump during account init
    let (derived_multisig_config_pda, bump) = pubkey::find_program_address(seeds, &crate::ID);
    if derived_multisig_config_pda.ne(config_acc.key()) {
        return Err(ProgramError::InvalidAccountOwner);
    }

    let bump_binding = [bump];
    //Signer Seeds
    let signer_seeds = [
        Seed::from(MultisigConfig::SEED.as_bytes()),
        Seed::from(&ix_data.config_authority),
        Seed::from(&bump_binding),
    ];
    let signers = [Signer::from(&signer_seeds[..])];

    CreateAccount {
        from: payer_acc,
        to: config_acc,
        space: MultisigConfig::LEN as u64,
        owner: &crate::ID,
        lamports: rent.minimum_balance(MultisigConfig::LEN),
    }
    .invoke_signed(&signers)?;

    MultisigConfig::initialize(config_acc, &ix_data, mint_acc, bump)?;

    Ok(())
}
