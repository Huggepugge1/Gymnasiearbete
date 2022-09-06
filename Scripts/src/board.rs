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

pub fn find_piece(piece: i8) -> Piece {
    let color = if piece & WHITE == WHITE { WHITE } else { BLACK };
    let mut piece_type: i8 = 0;
    if piece & PIECE == 0 {
        piece_type = EMPTY;
    } else if piece & PIECE == PAWN {
        piece_type = PAWN;
    } else if piece & PIECE == ROOK {
        piece_type = ROOK;
    } else if piece & PIECE == KNIGHT {
        piece_type = KNIGHT;
    } else if piece & PIECE == BISHOP {
        piece_type = BISHOP;
    } else if piece & PIECE == QUEEN {
        piece_type = QUEEN;
    } else if piece & PIECE == KING {
        piece_type = KING;
    }
    return Piece{color, piece_type}
}

pub fn print_board(board: [i8; 64]) {
    println!(" --- --- --- --- --- --- --- ---");
    for i in 0..8 {
        print!("|");
        for j in 0..8 {
            print!(" {} |", converter(find_piece(board[63 - ((i*8)+(7-j))])));
        }
        println!("");
        println!(" --- --- --- --- --- --- --- ---");
    }
}
// Converts i8 to understandable text
pub fn converter(piece: Piece) -> char {
    let mut c: char = ' ';
    if piece.piece_type == 0 {
        return c
    } else if piece.piece_type == 1 {
        c = 'p';
    } else if piece.piece_type == 2 {
        c = 'r';
    } else if piece.piece_type == 3 {
        c = 'n';
    } else if piece.piece_type == 4 {
        c = 'b';
    } else if piece.piece_type == 5 {
        c = 'q';
    } else if piece.piece_type == 6 {
        c = 'k';
    }
    return if piece.color == WHITE {
        c.to_ascii_uppercase()
    } else {
        c
    }
}

pub fn create_board() -> [i8; 64] {
    let mut board: [i8; 64] = [0; 64];
    board[0] = ROOK | WHITE;
    board[1] = KNIGHT | WHITE;
    board[2] = BISHOP | WHITE;
    board[3] = QUEEN | WHITE;
    board[4] = KING | WHITE;
    board[5] = BISHOP | WHITE;
    board[6] = KNIGHT | WHITE;
    board[7] = ROOK | WHITE;
    for i in 8..16 {
        board[i] = PAWN | WHITE;
    }
    for i in 16..48 {
        board[i] = EMPTY;
    }
    for i in 48..56 {
        board[i] = PAWN | BLACK;
    }
    board[56] = ROOK | BLACK;
    board[57] = KNIGHT | BLACK;
    board[58] = BISHOP | BLACK;
    board[59] = QUEEN | BLACK;
    board[60] = KING | BLACK;
    board[61] = BISHOP | BLACK;
    board[62] = KNIGHT | BLACK;
    board[63] = ROOK | BLACK;
    return board;
}