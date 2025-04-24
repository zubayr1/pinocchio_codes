use pinocchio::program_error::ProgramError;

mod initialize_config;
mod initialize_user;
mod stake;
mod unstake;

pub use initialize_config::*;
pub use initialize_user::*;
pub use stake::*;
pub use unstake::*;

#[repr(u8)]
pub enum StakingInstructions {
    InitializeConfig = 0,
    InitializeUser = 1,
    Stake = 2,
    Unstake = 3,
}

impl TryFrom<u8> for StakingInstructions {
    type Error = ProgramError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::InitializeConfig),
            1 => Ok(Self::InitializeUser),
            2 => Ok(Self::Stake),
            3 => Ok(Self::Unstake),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
