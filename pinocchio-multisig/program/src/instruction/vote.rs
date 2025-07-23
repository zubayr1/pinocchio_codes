use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    ProgramResult,
};

use crate::{
    error::MyProgramError,
    state::{
        MultisigState, Proposal, Permission,
        utils::{load_ix_data, DataLen},
        try_from_account_info_mut,
    },
};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, shank::ShankType)]
pub struct VoteIxData {
    pub multisig_id: u64,
    pub payer_index: usize,
    pub vote: u8,
}

impl DataLen for VoteIxData {
    const LEN: usize = core::mem::size_of::<VoteIxData>();
}

pub fn process_vote(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [
        payer_acc, 
        multisig_acc,
        proposal_acc,
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

    let ix_data = unsafe { load_ix_data::<VoteIxData>(data)? };

    if ix_data.payer_index > multisig_state.threshold as usize {
        return Err(MyProgramError::InvalidPayerIndex.into());
    }

    if ix_data.multisig_id != multisig_state.multisig_id {
        return Err(MyProgramError::InvalidMultisig.into());
    }

    if multisig_state.members[ix_data.payer_index].permissions == Permission::Readonly {
        return Err(MyProgramError::InvalidPayer.into());
    }

    Proposal::update_vote(proposal_acc, ix_data.payer_index, payer_acc.key(), ix_data.vote)?;

    Ok(())
}
