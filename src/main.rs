mod tic_tac_toe;
use crate::tic_tac_toe::game::Game;

fn main() {
    let mut game = Game::new(3, vec!['X', 'Y', 'Z']);
    game.play();
}
