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
pub fn piece_score(pieces: u64) -> f64 {
    let mut score: f64 = 0.0;
    for i in 0..64 {
        if pieces & (1 << i) > 0 {
            score += 1.0 + (1.0 - f64::abs(3.5 - i as f64 % 8.0) / 3.5) * (1.0 - (f64::abs(3.5 - i as f64 / 8.0) / 3.5));
            // score += 1;
        }
    }
    score
}

// Every position gets a score based on the amount of white pieces vs the amount of black pieces
pub fn eval_pos(board: &board::Board) -> f64 {
    let moves = generate_moves(board).len();
    if moves == 0 {
        return if board.turn == board::WHITE {
            10000000.0
        } else {
            -10000000.0
        };
    }

    let white_pawns:   f64 = piece_score(board.white_pieces & board.pawns);
    let white_rooks:   f64 = piece_score(board.white_pieces & board.rooks)   * 5.0;
    let white_knights: f64 = piece_score(board.white_pieces & board.knights) * 3.0;
    let white_bishops: f64 = piece_score(board.white_pieces & board.bishops) * 3.0;
    let white_queens:  f64 = piece_score(board.white_pieces & board.queens)  * 9.0;

    let black_pawns:   f64 = piece_score(board.black_pieces & board.pawns);
    let black_rooks:   f64 = piece_score(board.black_pieces & board.rooks)   * 5.0;
    let black_knights: f64 = piece_score(board.black_pieces & board.knights) * 3.0;
    let black_bishops: f64 = piece_score(board.black_pieces & board.bishops) * 3.0;
    let black_queens:  f64 = piece_score(board.black_pieces & board.queens)  * 9.0;

    let eval: f64 =
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

pub fn sort_moves(board: &board::Board, current_moves: &mut Vec<moves::Move>) -> Vec<moves::Move> {
    let mut new_moves: Vec<moves::Move> = Vec::new();
    for current_move in current_moves.clone() {
        if board::get_piece(board, current_move.end).color != board::EMPTY {
            new_moves.push(current_move);
            current_moves.retain(|x| x != &current_move);
        }
    }
    for current_move in current_moves {
        new_moves.push(*current_move);
    }
    new_moves
}

// Better move generation
pub fn min_max(board: &board::Board, n: i64, parent_worst: f64) -> (moves::Move, f64) {
    let current_moves = sort_moves(board, &mut generate_moves(board));
    let mut best_move: (moves::Move, f64) = (moves::Move {
        start: -1,
        end: -1
    }, if board.turn == board::WHITE {
        -10000000000.0
    } else {
        10000000000.0
    });
    let mut current_worst: f64 = best_move.1;
    for current_move in current_moves {
        let mut current_pos = moves::make_move(board::copy_board(&board), moves::Move {
            start: current_move.start,
            end: current_move.end
        });
        if current_pos.promoted != -1 {
            current_pos.promoted_piece = 5;
            current_pos = moves::promote_piece(current_pos);
        }
        let eval: f64 = if n == 1 {
            eval_pos(board)
        } else {
            min_max(&current_pos, n - 1, current_worst).1
        };
        if current_pos.turn == board::WHITE {
            if eval < parent_worst {
                best_move.1 = -10000000000.0;
                return best_move;
            }
            if eval <= best_move.1 {
                best_move = (moves::Move {
                    start: current_move.start,
                    end: current_move.end
                }, eval);
            }
            current_worst = f64::max(current_worst, eval);
        } else {
            if eval > parent_worst {
                best_move.1 = 10000000000.0;
                return best_move;
            }
            if eval >= best_move.1 {
                best_move = (moves::Move {
                    start: current_move.start,
                    end: current_move.end
                }, eval);
            }
            current_worst = f64::min(current_worst, eval);
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
