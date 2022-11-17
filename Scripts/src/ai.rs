use crate::board;
use crate::moves;

use rand::seq::SliceRandom;

pub const EASY: i8 = 0;

pub fn generate_moves(board: &Box<board::Board>) -> Vec<(i8, i8)> {
    let mut moves: Vec<(i8, i8)> = Vec::new();
    for start_square in 0..64 {
        let piece = board::get_piece(board, start_square);
        let current_moves: u64 = moves::get_legal_moves(board, start_square);
        for end_square in 0..64 {
            if (1 << (end_square) as u64) & current_moves == 0 {
                continue;
            }
            let mut board_copy1: Box<board::Board> = board::copy_board(board);
            let mut board_copy2: Box<board::Board> = board::copy_board(board);
            if moves::make_move(board_copy1, start_square, end_square) != board_copy2 {
                moves.push((start_square, end_square));
            }
        }
    }
    moves
}

pub fn random_move(board: &Box<board::Board>) -> (i8, i8) {
    *generate_moves(board).choose(&mut rand::thread_rng()).unwrap()
}
