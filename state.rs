use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct GameAccount {
    pub game_code: String,
    pub game_type: String,
    pub game_winner_pubkey: Pubkey,
    pub one_player_bid: u64,
    pub player1_pubkey: Pubkey,
    pub player2_pubkey: Pubkey,
    pub player1_color: String,
    pub player2_color: String,
}