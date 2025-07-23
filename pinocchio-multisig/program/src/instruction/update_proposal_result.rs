use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::Pubkey,
    ProgramResult,
};

use crate::{
    error::MyProgramError,
    state::{
        MultisigState, Proposal, Transaction, ProposalStatus, TransactionStatus, Permission,
        utils::{load_ix_data, DataLen},
        try_from_account_info_mut,
    },
};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, shank::ShankType)]
pub struct UpdateProposalResultIxData {
    pub updater: Pubkey,
    pub multisig_id: u64,
    pub proposal_id: u64,
    pub transaction_index: u64,
    pub updater_index: usize,
}

impl DataLen for UpdateProposalResultIxData {
    const LEN: usize = core::mem::size_of::<UpdateProposalResultIxData>();
}

pub fn process_update_proposal_result(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [
        updater_acc, 
        multisig_acc,
        proposal_acc,
        transaction_acc,
    ] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !updater_acc.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if multisig_acc.data_is_empty() {
        return Err(MyProgramError::InvalidMultisig.into());
    }

    if proposal_acc.data_is_empty() {
        return Err(MyProgramError::InvalidProposal.into());
    }

    if transaction_acc.data_is_empty() {
        return Err(MyProgramError::InvalidTransaction.into());
    }

    let multisig_state = unsafe { try_from_account_info_mut::<MultisigState>(multisig_acc)? };
    let proposal_state = unsafe { try_from_account_info_mut::<Proposal>(proposal_acc)? };
    let transaction_state = unsafe { try_from_account_info_mut::<Transaction>(transaction_acc)? };

    let ix_data = unsafe { load_ix_data::<UpdateProposalResultIxData>(data)? };

    if ix_data.multisig_id != multisig_state.multisig_id {
        return Err(MyProgramError::InvalidMultisig.into());
    }

    if proposal_state.multisig_id != multisig_state.multisig_id {
        return Err(MyProgramError::InvalidProposal.into());
    }

    if transaction_state.multisig_id != multisig_state.multisig_id {
        return Err(MyProgramError::InvalidTransaction.into());
    }

    if proposal_state.transaction_index != ix_data.transaction_index {
        return Err(MyProgramError::InvalidTransactionIndex.into());
    }

    if ix_data.updater_index > multisig_state.threshold as usize {
        return Err(MyProgramError::InvalidPayerIndex.into());
    }

    if multisig_state.members[ix_data.updater_index].key.ne(updater_acc.key()) {
        return Err(MyProgramError::InvalidPayerIndex.into());
    }

    if multisig_state.members[ix_data.updater_index].permissions == Permission::Readonly {
        return Err(MyProgramError::Unauthorized.into());
    }

    if proposal_state.status != ProposalStatus::Active {
        return Err(MyProgramError::InvalidProposalStatus.into());
    }

    if transaction_state.status != TransactionStatus::Pending {
        return Err(MyProgramError::InvalidTransactionStatus.into());
    }

    let yes_votes = proposal_state.votes[11]; // Index 11 stores yes votes
    let no_votes = proposal_state.votes[12];  // Index 12 stores no votes
    let total_votes = proposal_state.votes[14]; // Index 14 stores total votes
    
    let majority_threshold = (total_votes / 2) + 1;
    
    if yes_votes >= majority_threshold {
        proposal_state.status = ProposalStatus::Approved;
    } else if no_votes >= majority_threshold {
        proposal_state.status = ProposalStatus::Rejected;
    } else {
        proposal_state.status = ProposalStatus::Paused;
    }

    Ok(())
} 