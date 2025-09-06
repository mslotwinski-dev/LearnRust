mod board;
mod genome;
mod terrain;

use crate::board::Board;

fn main() {
    let mut board = Board::new(40, 20);
    board.set_target(33, 12);
    board.set_pos(11, 3);
    board.generate_walls(0.15);

    board.print_board();

    loop {
        board.next_generation();
        board.print_board_with_path();
    }
}
