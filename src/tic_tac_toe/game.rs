use super::{board::Board, human_player::HumanPlayer};

#[derive(PartialEq, Debug)]
pub enum CurrentPlayer {
    Player1,
    Player2,
}

#[derive(PartialEq, Debug)]
pub enum GameState {
    InProgress,
    GameOver,
}

pub struct Game {
    current_player: CurrentPlayer,
    game_state: GameState,
}

impl Game {
    pub fn new() -> Self {
        Game {
            current_player: CurrentPlayer::Player1,
            game_state: GameState::InProgress,
        }
    }

    pub fn switch_turn(&mut self) {
        if self.current_player == CurrentPlayer::Player1 {
            self.current_player = CurrentPlayer::Player2;
        } else {
            self.current_player = CurrentPlayer::Player1;
        }
    }

    pub fn play_round(&mut self, board: &mut Board, player: &HumanPlayer) -> anyhow::Result<()> {
        let position = player.get_position()?;
        let mark = player.get_mark();

        board.place_mark(mark, position)?;

        if board.has_won(mark) {
            let mark = player.get_mark();
            println!("Player {} wins!", mark);
            board.print_board();
            self.game_state = GameState::GameOver;
            Ok(())
        } else {
            self.switch_turn();
            Ok(())
        }
    }

    pub fn play(&mut self, mut board: Board, player_1: HumanPlayer, player_2: HumanPlayer) -> () {
        while self.game_state == GameState::InProgress {
            board.print_board();

            if self.current_player == CurrentPlayer::Player1 {
                if let Err(err) = self.play_round(&mut board, &player_1) {
                    println!("{}", err);
                    continue;
                };
            } else {
                if let Err(err) = self.play_round(&mut board, &player_2) {
                    println!("{}", err);
                    continue;
                };
            }

            if board.get_has_empty_positions() == false {
                println!("Draw!");

                board.print_board();
                self.game_state = GameState::GameOver
            };
        }
    }
}
