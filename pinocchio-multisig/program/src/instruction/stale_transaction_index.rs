use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    ProgramResult,
    pubkey::Pubkey,
};

use crate::{
    error::MyProgramError,
    state::{
        MultisigState,
        utils::{load_ix_data, DataLen},
        try_from_account_info_mut,
    },
};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, shank::ShankType)]
pub struct MultisigStaleTransactionIndexIxData {
    pub multisig_id: u64,
    pub stale_transaction_index: i64,
    pub initiator: Pubkey,
}

impl DataLen for MultisigStaleTransactionIndexIxData {
    const LEN: usize = core::mem::size_of::<MultisigStaleTransactionIndexIxData>();
}

pub fn process_stale_transaction_index(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [
        payer_acc, 
        multisig_acc,
        _system_program
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

    let ix_data = unsafe { load_ix_data::<MultisigStaleTransactionIndexIxData>(data)? };

    if ix_data.multisig_id != multisig_state.multisig_id {
        return Err(MyProgramError::InvalidMultisig.into());
    }

    MultisigState::update_stale_transaction_index(multisig_acc, ix_data.stale_transaction_index)?;

    Ok(())
}
