use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer, CloseAccount, SetAuthority};
use solana_program::system_instruction;
use solana_program::program::{invoke, invoke_signed};
use anchor_spl::associated_token::AssociatedToken;
use std::str::FromStr;

pub mod state;
pub mod constant;
pub mod helper;
pub mod error;

use crate::{state::*, constant::*, helper::*, error::*};

const WSOL_MINT_ADDRESS: &str = "So11111111111111111111111111111111111111112"; // WSOL Mint
const ADMIN_PUBKEY: &str = "FhNZ5dafuzZLQXixkvRd2FP4XsDvmPyzaHnQwEtA1mPT"; // Replace with actual admin pubkey

declare_id!("GYF4HijcsfJYEpnxhrdMhVRz9X5xUoQ7P2ku9wzHyR8t");

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
        require!(is_valid_player_count(&game_type, total_players_count), GameError::InvalidPlayerCount);
        let player1_color = get_player1_color(&game_type);
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
        invoke(
            &spl_token::instruction::sync_native(
                &spl_token::ID,
                &ctx.accounts.escrow_token_account.key(),
            )?,
            &[ctx.accounts.escrow_token_account.to_account_info()],
        )?;
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
        require!(next_slot < game_account.total_players_count, GameError::GameFull);
        let next_color = get_next_player_color(&game_account.game_type, next_slot as usize);
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

    pub fn set_winner(
        ctx: Context<SetWinner>,
        game_type: String,
        game_code: String,
        winner_color: String,
    ) -> Result<()> {
        let game_account = &mut ctx.accounts.game_account;
        let escrow_token_account = &ctx.accounts.escrow_token_account;
        let winner_destination = &ctx.accounts.winner_destination;

        // Find the winner based on color
        let mut found_winner = Pubkey::default();
        for i in 0..(game_account.players_joined as usize) {
            if game_account.player_colors[i] == winner_color {
                found_winner = game_account.player_pubkeys[i];
                break;
            }
        }
        require!(found_winner != Pubkey::default(), GameError::InvalidWinnerColor);
        require!(winner_destination.key() == found_winner, GameError::WinnerMismatch);

        // Store the winner in the game account
        game_account.game_winner_pubkey = found_winner;

        // PDA Signer Seeds
        let seeds = &[
            GAME_TAG,
            game_type.as_bytes(),
            game_code.as_bytes(),
            &[ctx.bumps.game_account],
        ];
        let signer = &[&seeds[..]];

        // ✅ Fix: Corrected `set_authority` usage
        token::set_authority(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                SetAuthority {
                    current_authority: ctx.accounts.admin.to_account_info(),
                    account_or_mint: escrow_token_account.to_account_info(),
                },
            ),
            token::spl_token::instruction::AuthorityType::AccountOwner,
            Some(game_account.key()), // ✅ Removed extra `&`
        )?;

        // ✅ Transfer WSOL to the winner
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: escrow_token_account.to_account_info(),
                    to: winner_destination.to_account_info(),
                    authority: game_account.to_account_info(),
                },
                signer,
            ),
            escrow_token_account.amount,
        )?;

        // ✅ Close the WSOL escrow after transferring funds
        token::close_account(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                CloseAccount {
                    account: escrow_token_account.to_account_info(),
                    destination: winner_destination.to_account_info(),
                    authority: game_account.to_account_info(),
                },
                signer,
            ),
        )?;

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

#[derive(Accounts)]
#[instruction(game_type: String, game_code: String)]
pub struct SetWinner<'info> {
    #[account(mut, address = Pubkey::from_str(ADMIN_PUBKEY).unwrap())]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [GAME_TAG, game_type.as_bytes(), game_code.as_bytes()],
        bump,
    )]
    pub game_account: Box<Account<'info, GameAccount>>,

    #[account(
        mut,
        constraint = escrow_token_account.owner == game_account.key(),
        constraint = escrow_token_account.amount > 0 @ GameError::EmptyEscrow
    )]
    pub escrow_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = winner_destination.mint == escrow_token_account.mint,
        constraint = winner_destination.owner == game_account.game_winner_pubkey
    )]
    pub winner_destination: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
}

