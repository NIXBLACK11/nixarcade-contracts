pub fn get_player1_color(game_type: &str) -> String {
    let player1_color = if game_type == "ttt" {
        "O"
    } else if game_type == "s&l" || game_type == "ludo" {
        "red"
    } else {
        ""
    };

    player1_color.to_string()
}

pub fn get_next_player_color(game_type: &str, player_index: usize) -> String {
    let colors = if game_type == "ludo" || game_type == "s&l" {
        vec!["red", "blue", "green", "yellow"]
    } else if game_type == "ttt" {
        vec!["O", "X"]
    } else {
        vec![]
    };

    colors.get(player_index).unwrap_or(&"").to_string()
}

pub fn is_valid_player_count(game_type: &str, player_count: u8) -> bool {
    match game_type {
        "ttt" => player_count == 2,
        "s&l" | "ludo" => (2..=4).contains(&player_count),
        _ => false,
    }
}
