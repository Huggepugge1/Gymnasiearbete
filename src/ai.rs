use crate::board;
use crate::moves;

use rand::seq::SliceRandom;

pub const EASY: i8 = 0;
pub const HARD: i8 = 1;

pub fn generate_moves(board: &board::Board) -> Vec<moves::Move> {
    let mut moves: Vec<moves::Move> = Vec::new();
    for start_square in 0..64 {
        let current_moves: u64 = moves::get_all_moves(board, start_square);
        for end_square in 0..64 {
            if (1 << (end_square) as u64) & current_moves == 0 {
                continue;
            }
            let board_copy1: board::Board = board::copy_board(board);
            let board_copy2: board::Board = board::copy_board(board);
            if moves::make_move(
                board_copy1,
                moves::Move {
                    start: start_square,
                    end: end_square
                }
            ) != board_copy2 {
                moves.push(
                    moves::Move {
                        start: start_square,
                        end: end_square
                    }
                );
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
pub fn eval_pos(board: &board::Board) -> i64 {
    let moves = generate_moves(board).len();
    if moves == 0 {
        return if board.turn == board::WHITE {
            10000000
        } else {
            -10000000
        };
    }

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

// Better move generation
pub fn min_max(board: &board::Board, n: i64) -> (moves::Move, i64) {
    let current_moves = generate_moves(board);
    let mut best_move: (moves::Move, i64) = (moves::Move {
        start: -1,
        end: -1
    }, if board.turn == board::WHITE {
        -10000000000
    } else {
        10000000000
    });
    for current_move in current_moves {
        let mut current_pos = moves::make_move(board::copy_board(&board), moves::Move {
            start: current_move.start,
            end: current_move.end
        });
        if current_pos.promoted != -1 {
            current_pos.promoted_piece = 5;
            current_pos = moves::promote_piece(current_pos);
        }
        let eval: i64 = if n == 1 {
            eval_pos(board)
        } else {
            min_max(&current_pos, n - 1).1
        };
        if current_pos.turn == board::WHITE {
            if eval <= best_move.1 {
                best_move = (moves::Move {
                    start: current_move.start,
                    end: current_move.end
                }, eval);
            }
        } else {
            if eval >= best_move.1 {
                best_move = (moves::Move {
                    start: current_move.start,
                    end: current_move.end
                }, eval);
            }
        }
    }
    best_move
}

pub fn random_move(board: &board::Board) -> moves::Move {
    if board.promoted != -1 {
        let piece: i8 = *vec![2, 3, 4, 5].choose(&mut rand::thread_rng()).unwrap();
        moves::Move {
            start: piece,
            end: -1,
        }

    } else {
        *generate_moves(board).choose(&mut rand::thread_rng()).unwrap()
    }
}
