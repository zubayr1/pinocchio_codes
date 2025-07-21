use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::{self, Pubkey},
    sysvars::rent::Rent,
};

use pinocchio_system::instructions::CreateAccount;
use crate::state::utils::StateDefinition;
use crate::state::{FundraisingState, DepositState};

pub trait Owner {
    fn owner(&self) -> &Pubkey;
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, shank::ShankType)]
pub struct InitializeFundraisingStateIxData {
    pub initiator: Pubkey,
    pub initiation_amount: u64,
    pub required_amount: u64,
    pub end_time: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, shank::ShankType)]
pub struct DepositIxData {
    pub payer: Pubkey,
    pub amount: u64,
}

impl StateDefinition for InitializeFundraisingStateIxData {
    const LEN: usize = core::mem::size_of::<InitializeFundraisingStateIxData>(); // 32 bytes for Pubkey + 32 bytes for data
    const SEED: &'static str = FundraisingState::SEED;
}

impl StateDefinition for DepositIxData {
    const LEN: usize = core::mem::size_of::<DepositIxData>(); // 32 bytes for data
    const SEED: &'static str = DepositState::SEED;
}

impl Owner for InitializeFundraisingStateIxData {
    fn owner(&self) -> &Pubkey {
        &self.initiator
    }
}

impl Owner for DepositIxData {
    fn owner(&self) -> &Pubkey {
        &self.payer
    }
}

pub fn create_account<S, I>(seeds: &[&[u8]], payer_acc: &AccountInfo, to_acc: &AccountInfo, ix_data: &I, rent: &Rent)
 -> Result<u8, ProgramError> 
 where S: StateDefinition, I: Owner {
    let (derived_pda, bump) = pubkey::find_program_address(seeds, &crate::ID);
    if derived_pda.ne(to_acc.key()) {
        return Err(ProgramError::InvalidAccountOwner);
    }

    let bump_binding = [bump];
    // Signer seeds
    let signer_seeds = [
        Seed::from(S::SEED.as_bytes()),
        Seed::from(ix_data.owner()),
        Seed::from(&bump_binding),
    ];
    let signers = [Signer::from(&signer_seeds[..])];

    CreateAccount {
        from: payer_acc,
        to: to_acc,
        space: S::LEN as u64,
        owner: &crate::ID,
        lamports: rent.minimum_balance(S::LEN),
    }
    .invoke_signed(&signers)?;

    Ok(bump)
}