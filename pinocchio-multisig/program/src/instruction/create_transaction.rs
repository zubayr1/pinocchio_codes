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
        MultisigState, Transaction, CompiledInstruction, Permission,
        utils::{load_ix_data, DataLen},
        try_from_account_info_mut,
    },
};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, shank::ShankType)]
pub struct CreateTransactionIxData {
    pub creator: Pubkey,
    pub multisig_id: u64,
    pub member_index: usize,
    pub transaction_index: u64,
    pub account_keys: [Pubkey; 10],
    pub instructions: [CompiledInstruction; 20],
    pub num_account_keys: u8,
    pub num_instructions: u8,
}

impl DataLen for CreateTransactionIxData {
    const LEN: usize = core::mem::size_of::<CreateTransactionIxData>();
}

pub fn process_create_transaction(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [
        payer_acc, 
        multisig_acc,
        transaction_acc,
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

    let ix_data = unsafe { load_ix_data::<CreateTransactionIxData>(data)? };

    if ix_data.member_index > multisig_state.threshold as usize {
        return Err(MyProgramError::InvalidPayerIndex.into());
    }

    if multisig_state.members[ix_data.member_index].key.ne(payer_acc.key()) {
        return Err(MyProgramError::InvalidPayerIndex.into());
    }

    if multisig_state.members[ix_data.member_index].permissions == Permission::Readonly {
        return Err(MyProgramError::InvalidPayerIndex.into());
    }

    if ix_data.multisig_id != multisig_state.multisig_id {
        return Err(MyProgramError::InvalidMultisig.into());
    }

    if ix_data.transaction_index != multisig_state.transaction_index + 1 {
        return Err(MyProgramError::InvalidTransactionIndex.into());
    }

    let rent = Rent::from_account_info(sysvar_rent_acc)?;

    let seeds = &[
        Transaction::SEED.as_bytes(), 
        &ix_data.multisig_id.to_le_bytes(),
        &ix_data.transaction_index.to_le_bytes()
    ];
    let (derived_transaction_pda, bump) = pubkey::find_program_address(seeds, &crate::ID);
    if derived_transaction_pda.ne(transaction_acc.key()) {
        return Err(ProgramError::InvalidAccountOwner);
    }

    let bump_binding = [bump];
    let binding = ix_data.multisig_id.to_le_bytes();
    let binding2 = ix_data.transaction_index.to_le_bytes();
    let signer_seeds = [
        Seed::from(Transaction::SEED.as_bytes()),
        Seed::from(&binding),
        Seed::from(&binding2),
        Seed::from(&bump_binding),
    ];
    let signers = [Signer::from(&signer_seeds[..])];

    CreateAccount {
        from: payer_acc,
        to: transaction_acc,
        space: Transaction::LEN as u64,
        owner: &crate::ID,
        lamports: rent.minimum_balance(Transaction::LEN),
    }
    .invoke_signed(&signers)?;
    
    Transaction::initialize(
        transaction_acc,
        ix_data.multisig_id,
        ix_data.transaction_index,
        ix_data.creator,
        ix_data.account_keys,
        ix_data.instructions,
        ix_data.num_account_keys,
        ix_data.num_instructions,
        bump,
    )?;

    multisig_state.transaction_index = ix_data.transaction_index;

    Ok(())
} 