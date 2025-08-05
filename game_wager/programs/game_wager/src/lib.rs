use anchor_lang::prelude::*;
pub mod instructions;
pub mod constants;
pub mod state;
pub mod error;
pub mod helpers;

use instructions::*;


declare_id!("2bTvcg86gf1M8QbGaF9Xqtx3YCyNQzYCeGjMeNXaTF2q");

#[program]
pub mod game_wager {
    use super::*;

    pub fn initialize_game(ctx: Context<InitializeGame>, game_code: u8, game_type: u8,  wager: u64, num_players: u8) -> Result<()> {
        _initialize_game(ctx, game_code, game_type, wager, num_players)
    }

    pub fn join_game(ctx: Context<JoinGame>, game_code: u8, game_type: u8) -> Result<()> {
        _join_game(ctx, game_code, game_type)
    }

    pub fn end_game(ctx: Context<EndGame>, game_code: u8, game_type: u8, winner: Pubkey) -> Result<()> {
        _end_game(ctx, game_code, game_type, winner)
    }
}
