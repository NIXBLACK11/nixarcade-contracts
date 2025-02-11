use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
use solana_program::system_instruction;
use solana_program::program::invoke;
use anchor_spl::associated_token::AssociatedToken;

pub mod state;
pub mod constant;
pub mod helper;
pub mod error;

use crate::{state::*, constant::*, helper::*, error::*};

const WSOL_MINT_ADDRESS: &str = "So11111111111111111111111111111111111111112"; // WSOL Mint address

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

        // Transfer SOL to WSOL token account
        invoke(
            &system_instruction::transfer(
                &ctx.accounts.player_pubkey.key(),
                &ctx.accounts.escrow_token_account.key(),
                one_player_bid,
            ),
            &[
                ctx.accounts.player_pubkey.to_account_info(),
                ctx.accounts.escrow_token_account.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        // Convert to WSOL
        invoke(
            &spl_token::instruction::sync_native(
                &spl_token::ID,
                &ctx.accounts.escrow_token_account.key(),
            )?,
            &[ctx.accounts.escrow_token_account.to_account_info()],
        )?;

        // Store game details
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
        
        // Transfer SOL to WSOL token account
        invoke(
            &system_instruction::transfer(
                &ctx.accounts.player_pubkey.key(),
                &ctx.accounts.escrow_token_account.key(),
                game_account.one_player_bid,
            ),
            &[
                ctx.accounts.player_pubkey.to_account_info(),
                ctx.accounts.escrow_token_account.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        // Convert to WSOL
        invoke(
            &spl_token::instruction::sync_native(
                &spl_token::ID,
                &ctx.accounts.escrow_token_account.key(),
            )?,
            &[ctx.accounts.escrow_token_account.to_account_info()],
        )?;

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

    #[account(
        init_if_needed,
        payer = player_pubkey,
        associated_token::mint = wsol_mint,
        associated_token::authority = game_account
    )]
    pub escrow_token_account: Account<'info, TokenAccount>,

    #[account(address = WSOL_MINT_ADDRESS.parse::<Pubkey>().unwrap())]
    pub wsol_mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
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

    #[account(
        init_if_needed,
        payer = player_pubkey,
        associated_token::mint = wsol_mint,
        associated_token::authority = game_account
    )]
    pub escrow_token_account: Account<'info, TokenAccount>,

    #[account(address = WSOL_MINT_ADDRESS.parse::<Pubkey>().unwrap())]
    pub wsol_mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}