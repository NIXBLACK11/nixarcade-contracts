use anchor_lang::prelude::*;
use crate::{
    error::GameError,
};

/*
    GAME
    0 -> Ludo
    1 -> TicTacToe
    2 -> Snake and Ladder
*/

pub struct GameLimits {
    pub min: u8,
    pub max: u8,
}

pub fn get_min_players(game_type: u8) -> Option<u8> {
    match game_type {
        0 => Some(2),
        1 => Some(2),
        2 => Some(2),
        _ => None,
    }
}

pub fn get_max_players(game_type: u8) -> Option<u8> {
    match game_type {
        0 => Some(4),
        1 => Some(2),
        2 => Some(4),
        _ => None,
    }
}

pub fn get_game_details(
    game_type: u8
) -> Result<GameLimits> {
    match (get_min_players(game_type), get_max_players(game_type)) {
        (Some(min), Some(max)) => Ok(GameLimits { min, max }),
        _ => Err(error!(GameError::InvalidGameType))
    }
}