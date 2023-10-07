mod tic_tac_toe;
use crate::tic_tac_toe::board::Board;
use crate::tic_tac_toe::human_player::HumanPlayer;

fn main() {
    let mut board = Board::new();
    let mut player_01 = HumanPlayer::new('P');
    board.print_board();

    println!("{:?}", player_01.get_mark());
    println!("{:?}", player_01.get_position());

    // match board.place_mark('0', [0, 0]) {
    //     Ok(value) => println!("Result: {:?}", value),
    //     Err(error) => eprintln!("Error: {:?}", error),
    // }
    // match board.place_mark('0', [0, 1]) {
    //     Ok(value) => println!("Result: {:?}", value),
    //     Err(error) => eprintln!("Error: {:?}", error),
    // }
    // match board.place_mark('0', [0, 2]) {
    //     Ok(value) => println!("Result: {:?}", value),
    //     Err(error) => eprintln!("Error: {:?}", error),
    // }

    // board.print_board();
    // println!("{:?}", board.win_row('0'));

    // let mark = board.get_mark([0, 1]);
    // let is_empty = board.is_position_empty([0, 1]);
}
