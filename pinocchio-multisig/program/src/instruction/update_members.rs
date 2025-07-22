use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    ProgramResult,
    pubkey::Pubkey,
};

use crate::{
    error::MyProgramError,
    state::{
        Member,
        MultisigState, update_member,
        utils::{load_ix_data, DataLen},
        try_from_account_info_mut,
    },
};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, shank::ShankType)]
pub struct MultisigUpdateMembersIxData {
    pub multisig_id: u64,
    pub member_key: Pubkey,
    pub members: [Member; 10],
    pub update_type: u8,
    pub permission: u8,
    pub is_active: u8,
    pub index: u8,
}

impl DataLen for MultisigUpdateMembersIxData {
    const LEN: usize = core::mem::size_of::<MultisigUpdateMembersIxData>();
}

pub fn process_update_members(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
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

    if multisig_state.threshold > 10 {
        return Err(MyProgramError::InvalidThreshold.into());
    }

    let ix_data = unsafe { load_ix_data::<MultisigUpdateMembersIxData>(data)? };

    if ix_data.multisig_id != multisig_state.multisig_id {
        return Err(MyProgramError::InvalidMultisig.into());
    }

    update_member(ix_data.member_key, &mut multisig_state.members, ix_data.update_type, 
        ix_data.permission, ix_data.is_active, ix_data.members, ix_data.index, multisig_state.threshold, &mut multisig_state.current_index, 
        &mut multisig_state.roaming_index)

}
