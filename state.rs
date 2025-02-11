use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct GameAccount {
    pub game_code: String,                // Unique game code for each game of the same type
    pub game_type: String,                // Type of game (e.g., "ttt", "ludo, "s&l")
    pub total_players_count: u8,          // Count of the total number of players in a game
    pub players_joined: u8,               // Number of players joined
    pub game_winner_pubkey: Pubkey,       // Winner's public key
    pub one_player_bid: u64,              // Bid amount per player
    pub player_pubkeys: [Pubkey; 4],      // Array to store up to 4 player public keys
    pub player_colors: [String; 4],       // Array to store up to 4 player colors
}