use crate::board;

// Number representation of all pieces and colors
const EMPTY: u8 = 0;
const PAWN: u8 = 1;
const ROOK: u8 = 2;
const KNIGHT: u8 = 3;
const BISHOP: u8 = 4;
const QUEEN: u8 = 5;
const KING: u8 = 6;

const WHITE: u8 = 8;
const BLACK: u8 = 16;
// A way to represent distance across both files and ranks
pub struct Distance {
    pub files: i8,
    pub ranks: i8
}
// Calculates distance between to squares in files and ranks
pub fn calculate_distance(start: i8, end: i8) -> Distance {
    let files: i8 = ((start % 8) - (end % 8)).abs();
    let ranks: i8 = ((start / 8) - (end / 8)).abs();
    Distance{ files, ranks }
}
// Checks for check, true means check is present
pub fn check(board: &Box<board::Board>, pos: i8) -> bool {
    let white_pawn_offsets: [i8; 2] = [-9, -7];
    let black_pawn_offsets: [i8; 2] = [7, 9];
    let knight_offsets: [i8; 8] = [-17, -15, -10, -6, 6, 10, 15, 17];
    let diagonal_offsets: [i8; 4] = [-9, -7, 7, 9];
    let straight_offsets: [i8; 4] = [-8, -1, 1, 8];
    let king_offsets: [i8; 8] = [-9, -8, -7, -1, 1, 7, 8, 9];

    if board.turn == WHITE {
        for offset in black_pawn_offsets {
            if pos + offset > 64 {
                continue;
            }
            let piece: board::Piece = board::get_piece(board, pos + offset);

            if piece.piece_type == PAWN && piece.color != board.turn {
                return true;
            }
        }
    } else {
        for offset in white_pawn_offsets {
            if pos + offset < 0 {
                continue;
            }
            let piece: board::Piece = board::get_piece(board, pos + offset);

            if piece.piece_type == PAWN && piece.color != board.turn {
                return true;
            }
        }
    }

    for offset in knight_offsets {
        if pos + offset < 64 && 0 <= pos + offset {
            let distance: Distance = calculate_distance(pos, pos + offset);
            if !((distance.files == 1 && distance.ranks == 2) || (distance.files == 2 && distance.ranks == 1)) {
                continue;
            }

            let piece: board::Piece = board::get_piece(board, pos + offset);
            if piece.piece_type == KNIGHT && piece.color != board.turn {
                return true
            }
            if piece.piece_type != EMPTY {
                continue;
            }
        }
    }

    for offset in diagonal_offsets {
        let mut steps: i8 = 1;
        loop {
            let distance: Distance = calculate_distance(pos, pos + offset*steps);
            if !(distance.files == distance.ranks) {
                break;
            }

            if 0 > (pos + offset*steps) || (pos + offset*steps) > 63 {
                break;
            }

            let piece: board::Piece = board::get_piece(board, pos + offset*steps);
            if (piece.piece_type == BISHOP || piece.piece_type == QUEEN) && piece.color != board.turn {
                return true;
            }
            if piece.piece_type != EMPTY {
                break;
            }
            steps += 1;
        }
    }

    for offset in straight_offsets {
        let mut steps: i8 = 1;

        loop {
            println!("debug");
            let distance: Distance = calculate_distance(pos, pos + offset*steps);
            if !(distance.files == 0 || distance.ranks == 0) {
                continue;
            }

            if 0 > (pos + offset*steps) || (pos + offset*steps) > 63 {
                break;
            }

            let piece: board::Piece = board::get_piece(board, pos + offset*steps);
            if (piece.piece_type == ROOK || piece.piece_type == QUEEN) && piece.color != board.turn {
                return true;
            }
            if piece.piece_type != EMPTY {
                break;
            }

            steps += 1;
        }
    }

    for offset in king_offsets {
        // Makes sure move is inside the board
        if pos + offset < 64 && 0 <= pos + offset {
            // Makes sure king does not move to far
            // For example, if king is on a8 and offset is -1, the king will move to h7 which is not a legal move
            let distance: Distance = calculate_distance(pos, pos + offset);
            if distance.files > 1 || distance.ranks > 1 {
                continue;
            }
            // Checks if possible piece is on square and makes sure it is not your own
            let piece: board::Piece = board::get_piece(board, pos + offset);
            if piece.piece_type == KING && piece.color != board.turn {
                return true;
            }
        }
    }
    false
}

