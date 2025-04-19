use pinocchio::account_info::AccountInfo;
use pinocchio::program_error::ProgramError;
use pinocchio::program_result::ProgramResult;
use pinocchio_system::instructions::CreateAccount;
use pinocchio_token::pubkey;

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
        _remaining @..

    ] = &accounts_info[..] else {
        return Err(ProgramError::InvalidAccountData);
    };

    let bump = unsafe{ *(instruction_data.as_ptr() as *const u8) }.to_le_bytes();
    let seed = [b"escrow", maker.key().as_slice(), bump.as_ref()];
    let signer_seeds = &seed[..];

    let pda = pubkey::checked_create_program_address(&seed, &crate::ID).unwrap();

    assert_eq!(&pda, escrow_account.key());

    unsafe{

        assert_eq!(mint_a_account.owner(), &pinocchio_token::ID);
        assert_eq!(mint_b_account.owner(), &pinocchio_token::ID);


    }

    CreateAccount{
        from: maker_account.key(),
        to: escrow_account.key(),
        lamports: Rent::get()?.minimum_balance(Escrow::LEN),
        space: Escrow::LEN,
        owner: crate::ID,
    }.invoke()?;

    let escrow_account = Esrow::from_account_info_unchecked(&escrow_account);

    escrow_account.maker = *maker_account.key();
    escrow_account.mint_a_account = *mint_a_account.key();
    escrow_account.mint_b_account = *mint_b_account.key();
    escrow_account.receive_account = *receive_account.key();
    escrow_account.bump = *data.as_ptr();

    pinocchio_token::instructions::Transfer {
        from: maker_ata_account,
        to: vault_account,
        amount: receive_account.amount,
        authority: maker_account.key(),
    }.invoke()?;

    Ok(())
}
