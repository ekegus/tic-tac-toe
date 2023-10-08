mod tic_tac_toe;
use crate::tic_tac_toe::board::Board;
use crate::tic_tac_toe::game::Game;
use crate::tic_tac_toe::human_player::HumanPlayer;

fn main() {
    let board = Board::new();
    let player_1 = HumanPlayer::new('X');
    let player_2 = HumanPlayer::new('O');

    let mut game = Game::new();

    game.play(board, player_1, player_2);
}
