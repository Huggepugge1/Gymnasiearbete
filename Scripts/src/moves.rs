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

pub fn gen_sliding_moves(board: [i8; 64], pos: i64) -> i64 {
    let piece = board::find_piece(board[pos as usize]);
    let diagonal_offsets: [i64; 4] = [-9, -7, -7, 9];
    let straight_offsets: [i64; 4] = [-8, -1, 1, 8];
    let mut moves: i64 = 0;
    if piece.piece_type == BISHOP {
        for offset in diagonal_offsets {
            let mut steps: i64 = 1;
            while ((pos as i64 % 8) - ((pos as i64 + offset*steps) % 8)).abs() == ((pos as i64 / 8) - ((pos as i64 + offset*steps) / 8)).abs() {
                if 0 > (pos + offset*steps) || (pos + offset*steps) > 63 {
                    break;
                }
                if board[(pos + offset*steps) as usize] != 0 {
                    if board::find_piece(board[(pos + offset*steps) as usize]).color != piece.color {
                        moves |= 1 << (pos as i64 + (offset*steps));
                    }
                    break;
                }
                println!("{}", pos as i64 + (offset*steps));
                moves |= 1 << (pos as i64 + (offset*steps));
                steps += 1;
            }
        }
    }
    else if piece.piece_type == ROOK {
        for offset in straight_offsets {
            let mut steps: i64 = 1;
            while ((pos as i64 % 8) - ((pos as i64 + offset*steps) % 8)).abs() == 0 || ((pos as i64 / 8) - ((pos as i64 + offset*steps) / 8)).abs() == 0 {
                if 0 > (pos + offset*steps) && (pos + offset*steps) > 63 {
                    break;
                }
                if board[(pos + offset*steps) as usize] != 0 {
                    if board::find_piece(board[(pos + offset*steps) as usize]).color != piece.color {
                        moves |= 1 << (pos as i64 + (offset*steps));
                    }
                    break;
                }
                println!("{}", pos as i64 + (offset*steps));
                moves |= 1 << (pos as i64 + (offset*steps));
                steps += 1;
            }
        }
    }
    return moves
}