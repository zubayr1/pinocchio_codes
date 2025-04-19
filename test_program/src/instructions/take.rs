use pinocchio::account_info::AccountInfo;
use pinocchio::program_error::ProgramError;
use pinocchio::program_result::ProgramResult;
use pinocchio_system::instructions::CreateAccount;
use pinocchio_token::pubkey;

pub fn process_take_instruction(
    accounts_info: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    let [
        taker_account,
        taker_ata_x,
        taker_ata_y,
        maker_account,
        maker_ata_x,
        maker_ata_y,
        escrow_account,
        vault_account,
        mint_a_account,
        mint_b_account,
        receive_account,
        token_program_account,
        system_program_account,
        _remaining @..

    ] = &accounts_info[..] else {
        return Err(ProgramError::InvalidAccountData);
    };

    let escrow_account = Esrow::from_account_info_unchecked(&escrow_account);

    assert_eq!(escrow_account.mint_a_account, *mint_a_account.key());
    assert_eq!(escrow_account.mint_b_account, *mint_b_account.key());

    let bump = unsafe{ *(instruction_data.as_ptr() as *const u8) }.to_le_bytes();
    let seed = [b"escrow", maker.key().as_slice(), bump.as_ref()];
    let seeds = &seed[..];

    let escrow_pda = from_account_info_unchecked(seeds, &crate::ID).0;

    assert_eq!(*escrow_account.key(), escrow_pda);




    Ok(())
}

