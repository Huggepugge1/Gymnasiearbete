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

pub struct Piece {
    pub color: i8,
    pub piece_type: i8
}

pub struct Distance {
    pub files: i64,
    pub ranks: i64
}

pub fn gen_sliding_moves(board: [i8; 64], pos: i64) -> i64 {
    let piece = board::find_piece(board[pos as usize]);
    let diagonal_offsets: [i64; 4] = [-9, -7, -7, 9];
    let straight_offsets: [i64; 4] = [-8, -1, 1, 8];
    let mut moves: i64 = 0;

    if piece.piece_type == BISHOP || piece.piece_type == QUEEN {
        for offset in diagonal_offsets {
            let mut steps: i64 = 1;
            while true {
                let distance = check_distance(pos, pos + offset*steps);
                if !(distance.files == distance.ranks) {
                    break;
                }

                if 0 > (pos + offset*steps) || (pos + offset*steps) > 63 {
                    break;
                }

                if board[(pos + offset*steps) as usize] != 0 {
                    if board::find_piece(board[(pos + offset*steps) as usize]).color != piece.color {
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

            while true {
                let distance = check_distance(pos, pos + offset*steps);
                if !(distance.files == 0 || distance.ranks == 0) {
                    break;
                }

                if 0 > (pos + offset*steps) && (pos + offset*steps) > 63 {
                    break;
                }

                if board[(pos + offset*steps) as usize] != 0 {
                    if board::find_piece(board[(pos + offset*steps) as usize]).color != piece.color {
                        moves |= 1 << (pos + (offset*steps));
                    }
                    break;
                }

                println!("{}", pos + (offset*steps));
                moves |= 1 << (pos + (offset*steps));
                steps += 1;
            }
        }
    }

    moves
}

pub fn gen_knight_moves(board: [i8; 64], pos: i64) -> i64 {
    let piece = board::find_piece(board[pos as usize]);
    let offsets: [i64; 8] = [-17, -15, -10, -6, 6, 10, 15, 17];
    let mut moves = 0;

    if piece.piece_type == KNIGHT {
        for offset in offsets {
            if pos + offset < 64 && 0 <= pos + offset {
                let distance = check_distance(pos, pos + offset);
                if (distance.files == 2 && distance.ranks == 1) || (distance.files == 1 && distance.ranks == 2) {
                    if board::find_piece(board[(pos + offset) as usize]).color != piece.color {
                        moves |= 1 << (pos + offset);
                    }
                }
            }
        }
    }
    moves
}

pub fn check_distance(start: i64, end: i64) -> Distance {
    let files = ((start % 8) - (end % 8)).abs();
    let ranks = ((start / 8) - (end / 8)).abs();
    Distance{ files, ranks }
}