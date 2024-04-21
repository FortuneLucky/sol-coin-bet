use anchor_lang::error_code;

#[error_code]
pub enum ContestError {
    #[msg("ContestError: Not allowed owner")]
    NotAllowedOwner,

    #[msg("ContestError: Invalid Contest Index")]
    InvalidContestIndex,

    #[msg("ContestError: Over Time Contest")]
    OverTimeContest,

    #[msg("ContestError: Invalid Token")]
    InvalidToken,

    #[msg("Invalid Price Feed")]
    InvalidPriceFeed,

    #[msg("Not Fiished")]
    NotFinished,

    #[msg("Contest already set")]
    AlreadySet,

    #[msg("Didn't set winner yet")]
    DidntSetWinner,

    #[msg("Not Winner")]
    NotWinner,

    #[msg("No time to withdraw token")]
    NotWithdrawTime,

    #[msg("Already Claimed")]
    AlreadyClaimed,

    #[msg("max amount to deposit")]
    MaxAmount
}
