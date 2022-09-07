use crate::board;

const EMPTY: i8 = 0;
const PAWN: i8 = 1;
const ROOK: i8 = 2;
const KNIGHT: i8 = 3;
const BISHOP: i8 = 4;
const QUEEN: i8 = 5;
const KING: i8 = 6;
const PIECE: i8 = 7;

const WHITE: i8 = 8;
const BLACK: i8 = 16;

pub struct Distance {
    pub files: i64,
    pub ranks: i64
}

pub fn check(board: &board::Board, en_passant: i64, turn: i8) -> bool {
    let mut attacks = 0;
    for square in 0..64 {
        let piece = board::get_piece(&board, square);

        if piece.piece_type == EMPTY { continue; };
        if piece.color == turn { continue; }

        attacks |= if piece.piece_type == PAWN {
            gen_pawn_moves(board, square, en_passant)
        } else if piece.piece_type == KNIGHT {
            gen_knight_moves(board, square)
        } else if piece.piece_type == KING {
            gen_king_moves(board, square)
        } else {
            gen_sliding_moves(board, square)
        };
    }
    if turn == WHITE {
        attacks & board.white_pieces & board.kings > 0
    } else {
        attacks & board.black_pieces & board.kings > 0
    }
}

pub fn gen_pawn_moves(board: &board::Board, pos: i64, en_passant: i64) -> u64 {
    let piece: board::Piece = board::get_piece(&board, pos);
    let mut moves: u64 = 0;

    if piece.piece_type == PAWN {
        if piece.color == WHITE{
            if board::get_piece(&board, pos + 8).piece_type == EMPTY {
                moves |= 1 << (pos + 8);
            }
            
            if board::get_piece(&board, pos + 8).piece_type == EMPTY && board::get_piece(&board, pos + 16).piece_type == EMPTY && pos / 8 == 1 {
                moves |= 1 << (pos + 16);
            }

            if board::get_piece(&board, pos + 7).color != piece.color && board::get_piece(&board, pos + 7).color != EMPTY && (pos % 8) - ((pos + 7) % 8) == -1 {
                moves |= 1 << (pos + 7);
            }

            if board::get_piece(&board, pos + 9).color != piece.color && board::get_piece(&board, pos + 9).color != EMPTY && (pos % 8) - ((pos + 9) % 8) == 1 {
                moves |= 1 << (pos + 9);
            }

            if pos - en_passant == 1 {
                moves |= 1 << (pos + 7);
            }

            if pos - en_passant == -1 {
                moves |= 1 << (pos + 9);
            }

        } else {
            if board::get_piece(&board, pos - 8).piece_type == EMPTY {
                moves |= 1 << (pos - 8);
            }
            
            if board::get_piece(&board, pos - 8).piece_type == EMPTY && board::get_piece(&board, pos - 16).piece_type == EMPTY && pos / 8 == 7 {
                moves |= 1 << (pos + -16);
            }

            if board::get_piece(&board, pos - 7).color != piece.color && board::get_piece(&board, pos - 7).color != EMPTY && (pos % 8) - ((pos - 7) % 8) == 1 {
                moves |= 1 << (pos + -7);
            }

            if board::get_piece(&board, pos - 9).color != piece.color && board::get_piece(&board, pos - 9).color != EMPTY && (pos % 8) - ((pos - 9) % 8) == -1 {
                moves |= 1 << (pos - 9);
            }
            
            if pos - en_passant == 1 {
                moves |= 1 << (pos - 9);
            }

            if pos - en_passant == -1 {
                moves |= 1 << (pos - 7);
            }
        }
    }

    moves
}

pub fn gen_knight_moves(board: &board::Board, pos: i64) -> u64 {
    let piece: board::Piece = board::get_piece(board, pos);
    let offsets: [i64; 8] = [-17, -15, -10, -6, 6, 10, 15, 17];
    let mut moves: u64 = 0;

    if piece.piece_type == KNIGHT {
        for offset in offsets {
            if pos + offset < 64 && 0 <= pos + offset {
                let distance: Distance = check_distance(pos, pos + offset);
                if !((distance.files == 1 && distance.ranks == 2) || (distance.files == 2 && distance.ranks == 1)) {
                    continue;
                }
                if board::get_piece(board, pos + offset).piece_type != EMPTY {
                    if board::get_piece(board, pos + offset).color != piece.color {
                        moves |= 1 << (pos + offset);
                    }
                    continue;
                }
                moves |= 1 << (pos + offset);
            }
        }
    }
    moves
}

pub fn gen_sliding_moves(board: &board::Board, pos: i64) -> u64 {
    let piece: board::Piece = board::get_piece(board, pos);
    let diagonal_offsets: [i64; 4] = [-9, -7, 7, 9];
    let straight_offsets: [i64; 4] = [-8, -1, 1, 8];
    let mut moves: u64 = 0;

    if piece.piece_type == BISHOP || piece.piece_type == QUEEN {
        for offset in diagonal_offsets {
            let mut steps: i64 = 1;
            loop {
                let distance: Distance = check_distance(pos, pos + offset*steps);
                if !(distance.files == distance.ranks) {
                    break;
                }

                if 0 > (pos + offset*steps) || (pos + offset*steps) > 63 {
                    break;
                }

                if board::get_piece(board, pos + (offset * steps)).piece_type != EMPTY {
                    if board::get_piece(board, pos + (offset*steps)).color != piece.color {
                        moves |= 1 << (pos + (offset*steps));
                    }
                    break;
                }

                moves |= 1 << (pos + (offset*steps));
                steps += 1;
            }
        }
    }
    
    if piece.piece_type == ROOK || piece.piece_type == QUEEN {
        for offset in straight_offsets {
            let mut steps: i64 = 1;

            loop {
                let distance: Distance = check_distance(pos, pos + offset*steps);
                if !(distance.files == 0 || distance.ranks == 0) {
                    break;
                }

                if 0 > (pos + offset*steps) || (pos + offset*steps) > 63 {
                    break;
                }

                if board::get_piece(board, pos + (offset * steps)).piece_type != EMPTY {
                    if board::get_piece(board, pos + (offset * steps)).color != piece.color {
                        moves |= 1 << (pos + (offset*steps));
                    }
                    break;
                }

                moves |= 1 << (pos + (offset*steps));
                steps += 1;
            }
        }
    }

    moves
}


pub fn gen_king_moves(board: &board::Board, pos: i64) -> u64 {
    let piece: board::Piece = board::get_piece(board, pos);
    let offsets: [i64; 8] = [-9, -8, -7, -1, 1, 7, 8, 9];
    let mut moves: u64 = 0;

    if piece.piece_type == KING {
        for offset in offsets {
            if pos + offset < 64 && 0 <= pos + offset {
                let distance: Distance = check_distance(pos, pos + offset);
                if distance.files > 1 || distance.ranks > 1 {
                    continue;
                }

                if board::get_piece(board, pos).piece_type != EMPTY {
                    if board::get_piece(board, pos).color != piece.color {
                        moves |= 1 << (pos + offset);
                    }
                    continue;
                }
                moves |= 1 << (pos + offset);
            }
        }
    }

    moves
}

pub fn check_distance(start: i64, end: i64) -> Distance {
    let files: i64 = ((start % 8) - (end % 8)).abs();
    let ranks: i64 = ((start / 8) - (end / 8)).abs();
    Distance{ files, ranks }
}