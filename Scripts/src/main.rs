mod board;
mod moves;

const WHITE: u8 = 8;
const BLACK: u8 = 16;

fn main() {
    let mut board: board::Board = board::create_board();
    board::print_board(&board);
    board = moves::make_move(board, 11, 27);
    board = moves::make_move(board, 2, 20);
    board = moves::make_move(board, 1, 18);
    board = moves::make_move(board, 3, 11);
    board = moves::make_move(board, 4, 2);
    board::print_board(&board);
    println!("{:b}", board.castling);
}
