mod board;
mod moves;

const WHITE: u8 = 8;
const BLACK: u8 = 16;

fn main() {
    let mut board: board::Board = board::create_board();
    board::print_board(&board);
    board = moves::make_move(board, 12, 28);
    board::print_board(&board);
    board = moves::make_move(board, 28, 36);
    board::print_board(&board);
    board.turn = BLACK;
    board = moves::make_move(board, 51, 35);
    board::print_board(&board);
    board.turn = WHITE;
    board = moves::make_move(board, 36, 43);
    board::print_board(&board);
}
