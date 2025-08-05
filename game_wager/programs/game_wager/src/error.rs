use anchor_lang::prelude::*;

#[error_code]
pub enum GameError {
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Game is already full")]
    GameAlreadyFull,
    #[msg("Player has already joined this game")]
    PlayerAlreadyJoined,
    #[msg("Invalid game type")]
    InvalidGameType,
    #[msg("Invalid winner")]
    InvalidWinner,
    #[msg("Invalid game data provided in final instruction")]
    GameDataMismatch,
    #[msg("Invalid number of players")]
    InvaildNumberPlayers,
    #[msg("First player account does not match game creator")]
    FirstPlayerMismatch,
    #[msg("This account is not authorized to close the game")]
    NotAuthorized,
}