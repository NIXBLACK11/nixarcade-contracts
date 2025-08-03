use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::system_instruction::transfer;
use crate::{
    error::GameError,
    state::Game
};

#[derive(Accounts)]
#[instruction(game_code: u8, game_type: u8)]
pub struct JoinGame<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(
        mut,
        seeds = [b"game", &[game_code][..], &[game_type][..]],
        bump
    )]
    pub game_account: Account<'info, Game>,
    pub system_program: Program<'info, System>,
}

pub fn _join_game(ctx: Context<JoinGame>, game_code: u8, game_type: u8) -> Result<()> {
    let player = & ctx.accounts.player;
    let game_account = &mut ctx.accounts.game_account;
    let player_index = game_account.player_index;

    require!(player_index < game_account.num_players, GameError::GameAlreadyFull);

    require!((player_index as usize) < game_account.players.len(), GameError::GameAlreadyFull);

    require!(!game_account.players.contains(&player.key()), GameError::PlayerAlreadyJoined);

    invoke(
        &transfer(
            &player.key(),
            &game_account.key(),
            game_account.wager,
        ),
        &[
            player.to_account_info(),
            game_account.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    game_account.players[player_index as usize] = player.key();
    game_account.player_index = game_account.player_index + 1;

    Ok(())
}