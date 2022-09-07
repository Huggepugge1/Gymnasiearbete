mod board;
mod moves;

fn main() {
    let mut board: [i8; 64] = board::create_board();
    board[35] = 11;
    board::print_board(board);
    println!("{:#b}", moves::gen_knight_moves(board, 35));
}
