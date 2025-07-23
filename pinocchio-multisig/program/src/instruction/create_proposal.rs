use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::{self, Pubkey},
    sysvars::rent::Rent,
    ProgramResult,
};

use pinocchio_system::instructions::CreateAccount;

use crate::{
    error::MyProgramError,
    state::{
        MultisigState, Proposal,
        utils::{load_ix_data, DataLen},
        try_from_account_info_mut,
    },
};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, shank::ShankType)]
pub struct CreateProposalIxData {
    pub initiator: Pubkey,
    pub multisig_id: u64,
    pub proposal_id: u64,
    pub transaction_index: u64,
}

impl DataLen for CreateProposalIxData {
    const LEN: usize = core::mem::size_of::<CreateProposalIxData>();
}

pub fn process_create_proposal(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [
        payer_acc, 
        multisig_acc,
        proposal_acc,
        sysvar_rent_acc,
        _system_program,
    ] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !payer_acc.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if multisig_acc.data_is_empty() {
        return Err(MyProgramError::InvalidMultisig.into());
    }

    let multisig_state = unsafe { try_from_account_info_mut::<MultisigState>(multisig_acc)? };

    if multisig_state.config_authority.ne(payer_acc.key()) {
        return Err(MyProgramError::InvalidOwner.into());
    }

    let ix_data = unsafe { load_ix_data::<CreateProposalIxData>(data)? };

    if ix_data.multisig_id != multisig_state.multisig_id {
        return Err(MyProgramError::InvalidMultisig.into());
    }

    let rent = Rent::from_account_info(sysvar_rent_acc)?;

    let seeds = &[Proposal::SEED.as_bytes(), &ix_data.initiator];
    // derive the canonical bump during account init
    let (derived_proposal_pda, bump) = pubkey::find_program_address(seeds, &crate::ID);
    if derived_proposal_pda.ne(proposal_acc.key()) {
        return Err(ProgramError::InvalidAccountOwner);
    }

    let bump_binding = [bump];
    //Signer Seeds
    let signer_seeds = [
        Seed::from(Proposal::SEED.as_bytes()),
        Seed::from(&ix_data.initiator),
        Seed::from(&bump_binding),
    ];
    let signers = [Signer::from(&signer_seeds[..])];

    CreateAccount {
        from: payer_acc,
        to: proposal_acc,
        space: Proposal::LEN as u64,
        owner: &crate::ID,
        lamports: rent.minimum_balance(Proposal::LEN),
    }
    .invoke_signed(&signers)?;
    
    let mut members = [Pubkey::default(); 10];
    let mut index = 0;
    let mut total_votes = 0;
    for member in multisig_state.members.iter() {
        if member.key.ne(&Pubkey::default()) 
        {
            total_votes += 1;
        }
        members[index] = member.key;
        index += 1;
    }

    Proposal::initialize(proposal_acc, ix_data.multisig_id, members, &ix_data, total_votes, bump)?;

    Ok(())
}
