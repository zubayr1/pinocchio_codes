use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::Pubkey,
    ProgramResult,
};

use crate::{
    error::MyProgramError,
    state::{
        MultisigState, Proposal, Transaction, ProposalStatus,
        utils::{load_ix_data, DataLen},
        try_from_account_info_mut,
    },
};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, shank::ShankType)]
pub struct ApproveTransactionIxData {
    pub approver: Pubkey,
    pub multisig_id: u64,
    pub proposal_id: u64,
    pub transaction_index: u64,
    pub approver_index: usize,
}

impl DataLen for ApproveTransactionIxData {
    const LEN: usize = core::mem::size_of::<ApproveTransactionIxData>();
}

pub fn process_approve_transaction(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [
        approver_acc, 
        multisig_acc,
        proposal_acc,
        transaction_acc,
    ] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !approver_acc.is_signer() {
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

    let ix_data = unsafe { load_ix_data::<ApproveTransactionIxData>(data)? };

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

    if ix_data.approver_index > multisig_state.threshold as usize {
        return Err(MyProgramError::InvalidPayerIndex.into());
    }

    if multisig_state.members[ix_data.approver_index].key.ne(approver_acc.key()) {
        return Err(MyProgramError::InvalidPayer.into());
    }

    if proposal_state.status != ProposalStatus::Approved {
        Transaction::reject(transaction_acc)?;
        return Err(MyProgramError::InvalidProposalStatus.into());
    }

    Transaction::approve(transaction_acc)?;

    Ok(())
} 