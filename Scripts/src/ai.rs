use crate::board;
use crate::moves;

use rand::seq::SliceRandom;

pub const EASY: i8 = 0;

pub fn generate_moves(board: &Box<board::Board>) -> Vec<(i8, i8)> {
    let mut moves: Vec<(i8, i8)> = Vec::new();
    for start_square in 0..64 {
        let current_moves: u64 = moves::get_legal_moves(board, start_square);
        for end_square in 0..64 {
            if (1 << (end_square) as u64) & current_moves == 0 {
                continue;
            }
            let board_copy1: Box<board::Board> = board::copy_board(board);
            let board_copy2: Box<board::Board> = board::copy_board(board);
            if moves::make_move(board_copy1, start_square, end_square) != board_copy2 {
                moves.push((start_square, end_square));
            }
        }
    }
    moves
}

// Gets the amount of a specific piece
// For example number of white pawns
pub fn amount_of_pieces(pieces: u64) -> i64 {
    let mut count: i64 = 0;
    for i in 0..64 {
        if pieces & (1 << i) > 0 {
            count += 1;
        }
    }
    count
}

// Every position gets a score based on the amount of white pieces vs the amount of black pieces
pub fn eval_pos(board: &Box<board::Board>) -> i64 {
    let white_pawns:   i64 = amount_of_pieces(board.white_pieces & board.pawns);
    let white_rooks:   i64 = amount_of_pieces(board.white_pieces & board.rooks) * 5;
    let white_knights: i64 = amount_of_pieces(board.white_pieces & board.knights) * 3;
    let white_bishops: i64 = amount_of_pieces(board.white_pieces & board.bishops) * 3;
    let white_queens:  i64 = amount_of_pieces(board.white_pieces & board.queens) * 9;
    
    let black_pawns:   i64 = amount_of_pieces(board.black_pieces & board.pawns);
    let black_rooks:   i64 = amount_of_pieces(board.black_pieces & board.rooks) * 5;
    let black_knights: i64 = amount_of_pieces(board.black_pieces & board.knights) * 3;
    let black_bishops: i64 = amount_of_pieces(board.black_pieces & board.bishops) * 3;
    let black_queens:  i64 = amount_of_pieces(board.black_pieces & board.queens) * 9;
    
    let eval: i64 = 
        white_pawns
        + white_rooks
        + white_knights
        + white_bishops
        + white_queens
        
        - black_pawns
        - black_rooks
        - black_knights
        - black_bishops
        - black_queens;
    eval
}

pub fn random_move(board: &Box<board::Board>) -> (i8, i8) {
    println!("{:?}", [board.promoted, board.promoted_piece as i8, board.turn as i8]);

    if board.promoted != -1 {
        let piece: i8 = *vec![2, 3, 4, 5].choose(&mut rand::thread_rng()).unwrap();
        println!("{}", eval_pos(board));
        (piece, 0)

    } else {
        *generate_moves(board).choose(&mut rand::thread_rng()).unwrap()
    }
}
