use crate::board;

const EMPTY: u8 = 0;
const PAWN: u8 = 1;
const ROOK: u8 = 2;
const KNIGHT: u8 = 3;
const BISHOP: u8 = 4;
const QUEEN: u8 = 5;
const KING: u8 = 6;

const WHITE: u8 = 8;
const BLACK: u8 = 16;

pub struct Distance {
pub files: i8,
pub ranks: i8
}

pub fn check_distance(start: i8, end: i8) -> Distance {
    let files: i8 = ((start % 8) - (end % 8)).abs();
    let ranks: i8 = ((start / 8) - (end / 8)).abs();
    Distance{ files, ranks }
}

pub fn check(board: &board::Board) -> bool {
    let mut attacks: u64 = 0;
    for square in 0..64 {
        let piece = board::get_piece(&board, square);

        if piece.piece_type == EMPTY { continue; };
        if piece.color == board.turn { continue; }

        attacks |= if piece.piece_type == PAWN {
            gen_pawn_moves(board, square)
        } else if piece.piece_type == KNIGHT {
            gen_knight_moves(board, square)
        } else if piece.piece_type == KING {
            gen_king_moves(board, square)
        } else {
            gen_sliding_moves(board, square)
        };
    }
    if board.turn == WHITE {
        attacks & board.white_pieces & board.kings > 0
    } else {
        attacks & board.black_pieces & board.kings > 0
    }
}

pub fn gen_pawn_moves(board: &board::Board, pos: i8) -> u64 {
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

            if pos - board.en_passant == 1 {
                moves |= 1 << (pos + 7);
            }

            if pos - board.en_passant == -1 {
                moves |= 1 << (pos + 9);
            }

        } else {
            if board::get_piece(&board, pos - 8).piece_type == EMPTY {
                moves |= 1 << (pos - 8);
            }

            if board::get_piece(&board, pos - 8).piece_type == EMPTY && board::get_piece(&board, pos - 16).piece_type == EMPTY && pos / 8 == 6 {
                moves |= 1 << (pos + -16);
            }

            if board::get_piece(&board, pos - 7).color != piece.color && board::get_piece(&board, pos - 7).color != EMPTY && (pos % 8) - ((pos - 7) % 8) == 1 {
                moves |= 1 << (pos + -7);
            }

            if board::get_piece(&board, pos - 9).color != piece.color && board::get_piece(&board, pos - 9).color != EMPTY && (pos % 8) - ((pos - 9) % 8) == -1 {
                moves |= 1 << (pos - 9);
            }

            if pos - board.en_passant == 1 {
                moves |= 1 << (pos - 9);
            }

            if pos - board.en_passant == -1 {
                moves |= 1 << (pos - 7);
            }
        }
    }

    moves
}

pub fn gen_knight_moves(board: &board::Board, pos: i8) -> u64 {
    let piece: board::Piece = board::get_piece(board, pos);
    let offsets: [i8; 8] = [-17, -15, -10, -6, 6, 10, 15, 17];
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

pub fn gen_sliding_moves(board: &board::Board, pos: i8) -> u64 {
    let piece: board::Piece = board::get_piece(board, pos);
    let diagonal_offsets: [i8; 4] = [-9, -7, 7, 9];
    let straight_offsets: [i8; 4] = [-8, -1, 1, 8];
    let mut moves: u64 = 0;

    if piece.piece_type == BISHOP || piece.piece_type == QUEEN {
        for offset in diagonal_offsets {
            let mut steps: i8 = 1;
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
            let mut steps: i8 = 1;

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


pub fn gen_king_moves(board: &board::Board, pos: i8) -> u64 {
    let piece: board::Piece = board::get_piece(board, pos);
    let offsets: [i8; 8] = [-9, -8, -7, -1, 1, 7, 8, 9];
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


pub fn make_move(mut board: board::Board, start: i8, end: i8) -> board::Board {
    let piece = board::get_piece(&board, start);
    if piece.color != board.turn {
        return board;
    }
    if piece.piece_type == EMPTY {
        return board;
    } else if piece.piece_type == PAWN {
        let moves: u64 = gen_pawn_moves(&board, start);

        if moves & (1 << end) == 0 || check(&board) {
            return board;
        }

        if (piece.color == WHITE && end == board.en_passant + 8) || (piece.color == BLACK && end == board.en_passant - 8) {
            if piece.color == WHITE {
                board.black_pieces &= (((1 << 63) - 1) + (1 << 63)) - (1 << board.en_passant);
            } else {
                board.white_pieces &= (((1 << 63) - 1) + (1 << 63)) - (1 << board.en_passant);
            }
        }

        if (start - end).abs() == 16 {
            board.en_passant = end;
        } else {
            board.en_passant = 0;
        }

        board.pawns &= (((1 << 63) - 1) + (1 << 63)) - (1 << start);
        board.pawns |= 1 << end;

    } else if piece.piece_type == KNIGHT {
        let moves: u64 = gen_knight_moves(&board, start);

        if moves & (1 << end) == 0 || check(&board) {
            return board;
        }

        board.knights &= (((1 << 63) - 1) + (1 << 63)) - (1 << start);
        board.knights |= 1 << end;

    } else if piece.piece_type == KING {
        let moves: u64 = gen_king_moves(&board, start);

        if moves & (1 << end) == 0 || check(&board) {
            return board;
        }

        board.kings &= (((1 << 63) - 1) + (1 << 63)) - (1 << start);
        board.kings |= 1 << end;

    } else {
        let moves: u64 = gen_sliding_moves(&board, start);

        if moves & (1 << end) == 0 || check(&board) {
            return board;
        }

        if piece.piece_type == ROOK {
            board.rooks &= (((1 << 63) - 1) + (1 << 63)) - (1 << start);
            board.rooks |= 1 << end;
        } else if piece.piece_type == BISHOP {
            board.bishops &= (((1 << 63) - 1) + (1 << 63)) - (1 << start);
            board.bishops |= 1 << end;
        } else {
            board.queens &= (((1 << 63) - 1) + (1 << 63)) - (1 << start);
            board.queens |= 1 << end;
        }
    }

    let enemy_piece = board::get_piece(&board, end);

    if enemy_piece.piece_type == PAWN {

    }

    if piece.color == WHITE {
        board.white_pieces &= (((1 << 63) - 1) + (1 << 63)) - (1 << start);
        board.white_pieces |= 1 << end;
    } else {
        board.black_pieces &= (((1 << 63) - 1) + (1 << 63)) - (1 << start);
        board.black_pieces |= 1 << end;
    }

    if piece.piece_type != PAWN {
        board.en_passant = 0;
    }
    return board
}