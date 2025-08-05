use anchor_lang::prelude::*;
use std::str::FromStr;

// authority1 pubkey: DEydVpVwhwXWM2UQHHodt7TY2ucWmh5DkBEoETAJUzXK
// authority2 pubkey: 3rPzgWyNkgSsLQ6VR4wfCmPMEwkziShNZWM8Gi16kguW
// authority3 pubkey: 5t8NUWjq7e3LS6vLfvT6pSf1yustTz9qQ7qDh3vrTN6B

pub const ALLOWED_AUTHORITIES: [&str; 3] = [
    "DEydVpVwhwXWM2UQHHodt7TY2ucWmh5DkBEoETAJUzXK",
    "3rPzgWyNkgSsLQ6VR4wfCmPMEwkziShNZWM8Gi16kguW",
    "5t8NUWjq7e3LS6vLfvT6pSf1yustTz9qQ7qDh3vrTN6B" 
];

pub fn is_valid_game_authority(signer: &Pubkey) -> bool {
    ALLOWED_AUTHORITIES.iter().any(|key| {
        Pubkey::from_str(key).map_or(false, |allowed| allowed == *signer)
    })
}