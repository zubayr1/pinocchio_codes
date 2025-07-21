use pinocchio::program_error::ProgramError;

pub mod initialize_fundraising;
pub mod deposit;
pub mod create_account;
pub mod withdraw;

pub use initialize_fundraising::*;
pub use deposit::*;
pub use create_account::*;
pub use withdraw::*;

#[repr(u8)]
pub enum FundraisingVestingInstruction {
    InitializeFundraising,
    Deposit,
    Withdraw,
}

impl TryFrom<&u8> for FundraisingVestingInstruction {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match *value {
            0 => Ok(FundraisingVestingInstruction::InitializeFundraising),
            1 => Ok(FundraisingVestingInstruction::Deposit),
            2 => Ok(FundraisingVestingInstruction::Withdraw),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}

mod idl_gen {
    use super::{InitializeFundraisingStateIxData, DepositIxData};

    #[derive(shank::ShankInstruction)]
    enum _FundraisingVestingInstruction {
        #[account(0, writable, signer, name = "initiator_acc", desc = "Initiator account")]
        #[account(1, writable, name = "vesting_acc", desc = "Vesting account")]
        #[account(2, name = "mint_acc", desc = "Mint account")]
        #[account(3, name = "system_program_acc", desc = "System program account")]
        InitializeFundraising(InitializeFundraisingStateIxData),
        #[account(0, writable, signer, name = "depositor_acc", desc = "Depositor account")]
        #[account(1, writable, name = "vesting_acc", desc = "Vesting account")]
        #[account(2, name = "mint_acc", desc = "Mint account")]
        #[account(3, name = "system_program_acc", desc = "System program account")]
        Deposit(DepositIxData),
    }
}
