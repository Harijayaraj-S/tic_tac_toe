// Game

#[derive(Debug)]
pub struct Game {
    board: [Option<char>; 9],
    pub current_turn: char,
    pub player_x: String,
    pub player_o: String,
}

impl Game {
    pub fn new(player_x: String, player_o: String, current_turn: char) -> Self {
        Self {
            board: [None; 9],
            current_turn,
            player_x,
            player_o,
        }
    }

    pub fn check_winner(self) -> Option<char> {
        let mut x_indexes = Vec::new();
        let mut o_indexes = Vec::new();

        let winning = vec![
            vec![0, 1, 2],
            vec![3, 4, 5],
            vec![6, 7, 8],
            vec![0, 4, 8],
            vec![2, 5, 4],
            vec![2, 4, 6],
            vec![1, 4, 7],
            vec![2, 5, 8],
        ];

        for index in 0..self.board.len() {
            if let Some(v) = self.board[index] {
                if v == 'x' {
                    x_indexes.push(index);
                } else {
                    o_indexes.push(index);
                }
            }
        }

        for itm in winning.clone() {
            if itm.iter().all(|ele| x_indexes.contains(ele)) {
                return Some('x');
            }
        }

        for itm in winning {
            if itm.iter().all(|ele| o_indexes.contains(ele)) {
                return Some('o');
            }
        }

        None
    }

    pub fn make_move(&mut self, player: char, position: usize) -> Result<(), String> {
        if self.current_turn != player {
            return Err("This is not your turn".to_string());
        }

        self.current_turn = if player == 'x' { 'o' } else { 'x' };
        if self.board[position].is_none() {
            self.board[position] = Some(player);
        } else {
            return Err("Already marked".to_string());
        }

        Ok(())
    }

    pub fn print_board(&self) -> String {
        let mut board = String::new();

        for row in 0..3 {
            board.push_str("| ");
            for col in 0..3 {
                let index = row * 3 + col;
                board.push(self.board[index].unwrap_or(' '));
                board.push_str(" | ");
            }
            board.push('\n');
        }

        board
    }
}
