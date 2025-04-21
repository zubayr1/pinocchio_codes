use pinocchio_token::{instructions::TransferChecked, state::{Mint, TokenAccount}};

use crate::utils::{DataLen, load_ix_data, load_account_mut, load_account_mut_unchecked};
use crate::state::{Contributor, Fundraiser};
use crate::constants::{MAX_CONTRIBUTION_PERCENTAGE, PERCENTAGE_SCALER, SECONDS_TO_DAYS};

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct ContributeIxData {
    pub amount: u64,
    pub fundraiser_bump: u8,
    pub contributor_bump: u8,
}

impl DataLen for ContributeIxData {
    const LEN: usize = core::mem::size_of::<ContributeIxData>();
}




