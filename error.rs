use anchor_lang::prelude::*;

#[error_code]
pub enum GameError {
    #[msg("The game is full!!")]
    GameFull,
    #[msg("Invalid player count for the selected game type!")]
    InvalidPlayerCount,

}
