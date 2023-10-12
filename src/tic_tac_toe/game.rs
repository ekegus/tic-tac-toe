use super::{board::Board, human_player::HumanPlayer};

#[derive(PartialEq, Debug)]
pub enum GameState {
    InProgress,
    GameOver,
}

pub struct Game {
    game_state: GameState,
    players: Vec<HumanPlayer>,
    board: Board,
}

impl Game {
    pub fn new(board_size: usize, marks: Vec<char>) -> Self {
        let board = Board::new(board_size);

        let players: Vec<HumanPlayer> = marks
            .into_iter()
            .map(|mark| HumanPlayer::new(mark))
            .collect();

        Game {
            game_state: GameState::InProgress,
            players,
            board,
        }
    }

    fn get_current_player(&self) -> Option<&HumanPlayer> {
        let current_player = self.players.get(0);

        return current_player;
    }

    pub fn switch_turn(&mut self) {
        self.players.rotate_left(1);
    }

    pub fn play_round(&mut self) -> anyhow::Result<()> {
        let current_player = self
            .get_current_player()
            .expect("Unable to get the current player.");

        let position = current_player.get_position()?;
        let mark = current_player.get_mark();

        self.board.place_mark(mark, position)?;

        if self.board.has_won(mark) {
            println!("Player {} wins!", mark);
            self.board.print_board();
            self.game_state = GameState::GameOver;
            Ok(())
        } else {
            self.switch_turn();
            Ok(())
        }
    }

    pub fn play(&mut self) -> () {
        while self.game_state == GameState::InProgress {
            self.board.print_board();

            if let Err(err) = self.play_round() {
                println!("{}", err);
                continue;
            };

            if self.board.get_has_empty_positions() == false {
                println!("Draw!");

                self.board.print_board();
                self.game_state = GameState::GameOver
            };
        }
    }
}
