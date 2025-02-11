use anchor_lang::prelude::*;

pub mod state;
pub mod constant;
pub mod helper;
pub mod error;

use crate::{state::*, constant::*, helper::*, error::*};

declare_id!("F6gFgjMZZ9VK26X1uhAtqNV7K92LvxyWjE1L6B4W9vrV");

#[program]
pub mod game {
    use super::*;

    pub fn initialize_game(
        ctx: Context<InitializeGame>,
        game_type: String,
        game_code: String,
        total_players_count: u8,
        one_player_bid: u64
    ) -> Result<()> {
        let game_account = &mut ctx.accounts.game_account;

        let is_valid = is_valid_player_count(&game_type, total_players_count);
        if !is_valid {
            return Err(GameError::InvalidPlayerCount.into());
        }
        let player1_color = get_player1_color(&game_type);

        game_account.game_code = game_code;
        game_account.game_type = game_type;
        game_account.total_players_count = total_players_count;
        game_account.one_player_bid = one_player_bid;
        game_account.players_joined = 1;
        game_account.player_colors[0] = player1_color.to_string();
        game_account.player_pubkeys[0] = ctx.accounts.player_pubkey.key();
        game_account.game_winner_pubkey = Pubkey::default();

        Ok(())
    }

    pub fn join_game(
        ctx: Context<JoinGame>,
        game_type: String,
        game_code: String
    ) -> Result<()> {
        let game_account = &mut ctx.accounts.game_account;
        let next_slot = game_account.players_joined;
        if next_slot == game_account.total_players_count {
            return Err(GameError::GameFull.into());
        }

        let next_color = get_next_player_color(&game_account.game_type, next_slot as usize);
        game_account.player_pubkeys[next_slot as usize] = ctx.accounts.player_pubkey.key();
        game_account.player_colors[next_slot as usize] = next_color;
        game_account.players_joined += 1;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(game_type: String, game_code: String, total_players_count: u8, one_player_bid: u64)]
pub struct InitializeGame<'info> {
    #[account(mut)]
    pub player_pubkey: Signer<'info>,

    #[account(
        init,
        seeds = [GAME_TAG, game_type.as_bytes(), game_code.as_bytes()],
        bump,
        payer = player_pubkey,
        space = ANCHOR_DISCRIMINATOR + std::mem::size_of::<GameAccount>()
    )]
    pub game_account: Box<Account<'info, GameAccount>>,

    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(game_type: String, game_code: String)]
pub struct JoinGame<'info> {
    #[account(mut)]
    pub player_pubkey: Signer<'info>,

    #[account(
        mut,
        seeds = [GAME_TAG, game_type.as_bytes(), game_code.as_bytes()],
        bump,
    )]
    pub game_account: Box<Account<'info, GameAccount>>,

    pub system_program: Program<'info, System>
}
