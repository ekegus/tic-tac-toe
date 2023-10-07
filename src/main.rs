mod tic_tac_toe;
use crate::tic_tac_toe::board::Board;

fn main() {
    let mut board = Board::new();
    board.print_board();

    match board.place_mark('0', [0, 0]) {
        Ok(value) => println!("Result: {:?}", value),
        Err(error) => eprintln!("Error: {:?}", error),
    }
    match board.place_mark('0', [0, 1]) {
        Ok(value) => println!("Result: {:?}", value),
        Err(error) => eprintln!("Error: {:?}", error),
    }
    match board.place_mark('0', [0, 2]) {
        Ok(value) => println!("Result: {:?}", value),
        Err(error) => eprintln!("Error: {:?}", error),
    }

    board.print_board();
    println!("{:?}", board.win_row('0'));

    // let mark = board.get_mark([0, 1]);
    // let is_empty = board.is_position_empty([0, 1]);
}
