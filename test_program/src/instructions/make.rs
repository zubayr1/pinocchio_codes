use pinocchio::account_info::AccountInfo;
use pinocchio::program_error::ProgramError;
use pinocchio::program_result::ProgramResult;
use pinocchio_system::instructions::CreateAccount;

pub fn process_make_instruction(
    accounts_info: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    let [
        maker_account,
        maker_ata_account,
        escrow_account,
        vault_account,
        mint_a_account,
        mint_b_account,
        receive_account,
        token_program_account,
        system_program_account,

    ] = &accounts_info[..] else {
        return Err(ProgramError::InvalidAccountData);
    };

    CreateAccount{
        from: maker_account.key(),
        to: escrow_account.key(),
        lamports: Rent::get()?.minimum_balance(Escrow::LEN),
        space: Escrow::LEN,
        owner: crate::ID,
    }.invoke()?;


    Ok(())
}
