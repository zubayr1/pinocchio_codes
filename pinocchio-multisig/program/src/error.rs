use pinocchio::program_error::ProgramError;

#[derive(Clone, PartialEq, shank::ShankType)]
pub enum MyProgramError {
    // overflow error
    WriteOverflow,
    // invalid instruction data
    InvalidInstructionData,
    // pda mismatch
    PdaMismatch,
    // Invalid Owner
    InvalidOwner,
    // Invalid Mint
    InvalidMint,
    // Invalid Treasury
    InvalidTreasury,
    // Invalid Multisig
    InvalidMultisig,
    // Invalid Threshold
    InvalidThreshold,
    // Invalid Index
    InvalidIndex,
    // Overflow
    Overflow,
    // Underflow
    Underflow,
    // Member Not Found
    MemberNotFound,
}

impl From<MyProgramError> for ProgramError {
    fn from(e: MyProgramError) -> Self {
        Self::Custom(e as u32)
    }
}
