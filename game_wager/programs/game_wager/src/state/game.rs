use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Game {
    pub player_index: u8,
    pub num_players: u8,
    pub players: [Pubkey; 4],
    pub wager: u64,
    pub game_type: u8,
    pub game_code: u8,
}