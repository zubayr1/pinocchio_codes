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
    // Invalid Payer Index
    InvalidPayerIndex,
    // Invalid Payer
    InvalidPayer,
    // Invalid Vote
    InvalidVote,
    // Invalid Transaction Index
    InvalidTransactionIndex,
    // Invalid Transaction Status
    InvalidTransactionStatus,
    // Invalid Proposal Status
    InvalidProposalStatus,
    // Insufficient Votes
    InsufficientVotes,
    // Invalid Transaction
    InvalidTransaction,
    // Invalid Proposal
    InvalidProposal,
    // Unauthorized
    Unauthorized,
    // Invalid Executor Index
    InvalidExecutorIndex,
}

impl From<MyProgramError> for ProgramError {
    fn from(e: MyProgramError) -> Self {
        Self::Custom(e as u32)
    }
}
