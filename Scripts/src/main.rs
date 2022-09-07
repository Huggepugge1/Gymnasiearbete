mod board;
mod moves;

struct Board {
    pawns: u64,
    rooks: u64,
    knights: u64,
    bishops: u64,
    queens: u64,
    kings: u64,

    white_pieces: u64,
    black_pieces: u64
}

fn main() {
    let mut board: board::Board = board::create_board();
    let mut en_passant: i64 = 0;
    let square: i64 = 15;
    board::print_board(&board);
    println!("{:#b}", moves::gen_pawn_moves(&board, square, en_passant));
}
