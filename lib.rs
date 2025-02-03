use anchor_lang::prelude::*;

pub mod error;
pub mod state;
pub mod constant;

use crate::{error::*, state::*, constant::*};

declare_id!("8xX5iHGmZgcNobANG2G6FuiYaVL3KqrAj1BQdtQQCEET");

#[program]
pub mod game {
    use super::*;

    pub fn initialize_game(
        ctx: Context<InitializeGame>,
        game_type: String,
        game_code: String,
        game_pool: u64
    ) -> Result<()> {
        let game_account = &mut ctx.accounts.game_account;

        let player1_color = "";
        if(game_type=="OX") {
            player1_color = "O";
        }
        
        game_account.game_code = game_code;
        game_account.game_type = game_type;
        game_account.game_pool = game_pool;
        game_account.player1_color = playe1_color;
        game_account.player1_pubkey = ctx.accounts.player1.key();
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(game_code: String, game_type: String)]
pub struct InitializeGame<'info> {
    #[account(mut)]
    pub player1: Signer<'info>,

    #[account(
        mut,
        seeds = [GAME_TAG, game_code, game_type],
        bump,
        payer: player1,
        space = ANCHOR_DISCRIMINATOR + std::mem::size_of::<GameAccount>(),
    )]
    pub game_account: Box<Account<'info, GameAccount>>,

    pub system_program: Program<'info, System>
}