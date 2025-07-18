use pinocchio::program_error::ProgramError;

pub mod make;
pub mod take;
pub mod create_account;
pub mod create_account_with_seed;

pub use make::*;
pub use take::*;
pub use create_account::*;
pub use create_account_with_seed::*;

#[repr(u8)]
pub enum EscrowInstruction {
    MakeEscrow,
    TakeEscrow,
}

impl TryFrom<&u8> for EscrowInstruction {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match *value {
            0 => Ok(EscrowInstruction::MakeEscrow),
            1 => Ok(EscrowInstruction::TakeEscrow),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}

mod idl_gen {
    use super::{MakeEscrowIxData, TakeEscrowIxData};

    #[derive(shank::ShankInstruction)]
    enum _EscrowInstruction {
        #[account(0, writable, signer, name = "maker", desc = "The user creating the escrow")]
        #[account(1, name = "mint_a", desc = "The mint of the token the maker is offering")]
        #[account(2, writable, name = "escrow_acc", desc = "The escrow state account (PDA)")]
        #[account(3, writable, name = "vault", desc = "The vault token account (PDA)")]
        #[account(4, writable, name = "maker_ata_a", desc = "The maker's token account for mint A")]
        #[account(5, name = "system_program", desc = "System program")]
        #[account(6, name = "token_program", desc = "Token program")]
        #[account(7, name = "rent", desc = "Rent sysvar")]
        MakeEscrow(MakeEscrowIxData),

        #[account(0, writable, signer, name = "taker", desc = "The user fulfilling the escrow")]
        #[account(1, writable, name = "maker", desc = "The original maker of the escrow")]
        #[account(2, writable, name = "escrow_acc", desc = "The escrow state account to be closed")]
        #[account(3, writable, name = "vault", desc = "The vault account to be closed")]
        #[account(4, name = "mint_a", desc = "The mint of the token the maker offered")]
        #[account(5, name = "mint_b", desc = "The mint of the token the taker is offering")]
        #[account(6, writable, name = "taker_ata_a", desc = "The taker's token account for mint A")]
        #[account(7, writable, name = "taker_ata_b", desc = "The taker's token account for mint B")]
        #[account(8, writable, name = "maker_ata_b", desc = "The maker's token account for mint B")]
        #[account(9, name = "system_program", desc = "System program")]
        TakeEscrow(TakeEscrowIxData),
    }
}
