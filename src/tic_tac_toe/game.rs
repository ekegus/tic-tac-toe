use super::{board::Board, human_player::HumanPlayer};

#[derive(PartialEq, Debug)]
pub enum CurrentPlayer {
    Player1,
    Player2,
}

// TODO: Simplify the play function
// TODO: Maybe introduce a game state...

pub struct Game {
    current_player: CurrentPlayer,
}

impl Game {
    pub fn new() -> Self {
        Game {
            current_player: CurrentPlayer::Player1,
        }
    }

    pub fn switch_turn(&mut self) {
        if self.current_player == CurrentPlayer::Player1 {
            self.current_player = CurrentPlayer::Player2;
            println!("{:?}", self.current_player);
        } else {
            self.current_player = CurrentPlayer::Player1;
            println!("{:?}", self.current_player);
        }
    }

    pub fn play(&mut self, mut board: Board, player_1: HumanPlayer, player_2: HumanPlayer) -> () {
        while board.get_has_empty_positions() == true {
            board.print_board();

            if self.current_player == CurrentPlayer::Player1 {
                match player_1.get_position() {
                    Ok(position) => {
                        let mark = player_1.get_mark();
                        match board.place_mark(mark, position) {
                            Ok(()) => {
                                if board.has_won(mark) {
                                    println!("Victory");
                                    break;
                                } else {
                                    self.switch_turn();
                                }
                            }
                            Err(err_message) => {
                                println!("{}", err_message);
                                continue;
                            }
                        };
                    }
                    Err(err_message) => {
                        println!("{}", err_message);
                        continue;
                    }
                }
            } else {
                match player_2.get_position() {
                    Ok(position) => {
                        let mark = player_2.get_mark();
                        match board.place_mark(mark, position) {
                            Ok(()) => {
                                if board.has_won(mark) {
                                    println!("Victory");
                                    break;
                                } else {
                                    self.switch_turn();
                                }
                            }
                            Err(err_message) => {
                                println!("{}", err_message);
                                continue;
                            }
                        };
                    }
                    Err(err_message) => {
                        println!("{}", err_message);
                        continue;
                    }
                }
            }
        }

        // TODO: Fix this displaying no matter what
        println!("Draw!");
    }
}