pub fn gen_pawn_moves(board: &Box<board::Board>, pos: i8) -> u64 {
    let piece: board::Piece = board::get_piece(&board, pos);
    let mut moves: u64 = 0;

    if piece.piece_type == PAWN {
        // Pawns move differently depending on whether there white or black
        if piece.color == WHITE {
            if board::get_piece(&board, pos + 8).piece_type == EMPTY {
                moves |= 1 << (pos + 8);
            }

            if pos / 8 == 1 && board::get_piece(&board, pos + 8).piece_type == EMPTY && board::get_piece(&board, pos + 16).piece_type == EMPTY {
                moves |= 1 << (pos + 16);
            }

            if board::get_piece(&board, pos + 7).color != piece.color && board::get_piece(&board, pos + 7).color != EMPTY && (pos % 8) - ((pos + 7) % 8) == 1 {
                moves |= 1 << (pos + 7);
            }

            if pos + 9 < 64 && board::get_piece(&board, pos + 9).color != piece.color && board::get_piece(&board, pos + 9).color != EMPTY && (pos % 8) - ((pos + 9) % 8) == -1 {
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

            if pos / 8 == 6 && board::get_piece(&board, pos - 8).piece_type == EMPTY && board::get_piece(&board, pos - 16).piece_type == EMPTY {
                moves |= 1 << (pos + -16);
            }

            if board::get_piece(&board, pos - 7).color != piece.color && board::get_piece(&board, pos - 7).color != EMPTY && (pos % 8) - ((pos - 7) % 8) == -1 {
                moves |= 1 << (pos + -7);
            }

            if pos - 9 > 0 && board::get_piece(&board, pos - 9).color != piece.color && board::get_piece(&board, pos - 9).color != EMPTY && (pos % 8) - ((pos - 9) % 8) == 1 {
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
// Check gen_king_moves for explanations of many pieces of code
pub fn gen_knight_moves(board: &Box<board::Board>, pos: i8) -> u64 {
    let piece: board::Piece = board::get_piece(board, pos);
    let offsets: [i8; 8] = [-17, -15, -10, -6, 6, 10, 15, 17];
    let mut moves: u64 = 0;

    if piece.piece_type == KNIGHT {
        for offset in offsets {
            if pos + offset < 64 && 0 <= pos + offset {
                let distance: Distance = calculate_distance(pos, pos + offset);
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
// Check gen_king_moves for explanations of many pieces of code
pub fn gen_sliding_moves(board: &Box<board::Board>, pos: i8) -> u64 {
    let piece: board::Piece = board::get_piece(board, pos);
    let diagonal_offsets: [i8; 4] = [-9, -7, 7, 9];
    let straight_offsets: [i8; 4] = [-8, -1, 1, 8];
    let mut moves: u64 = 0;

    if piece.piece_type == BISHOP || piece.piece_type == QUEEN {
        for offset in diagonal_offsets {
            let mut steps: i8 = 1;
            loop {
                let distance: Distance = calculate_distance(pos, pos + offset*steps);
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
                let distance: Distance = calculate_distance(pos, pos + offset*steps);
                if !((distance.files == 0 && distance.ranks != 0) || (distance.files != 0 && distance.ranks == 0)) {
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

pub fn gen_king_moves(board: &Box<board::Board>, pos: i8) -> u64 {
    let piece: board::Piece = board::get_piece(board, pos);
    let offsets: [i8; 8] = [-9, -8, -7, -1, 1, 7, 8, 9];
    let mut moves: u64 = 0;

    if piece.piece_type == KING {
        for offset in offsets {
            // Makes sure move is inside the board
            if pos + offset < 64 && 0 <= pos + offset {
                // Makes sure king does not move to far
                // For example, if king is on a8 and offset is -1, the king will move to h7 which is not a legal move
                let distance: Distance = calculate_distance(pos, pos + offset);
                if distance.files > 1 || distance.ranks > 1 {
                    continue;
                }
                // Checks if possible piece is on square and makes sure it is not your own
                let enemy_piece: board::Piece = board::get_piece(board, pos+offset);
                if enemy_piece.piece_type != EMPTY {
                    if enemy_piece.color != piece.color {
                        moves |= 1 << (pos + offset);
                    }
                    continue;
                }
                // Adds move to moves bitboard
                moves |= 1 << (pos + offset);
            }
        }
        // Castling
        if piece.color == WHITE {
            if board.castling & 1 == 1 {
                // Can't castle if something is in the way
                if board.white_pieces & ((1 << 2) - 1) << 5 == 0 && board.black_pieces & ((1 << 2) - 1) << 5 == 0 {
                    // Can't castle if check is anywhere from starting square to end square
                    if check_castling(board, pos, 1) {
                        moves |= 1 << (pos + 2);
                    }
                }
            }
            if board.castling & 2 == 2 {
                // Can't castle if something is in the way
                if board.white_pieces & ((1 << 3) - 1) << 1 == 0 && board.black_pieces & ((1 << 3) - 1) << 1 == 0 {
                    // Can't castle if check is anywhere from starting square to end square
                    if check_castling(board, pos, -1) {
                        moves |= 1 << (pos - 2);
                    }
                }
            }
        } else {
            if board.castling & 4 == 4 {
                // Can't castle if something is in the way
                if board.white_pieces & ((1 << 2) - 1) << 61 == 0 && board.black_pieces & ((1 << 2) - 1) << 61 == 0 {
                    // Can't castle if check is anywhere from starting square to end square
                    if check_castling(board, pos, 1) {
                        moves |= 1 << (pos + 2);
                    }
                }
            }
            if board.castling & 8 == 8 {
                // Can't castle if something is in the way
                if board.white_pieces & ((1 << 3) - 1) << 57 == 0 && board.black_pieces & ((1 << 3) - 1) << 57 == 0 {
                    // Can't castle if check is anywhere from starting square to end square
                    if check_castling(board, pos, -1) {
                        moves |= 1 << (pos - 2);
                    }
                }
            }
        }
    }
    
    moves
}

pub fn check_castling(board: &Box<board::Board>, pos: i8, offset: i8) -> bool {
    let mut board: Box<board::Board> = board::copy_board(board);
    if board.turn == WHITE {
        // Checks if check is present from first position
        if !check(&board, pos) {
            // Moves king one position to right or left
            board.kings &= (((1 << 63) - 1) + (1 << 63)) - (1 << pos);
            board.kings |= 1 << (pos + offset);
            board.white_pieces &= (((1 << 63) - 1) + (1 << 63)) - (1 << pos);
            board.white_pieces |= 1 << (pos + offset);
            // Checks if check is present from second position
            if !check(&board, pos + offset) {
                // Moves king one position to right or left again
                board.kings &= (((1 << 63) - 1) + (1 << 63)) - (1 << pos + offset);
                board.kings |= 1 << (pos + offset * 2);
                board.white_pieces &= (((1 << 63) - 1) + (1 << 63)) - (1 << pos + offset);
                board.white_pieces |= 1 << (pos + offset * 2);
                // Checks if check is present from end position
                if !check(&board, pos + offset * 2) {
                    return true;
                }
            }
        }
    } else {
        if !check(&board, pos) {
            // Moves king one position to right or left
            board.kings &= (((1 << 63) - 1) + (1 << 63)) - (1 << pos);
            board.kings |= 1 << (pos + offset);
            board.black_pieces &= (((1 << 63) - 1) + (1 << 63)) - (1 << pos);
            board.black_pieces |= 1 << (pos + offset);
            // Checks if check is present from second position
            if !check(&board, pos + offset) {
                // Moves king one position to right or left again
                board.kings &= (((1 << 63) - 1) + (1 << 63)) - (1 << pos + offset);
                board.kings |= 1 << (pos + offset * 2);
                board.black_pieces &= (((1 << 63) - 1) + (1 << 63)) - (1 << pos + offset);
                board.black_pieces |= 1 << (pos + offset * 2);
                // Checks if check is present from end position
                if !check(&board, pos + offset * 2) {
                    return true;
                }
            }
        }
    }
    false
}

pub fn make_move(mut board: Box<board::Board>, start: i8, end: i8) -> Box<board::Board> {
    let board_copy: Box<board::Board> = board::copy_board(&board);
    let piece = board::get_piece(&board, start);
    // Makes sure the piece you are trying to move is of your color
    if piece.color != board.turn {
        return board;
    }
    // For each piece, makes sure move is legal and moves the piece in the pieces corresponding bitmap
    // &= is used instead of -= to prevent possible bugs where the whole board gets filled by a single piece
    if piece.piece_type == EMPTY {
        return board;
    } else if piece.piece_type == PAWN {
        let moves: u64 = gen_pawn_moves(&board, start);

        if moves & (1 << end) == 0 {
            return board;
        }

        let enemy_piece: board::Piece = board::get_piece(&board, end);
        if enemy_piece.piece_type != piece.piece_type && enemy_piece.piece_type != EMPTY {
            if enemy_piece.piece_type == PAWN {
                board.pawns &= (((1 << 63) - 1) + (1 << 63)) - (1 << end);
            } else if enemy_piece.piece_type == ROOK {
                board.rooks &= (((1 << 63) - 1) + (1 << 63)) - (1 << end);
            } else if enemy_piece.piece_type == KNIGHT {
                board.knights &= (((1 << 63) - 1) + (1 << 63)) - (1 << end);
            } else if enemy_piece.piece_type == BISHOP {
                board.bishops &= (((1 << 63) - 1) + (1 << 63)) - (1 << end);
            } else if enemy_piece.piece_type == QUEEN {
                board.queens &= (((1 << 63) - 1) + (1 << 63)) - (1 << end);
            } else {
                board.kings &= (((1 << 63) - 1) + (1 << 63)) - (1 << end);
            }
        }
        // Executes en passant
        if (piece.color == WHITE && end == board.en_passant + 8) || (piece.color == BLACK && end == board.en_passant - 8) {
            if piece.color == WHITE {
                board.black_pieces &= (((1 << 63) - 1) + (1 << 63)) - (1 << board.en_passant);
            } else {
                board.white_pieces &= (((1 << 63) - 1) + (1 << 63)) - (1 << board.en_passant);
            }
            board.pawns &= (((1 << 63) - 1) + (1 << 63)) - (1 << board.en_passant);
        } else if piece.color == WHITE {
            board.black_pieces &= (((1 << 63) - 1) + (1 << 63)) - (1 << end);
        } else if piece.color == BLACK {
            board.white_pieces &= (((1 << 63) - 1) + (1 << 63)) - (1 << end);
        }
        // If pawn has moved two squares, en passant is possible on the end square, otherwise en passant is not possible on the following move
        if (start - end).abs() == 16 {
            board.en_passant = end;
        } else {
            board.en_passant = 0;
        }

        if (board.turn == WHITE && end / 8 == 7) || (board.turn == BLACK && end / 8 == 0) {
            board.pawns &= (((1 << 63) - 1) + (1 << 63)) - (1 << start);
            board.pawns |= 1 << end;
            board.promoted = end;
        } else {
            board.pawns &= (((1 << 63) - 1) + (1 << 63)) - (1 << start);
            board.pawns |= 1 << end;
            if board.turn == WHITE {
                board.turn = BLACK
            } else {
                board.turn = WHITE;
            };
        }

    } else if piece.piece_type == KNIGHT {
        let moves: u64 = gen_knight_moves(&board, start);

        if moves & (1 << end) == 0 {
            return board;
        }

        board.knights &= (((1 << 63) - 1) + (1 << 63)) - (1 << start);
        board.knights |= 1 << end;

    } else if piece.piece_type == KING {
        let moves: u64 = gen_king_moves(&board, start);

        if moves & (1 << end) == 0 {
            return board;
        }
        // Castling
        if end == start + 2 {
            if piece.color == WHITE {
                board.castling &= 12;
                board.rooks &= (((1 << 63) - 1) + (1 << 63)) - (1 << (start + 3));
                board.rooks |= 1 << (start + 1);
                board.white_pieces &= (((1 << 63) - 1) + (1 << 63)) - (1 << (start + 3));
                board.white_pieces |= 1 << (start + 1);
            } else {
                board.castling &= 3;
                board.rooks &= (((1 << 63) - 1) + (1 << 63)) - (1 << (start + 3));
                board.rooks |= 1 << (start + 1);
                board.black_pieces &= (((1 << 63) - 1) + (1 << 63)) - (1 << (start + 3));
                board.black_pieces |= 1 << (start + 1);
            }
        } else if end == start - 2 {
            if piece.color == WHITE {
                board.castling &= 12;
                board.rooks &= (((1 << 63) - 1) + (1 << 63)) - (1 << (start - 4));
                board.rooks |= 1 << (start - 1);
                board.white_pieces &= (((1 << 63) - 1) + (1 << 63)) - (1 << (start - 4));
                board.white_pieces |= 1 << (start - 1);
            } else {
                board.castling &= 3;
                board.rooks &= (((1 << 63) - 1) + (1 << 63)) - (1 << (start - 4));
                board.rooks |= 1 << (start - 1);
                board.black_pieces &= (((1 << 63) - 1) + (1 << 63)) - (1 << (start - 4));
                board.black_pieces |= 1 << (start - 1);
            }
        }
        // Removes castling when king is moved
        if piece.color == WHITE {
            board.castling &= 12;
        } else {
            board.castling &= 3;
        }

        board.kings &= (((1 << 63) - 1) + (1 << 63)) - (1 << start);
        board.kings |= 1 << end;

    } else {
        let moves: u64 = gen_sliding_moves(&board, start);

        if moves & (1 << end) == 0 {
            return board;
        }
        if piece.piece_type == ROOK {
            board.rooks &= (((1 << 63) - 1) + (1 << 63)) - (1 << start);
            board.rooks |= 1 << end;

            if piece.color == WHITE {
                if board.castling & 3 > 0 {
                    if start == 7 {
                        // ((1 << 4) - 1) - 1 = 1111 - 0001 = 1110
                        board.castling &= (1 << 4) - 2;
                    } else if start == 0 {
                        // ((1 << 4) - 1) - 2 = 1111 - 0010 = 1101
                        board.castling &= (1 << 4) - 3;
                    }
                }
            } else {
                if board.castling & 12 > 0 {
                    if start == 63 {
                        board.castling &= (1 << 4) - 4;
                    } else if start == 56 {
                        board.castling &= (1 << 4) - 8;
                    }
                }
            }
        } else if piece.piece_type == BISHOP {
            board.bishops &= (((1 << 63) - 1) + (1 << 63)) - (1 << start);
            board.bishops |= 1 << end;
        } else {
            board.queens &= (((1 << 63) - 1) + (1 << 63)) - (1 << start);
            board.queens |= 1 << end;
        }
    }

    // This part deletes the potential piece on the end square
    let enemy_piece = board::get_piece(&board, end);
    // Makes sure the piece trying to move is not deleted and also checks that the end square is not empty
    if piece.piece_type != PAWN && enemy_piece.piece_type != piece.piece_type && enemy_piece.piece_type != EMPTY {
        if enemy_piece.piece_type == PAWN {
            board.pawns &= (((1 << 63) - 1) + (1 << 63)) - (1 << end);
        } else if enemy_piece.piece_type == ROOK {
            board.rooks &= (((1 << 63) - 1) + (1 << 63)) - (1 << end);
        } else if enemy_piece.piece_type == KNIGHT {
            board.knights &= (((1 << 63) - 1) + (1 << 63)) - (1 << end);
        } else if enemy_piece.piece_type == BISHOP {
            board.bishops &= (((1 << 63) - 1) + (1 << 63)) - (1 << end);
        } else if enemy_piece.piece_type == QUEEN {
            board.queens &= (((1 << 63) - 1) + (1 << 63)) - (1 << end);
        } else {
            board.kings &= (((1 << 63) - 1) + (1 << 63)) - (1 << end);
        }
    }

    if enemy_piece.color != EMPTY && enemy_piece.color != piece.color && piece.piece_type != PAWN {
        if piece.color == WHITE {
            board.black_pieces &= (((1 << 63) - 1) + (1 << 63)) - (1 << end);
        } else if piece.color == BLACK {
            board.white_pieces &= (((1 << 63) - 1) + (1 << 63)) - (1 << end);
        }
    }

    // Moves the piece in the corresponding colors bitmap
    if piece.color == WHITE {
        board.white_pieces &= (((1 << 63) - 1) + (1 << 63)) - (1 << start);
        board.white_pieces |= 1 << end;
    } else if piece.color == BLACK{
        board.black_pieces &= (((1 << 63) - 1) + (1 << 63)) - (1 << start);
        board.black_pieces |= 1 << end;
    }

    // If the piece is not a pawn, no pawn can be captured using en_passant the move after
    if piece.piece_type != PAWN {
        board.en_passant = 0;
    }
    if check(&board, board::get_king_pos(&board, board.turn)) {
        return board_copy;
    } else {
        if board.turn == WHITE && piece.piece_type != PAWN  {
            board.turn = BLACK;
        } else if piece.piece_type != PAWN {
            board.turn = WHITE;
        }
    }
    board
}

pub fn promote_piece(mut board: Box<board::Board>) -> Box<board::Board> {
    if board.promoted_piece == ROOK {
        board.rooks |= 1<< board.promoted;
    } else if board.promoted_piece == KNIGHT {
        board.knights |= 1 << board.promoted;
    } else if board.promoted_piece == BISHOP {
        board.bishops |= 1 << board.promoted;
    } else if board.promoted_piece == QUEEN {
        board.queens |= 1 << board.promoted;
    }
    if board.turn == WHITE {
        board.turn = BLACK;
    } else {
        board.turn = WHITE;
    }
    board.pawns &= (((1 << 63) - 1) + (1 << 63)) - (1 << board.promoted);
    board.promoted = -1;
    board
}

pub fn get_legal_moves(board: &Box<board::Board>, pos: i8) -> u64 { 
    let piece = board::get_piece(board, pos);
    if piece.color != board.turn {
        0
    } else if piece.piece_type == PAWN {
        gen_pawn_moves(board, pos)
    } else if piece.piece_type == ROOK {
        gen_sliding_moves(board, pos)
    } else if piece.piece_type == KNIGHT {
        gen_knight_moves(board, pos)
    } else if piece.piece_type == BISHOP {
        gen_sliding_moves(board, pos)
    } else if piece.piece_type == QUEEN {
        gen_sliding_moves(board, pos)
    } else if piece.piece_type == KING {
        gen_king_moves(board, pos)
    } else {
        0
    }
}

