use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::system_instruction::transfer;
use crate::{
    constants::ACCOUNT_DISCRIMINATOR_SIZE,
    error::GameError,
    helpers::get_game_details,
    state::Game
};

#[derive(Accounts)]
#[instruction(game_code: u8, game_type: u8)]
pub struct InitializeGame<'info> {
    #[account(mut)]
    pub first_player: Signer<'info>,
    #[account(
        init,
        payer = first_player,
        space = ACCOUNT_DISCRIMINATOR_SIZE + Game::INIT_SPACE,
        seeds = [b"game", &[game_code][..], &[game_type][..]],
        bump
    )]
    pub game_account: Account<'info, Game>,
    pub system_program: Program<'info, System>,
}

pub fn _initialize_game(ctx: Context<InitializeGame>, game_code: u8, game_type: u8, wager: u64, num_players: u8) -> Result<()> {
    let first_player = & ctx.accounts.first_player;
    let game_account = &mut ctx.accounts.game_account;
    
    require!(wager > 0, GameError::InvalidAmount);

    let game_details = get_game_details(game_type)?;
    require!(game_details.min <= num_players && num_players <= game_details.max,  GameError::InvaildNumberPlayers);

    invoke(
        &transfer(
            &first_player.key(),
            &game_account.key(),
            wager,
        ),
        &[
            first_player.to_account_info(),
            game_account.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    game_account.player_index = 1;
    game_account.num_players = num_players;
    game_account.players = [first_player.key(), Pubkey::default(), Pubkey::default(), Pubkey::default()];
    game_account.wager = wager;
    game_account.game_type = game_type;
    game_account.game_code = game_code;

    Ok(())
}