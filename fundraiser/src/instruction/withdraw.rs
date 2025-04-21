use pinocchio::{account_info::AccountInfo, program_error::ProgramError, pubkey::self};

use pinocchio_system::instruction::Transfer;

use solana_sdk::native_token::LAMPORTS_PER_SOL;

use crate::state::{DataLen, load_ix_data};

#[repr(C)]
#[derive(Clone, Debug, Copy, Default)]
pub struct WithdrawIxData {
    pub bump: u8;
}

impl DataLen for WithdrawIxData{
    const LEN: usize = core::mem::size_of::<WithdrawIxData>();
}

pub fn process_withdraw(
    accounts: &[AccountInfo]
) -> Result<()> {
    // check for accounts

    let [withdraw_acc: &AccountInfo, vault_acc: &AccountInfo, _system_program: &AccountInfo] = &accounts[..] else {
        return Err(ProgramError::InvalidAccountData);
    }

    if !withdraw_acc.is_signer() {
        return Err(ProgramError::MissingRequiredSigner);
    }

    let bump = data[0];

    let seeds = &[b"vault", withdraw_acc.key.as_ref(), &[bump]];

    let vault_pda = pubkey::find_program_address(&seeds, &crate::ID).unwrap();

    if vault_acc.key != vault_pda {
        return Err(ProgramError::InvalidAccountData);
    }

    let binding_bump = [bump];

    let signer_seeds = [
        Seed::from("vault".as_bytes()),
        Seed::from(withdraw_acc.key.as_ref()),
        Seed::from(&binding_bump),
    ];

    let signer = [Signer::from(&signer_seeds)];

    Transfer{
        from: vault_acc,
        to: withdraw_acc,
        lamports: vault_acc.lamports()
    }.invoke_signed(&signer)?;

    Ok(())
}
