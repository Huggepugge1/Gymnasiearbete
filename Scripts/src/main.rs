mod board;
mod moves;

fn main() {
    let mut board: [i8; 64] = board::create_board();
    board[32] = 10;
    board::print_board(board);
    println!("{:b}", moves::gen_sliding_moves(board, 32));
}
