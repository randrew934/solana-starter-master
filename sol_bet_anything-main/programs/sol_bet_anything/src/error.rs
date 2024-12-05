use anchor_lang::prelude::*;

#[error_code]
pub enum AdminError {
    #[msg("The caller is not the current admin.")]
    NotAuthorized,

    #[msg("The fee must be between 0 and 100.")]
    InvalidFee,
}

#[error_code]
pub enum BetError {
    #[msg("The provided bet_key does not match the game details.")]
    BetKeyMismatch,

    #[msg("The number of options must be between 2 and 4.")]
    InvalidNumberOfOptions,

    #[msg("The betting period has expired.")]
    BetPeriodExpired,

    #[msg("This user has already placed a bet.")]
    BetAlreadyPlaced,

    #[msg("The game is still active and cannot be ended yet.")]
    GameStillActive,

    #[msg("Only the Judge or Admin can end the game.")]
    UnauthorizedAccess,

    #[msg("The game status is invalid for this action.")]
    InvalidGameStatus,

    #[msg("The winning option set or retrieved is invalid.")]
    InvalidWinnerOption,

    #[msg("The game has not been closed.")]
    GameNotEnded,

    #[msg("This user isn't a winner in this game.")]
    NotAWinner,

    #[msg("No bet was placed on this option.")]
    NoBetsPlaced,

    #[msg("No enough funds for the bet.")]
    InsufficientFunds,
}

#[error_code]
pub enum AppealError {
    #[msg("Unauthorized signer.")]
    UnauthorizedSigner,
}
