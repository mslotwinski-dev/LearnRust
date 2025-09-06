mod board;
mod figure;

use crate::board::Board;

fn main() {
    let board = Board::new();
    board.display();
}
