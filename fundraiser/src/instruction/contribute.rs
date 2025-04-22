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

pub fn process_contribute(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [
        contributor,
        mint_to_raise,
        fundraiser,
        contributor_acc,
        contributor_ata,
        vault,
        sysvar_rent_acc,
        token_program,
        system_program,
        _rest @ ..
    ] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !contributor.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let vault_acc = TokenAccount::from_account_info(vault)?;

    assert_eq!(vault_acc.owner(), fundraiser.key());
    assert_eq!(vault_acc.mint, mint_to_raise);

    let contributor_ata_acc = TokenAccount::from_account_info(contributor_ata)?;
    assert_eq!(contributor_ata_acc.owner(), contributor.key());

    let ix_data = unsafe { load_ix_data::<ContributeIxData>(data)? };

    if contributor_acc.data_is_empty() || unsafe {contributor_acc.owner() != &crate::ID}{
        let rent = Rent::from_account_info(sysvar_rent_acc)?;

        let pda_bump_bytes = [ix_data.contributor_bump];

        let contributor_seeds = [
            Seed::from(Contributor::SEED.as_bytes()),
            Seed::from(fundraiser.key().as_ref()),
            Seed::from(contributor.key().as_ref()),
            Seed::from(&pda_bump_bytes[..]),
        ];

        let contributor_signer = Signer::from(&contributor_seeds[..]);

        (CreateAccount {
            from: contributor,
            to: contributor_acc,
            space: Contributor::LEN as u64,
            lamports: rent.minimum_balance(Contributor::LEN as u64),
            owner: &crate::ID,
        }).invoke_signed(&[contributor_signer])?;

        let mut contributor_state = (unsafe {
            load_account_mut_unchecked::<Contributor>(contributor_acc.borrow_mut_data_unchecked())
        })?;

        contributor_state.initialize(
            ix_data.amount,
        );

        let fundraiser_state = (unsafe {
            load_account_mut_unchecked::<Fundraiser>(fundraiser.borrow_mut_data_unchecked())
        })?;

        let mint_state = Mint::from_account_info(mint_to_raise)?;

        if ix_data.amount < (10_u32.pow(mint_state.decimals as u32) as u64) {
            return Err(ProgramError::InvalidAccountData);
        }

        if ix_data.amount > (fundraiser_state.amount_raised * MAX_CONTRIBUTION_PERCENTAGE / PERCENTAGE_SCALER) {
            return Err(ProgramError::InvalidAccountData);
        }

        let current_timestamp = Clock::get()?.unix_timestamp;

        if fundraiser_state.duration < ((current_timestamp - fundraiser_state.start_timestamp) / SECONDS_TO_DAYS) as u8 {
            return Err(ProgramError::InvalidAccountData);
        }

        let mint_authority = mint_state.mint_authority.ok_or(ProgramError::InvalidAccountData)?;

        (
            TransferChecked {
                from: contributor_ata,
                to: vault,
                mint: mint_to_raise,
                authority: contributor,
                amount: ix_data.amount,
                decimals: mint_state.decimals(),
            }
        ).invoke()?;

        contributor_state.amount += ix_data.amount;

        fundraiser_state.current_amount_raised += ix_data.amount;

        Ok(())
    }

}

