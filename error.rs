use anchor_lang::prelude::*;

#[error_code]
pub enum GameError {
    #[msg("The game is full!!")]
    GameFull,
    #[msg("Invalid player count for the selected game type!")]
    InvalidPlayerCount,
    #[msg("Invalid user trying to set winner!")]
    Unauthorized,
    #[msg("No valid winner Pubkey found!")]
    WinnerNotFound,
    #[msg("No valid match for winner found!")]
    WinnerMismatch,
    #[msg("No valid color found in the array!")]
    InvalidWinnerColor
}
