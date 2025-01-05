// Main

struct Game {
    board: [Option<char>; 9],
    current_turn: char,
    player_x: String,
    player_o: String,
}

impl Game {
    fn check_winner(self) -> Option<char> {
        None
    }

    fn make_move(mut self, player: char, position: usize) -> Result<(), String> {
        Ok(())
    }
}

fn main() {
    println!("Hello, world!");
}
