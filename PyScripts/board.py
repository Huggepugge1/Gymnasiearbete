from fen_parser import fen_parser


def print_board(board):
    print("  ---------------------------------------")
    for i in range(8):
        row = board[(i * 8):(i * 8) + 8][::-1]
        for j in range(8):
            print(" | " + row[j], end="")
        print(" | ")
        print("  ---------------------------------------")


def compute_board():
    board = ["  " for square in range(64)]
    white_pawns = pawns & white_pieces
    black_pawns = pawns & black_pieces
    white_knights = knights & white_pieces
    black_knights = knights & black_pieces
    white_bishops = bishops & white_pieces
    black_bishops = bishops & black_pieces
    white_rooks = rooks & white_pieces
    black_rooks = rooks & black_pieces
    white_queens = queens & white_pieces
    black_queens = queens & black_pieces
    white_king = kings & white_pieces
    black_king = kings & black_pieces

    for i in range(64):
        if ("0" * (64 - len(bin(white_pawns)[2:])) + bin(white_pawns)[2:])[i] == "1":
            board[i] = "wp"

        if ("0" * (64 - len(bin(black_pawns)[2:])) + bin(black_pawns)[2:])[i] == "1":
            board[i] = "bp"

        if ("0" * (64 - len(bin(white_knights)[2:])) + bin(white_knights)[2:])[i] == "1":
            board[i] = "wn"

        if ("0" * (64 - len(bin(black_knights)[2:])) + bin(black_knights)[2:])[i] == "1":
            board[i] = "bn"

        if ("0" * (64 - len(bin(white_bishops)[2:])) + bin(white_bishops)[2:])[i] == "1":
            board[i] = "wb"

        if ("0" * (64 - len(bin(black_bishops)[2:])) + bin(black_bishops)[2:])[i] == "1":
            board[i] = "bb"

        if ("0" * (64 - len(bin(white_rooks)[2:])) + bin(white_rooks)[2:])[i] == "1":
            board[i] = "wr"

        if ("0" * (64 - len(bin(black_rooks)[2:])) + bin(black_rooks)[2:])[i] == "1":
            board[i] = "br"

        if ("0" * (64 - len(bin(white_queens)[2:])) + bin(white_queens)[2:])[i] == "1":
            board[i] = "wq"

        if ("0" * (64 - len(bin(black_queens)[2:])) + bin(black_queens)[2:])[i] == "1":
            board[i] = "bq"

        if ("0" * (64 - len(bin(white_king)[2:])) + bin(white_king)[2:])[i] == "1":
            board[i] = "wk"

        if ("0" * (64 - len(bin(black_king)[2:])) + bin(black_king)[2:])[i] == "1":
            board[i] = "bk"

    return board


def set_board(white_pieces_tmp, black_pieces_tmp, pawns_tmp, knights_tmp, bishops_tmp, rooks_tmp, queens_tmp, kings_tmp, white_en_passants_tmp, black_en_passants_tmp):
    global white_pieces, black_pieces, pawns, knights, bishops, rooks, queens, kings, white_en_passants, black_en_passants
    white_pieces, black_pieces, pawns, knights, bishops, rooks, queens, kings, white_en_passants, black_en_passants = \
        white_pieces_tmp, black_pieces_tmp, pawns_tmp, knights_tmp, bishops_tmp, rooks_tmp, queens_tmp, kings_tmp, white_en_passants_tmp, black_en_passants_tmp


def get_board():
    global white_pieces, black_pieces, pawns, knights, bishops, rooks, queens, kings, white_en_passants, black_en_passants
    return white_pieces, black_pieces, pawns, knights, bishops, rooks, queens, kings, white_en_passants, black_en_passants


board = fen_parser(input("Input your FEN, (nothing for normal starting position)\n"))

white_pieces = board[0]
black_pieces = board[1]

pawns = board[2]
knights = board[3]
bishops = board[4]
rooks = board[5]
queens = board[6]
kings = board[7]
white_en_passants = 0
black_en_passants = 0