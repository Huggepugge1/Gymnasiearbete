// A number representation of all pieces and colors
pub const EMPTY: u8 = 0;
pub const PAWN: u8 = 1;
pub const ROOK: u8 = 2;
pub const KNIGHT: u8 = 3;
pub const BISHOP: u8 = 4;
pub const QUEEN: u8 = 5;
pub const KING: u8 = 6;

pub const WHITE: u8 = 8;
pub const BLACK: u8 = 16;
// A collection of bitmaps to represent the board in a simple and efficient manner
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Board {
    pub pawns: u64,
    pub rooks: u64,
    pub knights: u64,
    pub bishops: u64,
    pub queens: u64,
    pub kings: u64,

    pub white_pieces: u64,
    pub black_pieces: u64,
    // Number representing a square where en passant is possible, most likely 0
    // 0 does not encounter any problems because there is no situation where en passant on square 0 is a possibility
    pub en_passant: i8,
    // Four bit number where each bit represents if one specific castle is legal
    // First bit is kings side castling with white
    // Second bit is queens side castling with white
    // Third bit is kings side castling with black
    // Fourth bit is queens side castling with black
    pub castling: u8,
    // 8 == white, 16 == black
    pub turn: u8,
    // Square of promotion, default = -1
    pub promoted: i8,
    pub promoted_piece: u8,
    pub current_move: u64
}
// A way of representing a piece
#[derive(Debug)]
pub struct Piece {
    pub color: u8,
    pub piece_type: u8,
}
// Represents a square as two numbers, the color and piece on it
// If no piece is present piece_type and color will be EMPTY which is equal to 0
pub fn get_piece(board: &Board, pos: i8) -> Piece {
    let color = if board.white_pieces & (1 << pos) > 0 {
        WHITE
    } else if board.black_pieces & (1 << pos) > 0 {
        BLACK
    } else {
        EMPTY
    };
    let piece_type: u8 = if color == EMPTY {
        EMPTY
    } else if (1 << pos) & board.pawns > 0 {
        PAWN
    } else if (1 << pos) & board.rooks > 0 {
        ROOK
    } else if (1 << pos) & board.knights > 0 {
        KNIGHT
    } else if (1 << pos) & board.bishops > 0 {
        BISHOP
    } else if (1 << pos) & board.queens > 0 {
        QUEEN
    } else {
        KING
    };
    Piece{color, piece_type}
}

pub fn get_king_pos(board: &Board) -> i8 {
    let king_bit_board = if board.turn == WHITE {
        board.white_pieces & board.kings
    } else {
        board.black_pieces & board.kings
    };
    for square in 0..64 {
        if king_bit_board & (1 << square) > 0 {
            return square;
        }
    }
    return -1
}

pub fn print_board(board: &Board) {
    println!(" --- --- --- --- --- --- --- ---");
    for i in 0..8 {
        print!("|");
        for j in 0..8 {
            print!(" {} |", converter(&get_piece(&board, 63 - ((i * 8) + (7 - j)))));
        }
        println!("");
        println!(" --- --- --- --- --- --- --- ---");
    }
}
// Converts u8 to understandable text
pub fn converter(piece: &Piece) -> char {
    let c;
    if piece.piece_type == PAWN {
        c = 'p';
    } else if piece.piece_type == ROOK {
        c = 'r';
    } else if piece.piece_type == KNIGHT {
        c = 'n';
    } else if piece.piece_type == BISHOP {
        c = 'b';
    } else if piece.piece_type == QUEEN {
        c = 'q';
    } else if piece.piece_type == KING {
        c = 'k';
    } else {
        c = ' ';
    }
    return if piece.color == WHITE {
        c.to_ascii_uppercase()
    } else {
        c
    }
}
// Creates the starting positions bitboard
pub fn create_board() -> Board {
    let pawns: u64 = (((1 << 8) - 1) << 8) + (((1 << 8) - 1) << 48);
    let rooks: u64 = (1 << 0) + (1 << 7) + (1 << 56) + (1 << 63);
    let knights: u64 = (1 << 1) + (1 << 6) + (1 << 57) + (1 << 62);
    let bishops: u64 = (1 << 2) + (1 << 5) + (1 << 58) + (1 << 61);
    let queens: u64 = (1 << 3) + (1 << 59);
    let kings: u64 = (1 << 4) + (1 << 60);

    let white_pieces: u64 = (1 << 16) - 1;
    let black_pieces: u64 = ((1 << 16) - 1) << 48;
    let en_passant: i8 = 0;
    let castling: u8 = (1 << 4) - 1;
    let turn: u8 = WHITE;
    let promoted: i8 = -1;
    let promoted_piece: u8 = 0;
    let current_move: u64 = 0;

    Board {
        pawns,
        knights,
        rooks,
        bishops,
        queens,
        kings,

        white_pieces,
        black_pieces,

        en_passant,
        castling,
        turn,
        promoted,
        promoted_piece,
        current_move
    }
}
// Convert immutable &Board to mutable Board
pub fn copy_board(board: &Board) -> Board {
    let pawns: u64 = board.pawns;
    let rooks: u64 = board.rooks;
    let knights: u64 = board.knights;
    let bishops: u64 = board.bishops;
    let queens: u64 = board.queens;
    let kings: u64 = board.kings;

    let white_pieces: u64 = board.white_pieces;
    let black_pieces: u64 = board.black_pieces;
    let en_passant: i8 = board.en_passant;
    let castling: u8 = board.castling;
    let turn: u8 = board.turn;
    let promoted: i8 = board.promoted;
    let promoted_piece: u8 = board.promoted_piece;
    let current_move: u64 = board.current_move;

    Board {
        pawns,
        rooks,
        knights,
        bishops,
        queens,
        kings,

        white_pieces,
        black_pieces,

        en_passant,
        castling,
        turn,
        promoted,
        promoted_piece,
        current_move
    }
}
