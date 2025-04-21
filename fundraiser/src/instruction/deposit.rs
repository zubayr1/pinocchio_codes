use pinocchio::{account_info::AccountInfo, program_error::ProgramError, pubkey::self, ProgramResult};

use pinocchio_system::instructions::Transfer;

use solana_sdk::native_token::LAMPORTS_PER_SOL;

use crate::state::{DataLen, load_ix_data};

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct DepositIxData {
    pub amount: u16,
    pub bump: u8,
}

impl DataLen for DepositIxData {
    const LEN: usize = core::mem::size_of::<DepositIxData>();
}

pub fn process_deposit(
    accounts: &[AccountInfo],
    data: &[u8],
) -> Result<()> {
    // check for accounts

    let [deposit_account: &AccountInfo, vault_account: &AccountInfo, system_program: &AccountInfo] = &accounts[..] else {
        return Err(ProgramError::InvalidAccountData);
    };

    // let amount = u64::from_le_bytes(data);

    if !deposit_account.is_signer() {
        return Err(ProgramError::MissingRequiredSigner);
    }

    let ix_data = load_ix_data::<DepositIxData>(data)?;

    let seeds = &[b"vault", deposit_account.key.as_ref(), &[ix_data.bump]];

    let vault_pda = pubkey::create_program_address(&seeds, &crate::ID).unwrap();

    if vault_account.key != vault_pda {
        return Err(ProgramError::InvalidAccountData);
    }

    Transfer {
        from: deposit_account,
        to: vault_account,
        lamports: ix_data.amount * LAMPORTS_PER_SOL,
    }.invoke()?;

    Ok(())
}
