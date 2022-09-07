mod board;
mod moves;

fn main() {
    let mut board: board::Board = board::create_board();
    let mut en_passant: i64 = 14;
    let square: i64 = 15;
    board::print_board(&board);
    println!("{}", moves::check(&board, en_passant, 8));
}
