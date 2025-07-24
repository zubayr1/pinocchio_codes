use super::utils::DataLen;
use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};

use crate::{
    error::MyProgramError,
    instruction::CreateProposalIxData,
    state::try_from_account_info_mut,
};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ProposalStatus {
    Active,
    Paused,
    Approved,
    Rejected,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Vote {
    DidntVote,
    Yes,
    No,
    Abstain,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, shank::ShankAccount)]
pub struct Proposal {
    pub multisig_id: u64,
    pub proposal_id: u64,
    pub transaction_index: u64,
    pub status: ProposalStatus,
    pub members: [Pubkey; 10],
    pub votes: [u8; 15], // 10: didnt vote, 11: yes, 12: no, 13: abstain 14: total votes
    pub bump: u8,
}

impl DataLen for Proposal {
    const LEN: usize = core::mem::size_of::<Proposal>();
}

impl Proposal {
    pub const SEED: &'static str = "proposal";

    pub fn validate_pda(bump: u8, pda: &Pubkey, owner: &Pubkey) -> Result<(), ProgramError> {
        let seeds = &[Self::SEED.as_bytes(), owner];
        let derived = pinocchio_pubkey::derive_address(seeds, Some(bump), &crate::ID);
        if derived != *pda {
            return Err(MyProgramError::PdaMismatch.into());
        }
        Ok(())
    }

    pub fn initialize(
        proposal_acc: &AccountInfo,
        multisig_id: u64,
        members: [Pubkey; 10],
        ix_data: &CreateProposalIxData,
        total_votes: u8,
        bump: u8,
    ) -> ProgramResult {
        let proposal_state = unsafe { try_from_account_info_mut::<Proposal>(proposal_acc) }?;

        proposal_state.multisig_id = multisig_id;
        proposal_state.proposal_id = ix_data.proposal_id;
        proposal_state.transaction_index = ix_data.transaction_index;
        proposal_state.status = ProposalStatus::Active;
        proposal_state.members = members;
        proposal_state.votes = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, total_votes];
        proposal_state.bump = bump;

        Ok(())
    }

    pub fn update_vote(proposal_acc: &AccountInfo, payer_index: usize, payer: &Pubkey, vote: u8) -> ProgramResult {
        let proposal_state = unsafe { try_from_account_info_mut::<Proposal>(proposal_acc) }?;

        if proposal_state.members[payer_index] != *payer {
            return Err(MyProgramError::InvalidPayer.into());
        }

        if vote < 1 || vote > 3 {
            return Err(MyProgramError::InvalidVote.into());
        }

        match proposal_state.votes[payer_index] {
            0 => {
                proposal_state.votes[10 + vote as usize] += 1;
            }
            1 => {
                if vote != 1 {
                    proposal_state.votes[10 + vote as usize] += 1;
                    proposal_state.votes[11] -= 1;
                }
            }
            2 => {
                if vote != 2 {
                    proposal_state.votes[10 + vote as usize] += 1;
                    proposal_state.votes[12] -= 1;
                }
            }
            3 => {
                if vote != 3 {
                    proposal_state.votes[10 + vote as usize] += 1;
                    proposal_state.votes[13] -= 1;
                }
            }
            _ => {
                return Err(MyProgramError::InvalidVote.into());
            }
        }

        proposal_state.votes[payer_index] = vote;

        proposal_state.votes[10] -= 1;

        Ok(())
    }
}
