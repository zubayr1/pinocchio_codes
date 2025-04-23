use pinocchio::program_error::ProgramError;

pub mod make;
pub mod take;

pub use make::*;
pub use take::*;

#[repr(u8)]
pub enum EscrowProgramInstrution {
    Make,
    Take,
}

impl TryFrom<&u8> for EscrowProgramInstrution {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match *value {
            0 => Ok(EscrowProgramInstrution::Make),
            1 => Ok(EscrowProgramInstrution::Take),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
