use anchor_lang::prelude::*;
use anchor_lang::solana_program::hash::hash;

pub mod state;
pub mod constant;

use crate::{state::*, constant::*};

declare_id!("8xX5iHGmZgcNobANG2G6FuiYaVL3KqrAj1BQdtQQCEET");

#[program]
pub mod game {
    use super::*;

    pub fn initialize_game(
        ctx: Context<InitializeGame>,
        game_type: String,
        game_code: String,
        one_player_bid: u64
    ) -> Result<()> {
        let game_account = &mut ctx.accounts.game_account;

        let mut player1_color = "";
        if game_type == "ttt" {
            player1_color = "O";
        }

        game_account.game_code = game_code;
        game_account.game_type = game_type;
        game_account.one_player_bid = one_player_bid;
        game_account.player1_color = player1_color.to_string();
        game_account.player1_pubkey = ctx.accounts.player1_pubkey.key();
        game_account.player2_pubkey = Pubkey::default();
        game_account.player2_color = "".to_string();
        game_account.game_winner_pubkey = Pubkey::default();

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(game_code: String, game_type: String)]
pub struct InitializeGame<'info> {
    #[account(mut)]
    pub player1_pubkey: Signer<'info>,

    #[account(
        init,
        seeds = [GAME_TAG, game_code.as_ref(), game_type.as_ref()],
        bump,
        payer= player1_pubkey,
        space = ANCHOR_DISCRIMINATOR + std::mem::size_of::<GameAccount>(),
    )]
    pub game_account: Box<Account<'info, GameAccount>>,

    pub system_program: Program<'info, System>
}
