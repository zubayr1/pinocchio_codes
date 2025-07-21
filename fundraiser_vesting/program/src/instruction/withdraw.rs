use pinocchio::{account_info::AccountInfo, instruction::{Seed, Signer}, program_error::ProgramError, ProgramResult};

use crate::{
    error::MyProgramError,
    state::{
        try_from_account_info_mut,
        FundraisingState, DepositState,
    },
};
use pinocchio_token::{ instructions::TransferChecked, state::Mint };

pub fn process_withdraw(accounts: &[AccountInfo], _: &[u8]) -> ProgramResult {
    let [payer_acc, vesting_acc, deposit_state_acc,mint, vault] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !payer_acc.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let fundraising_state = unsafe { try_from_account_info_mut::<FundraisingState>(vesting_acc)? };
    let deposit_state = unsafe { try_from_account_info_mut::<DepositState>(deposit_state_acc)? };

    if deposit_state.payer.ne(payer_acc.key()) {
        return Err(MyProgramError::InvalidOwner.into());
    }

    // Validate PDA
    FundraisingState::validate_pda(fundraising_state.bump, vesting_acc.key(), &payer_acc.key())?;
    DepositState::validate_pda(deposit_state.bump, deposit_state_acc.key(), &payer_acc.key())?;

    let mint_acc = Mint::from_account_info(mint)?; 

    let bump_binding = [fundraising_state.bump];
    // Signer seeds
    let signer_seeds = [
        Seed::from(FundraisingState::SEED.as_bytes()),
        Seed::from(&fundraising_state.initiator),
        Seed::from(&bump_binding),
    ];
    let signers = [Signer::from(&signer_seeds[..])];

    if fundraising_state.receive_amount < fundraising_state.required_amount {
        if deposit_state.payer.ne(&fundraising_state.initiator) {
            (
                TransferChecked {
                    from: vault,
                    to: payer_acc,
                    authority: vesting_acc,
                    mint: mint,
                    amount: deposit_state.amount,
                    decimals: mint_acc.decimals(),
                }
            ).invoke_signed(&signers)?;
        }
    }

    Ok(())
}
