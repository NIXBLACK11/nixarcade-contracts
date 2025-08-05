use anchor_lang::prelude::*;
use crate::{
    error::GameError,
    state::Game,
    helpers::validate_game_authority::is_valid_game_authority,
};

#[derive(Accounts)]
#[instruction(game_code: u8, game_type: u8)]
pub struct EndGame<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub winner: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [b"game", &[game_code][..], &[game_type][..]],
        bump,
        close = first_player
    )]
    pub game_account: Account<'info, Game>,
    #[account(mut)]
    pub first_player: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub fn _end_game(
    ctx: Context<EndGame>,
    game_code: u8,
    game_type: u8,
    winner: Pubkey,
) -> Result<()> {
    let signer = ctx.accounts.signer.key();
    let game_account = &ctx.accounts.game_account;
    let winner_account = &ctx.accounts.winner;
    let first_player = &ctx.accounts.first_player;

    require!(is_valid_game_authority(&signer), GameError::NotAuthorized);

    require!(game_account.players[0] == first_player.key(), GameError::FirstPlayerMismatch);

    require!(game_account.players.contains(&winner), GameError::InvalidWinner);

    let lamports = game_account.to_account_info().lamports();
    let rent_exempt = Rent::get()?.minimum_balance(game_account.to_account_info().data_len());
    let amount = lamports.saturating_sub(rent_exempt);

    **game_account.to_account_info().try_borrow_mut_lamports()? -= amount;
    **winner_account.to_account_info().try_borrow_mut_lamports()? += amount;

    Ok(())
}