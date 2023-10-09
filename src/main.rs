mod tic_tac_toe;
use crate::tic_tac_toe::board::Board;
use crate::tic_tac_toe::game::Game;
use crate::tic_tac_toe::human_player::HumanPlayer;

fn main() {
    let mut board = Board::new(3);
    let player_1 = HumanPlayer::new('X');
    let player_2 = HumanPlayer::new('O');

    board.place_mark('X', [0, 2]).unwrap();
    board.place_mark('X', [1, 1]).unwrap();
    board.place_mark('X', [2, 0]).unwrap();

    board.print_board();

    // println!("{:?}", board.win_row('X'));
    println!("{:?}", board.win_diagonal('X'));

    // let mut game = Game::new();

    // game.play(board, player_1, player_2);
}
