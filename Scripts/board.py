from boards import white_pieces, black_pieces, pawns, knights, bishops, rooks, queens, kings


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


def precompute_all_moves(piece):
    moves = []
    if piece == "white_pawn":
        white_pawn_offsets = [8, 16]
        for current_square in range(64):
            if 7 < current_square < 16:
                moves.append(((1 << current_square) << white_pawn_offsets[0]) + ((1 << current_square) << white_pawn_offsets[1]))
            elif current_square < 56:
                moves.append(((1 << current_square) << white_pawn_offsets[0]))
            else:
                moves.append(0)

    if piece == "black_pawn":
        black_pawn_offsets = [-8, -16]
        for current_square in range(64):
            if 47 < current_square < 56:
                moves.append((1 << (current_square + black_pawn_offsets[0])) + (1 << (current_square + black_pawn_offsets[1])))
            elif current_square > 7:
                moves.append(1 << (current_square + black_pawn_offsets[0]))
            else:
                moves.append(0)

    if piece == "knight":
        knight_offsets = [-17, -15, -10, -6, 6, 10, 15, 17]
        for current_square in range(64):
            current_move = 0
            for offset in knight_offsets:
                if 63 >= (current_square + offset) >= 0:
                    if abs((current_square % 8) - (current_square + offset) % 8) <= 2 and abs(current_square // 8 - (current_square + offset) // 8) <= 2:
                        current_move |= 1 << (current_square + offset)

            moves.append(current_move)

    if piece == "bishop":
        bishop_offsets = []
        for offset in range(1, 8):
            bishop_offsets.append(offset * 7)
            bishop_offsets.append(-offset * 7)

            bishop_offsets.append(offset * 9)
            bishop_offsets.append(-offset * 9)
        for current_square in range(64):
            current_move = 0
            for offset in bishop_offsets:
                if 63 >= (current_square + offset) >= 0:
                    horizontal_difference = abs((current_square % 8) - ((current_square + offset) % 8))
                    vertical_difference = abs((current_square // 8) - ((current_square + offset) // 8))
                    if horizontal_difference == vertical_difference:
                        current_move |= 1 << (current_square + offset)

            moves.append(current_move)

    if piece == "rook":
        rook_offsets = []
        for offset in range(1, 8):
            rook_offsets.append(offset * 8)
            rook_offsets.append(-offset * 8)

            rook_offsets.append(offset)
            rook_offsets.append(-offset)

        for current_square in range(64):
            current_move = 0
            for offset in rook_offsets:
                if 63 >= (current_square + offset) >= 0:
                    horizontal_difference = abs((current_square % 8) - ((current_square + offset) % 8))
                    vertical_difference = abs((current_square // 8) - ((current_square + offset) // 8))
                    if horizontal_difference == vertical_difference:
                        current_move |= 1 << (current_square + offset)

            moves.append(current_move)

    if piece == "queen":
        queen_diagonal_offsets = []
        queen_straight_offsets = []
        for offset in range(1, 8):
            queen_straight_offsets.append(offset)
            queen_straight_offsets.append(-offset)

            queen_straight_offsets.append(offset * 8)
            queen_straight_offsets.append(-offset * 8)

            queen_diagonal_offsets.append(offset*7)
            queen_diagonal_offsets.append(-offset*7)

            queen_diagonal_offsets.append(offset * 9)
            queen_diagonal_offsets.append(-offset * 9)

        for current_square in range(64):
            current_move = 0

            for offset in queen_straight_offsets:
                if 63 >= (current_square + offset) >= 0:
                    horizontal_difference = abs((current_square % 8) - ((current_square + offset) % 8))
                    vertical_difference = abs((current_square // 8) - ((current_square + offset) // 8))
                    if horizontal_difference == 0 or vertical_difference == 0:
                        current_move |= 1 << (current_square + offset)

            for offset in queen_diagonal_offsets:
                if 63 >= (current_square + offset) >= 0:
                    horizontal_difference = abs((current_square % 8) - ((current_square + offset) % 8))
                    vertical_difference = abs((current_square // 8) - ((current_square + offset) // 8))
                    if horizontal_difference == vertical_difference:
                        current_move |= 1 << (current_square + offset)

            moves.append(current_move)

    if piece == "king":
        king_diagonal_offsets = [-9, -7, 7, 9]
        king_straight_offsets = [-8, -1, 1, 8]

        for current_square in range(64):
            current_move = 0

            for offset in king_straight_offsets:
                if 63 >= (current_square + offset) >= 0:
                    horizontal_difference = abs((current_square % 8) - ((current_square + offset) % 8))
                    vertical_difference = abs((current_square // 8) - ((current_square + offset) // 8))
                    if horizontal_difference == 0 or vertical_difference == 0:
                        current_move |= 1 << (current_square + offset)

            for offset in king_diagonal_offsets:
                if 63 >= (current_square + offset) >= 0:
                    horizontal_difference = abs((current_square % 8) - ((current_square + offset) % 8))
                    vertical_difference = abs((current_square // 8) - ((current_square + offset) // 8))
                    if horizontal_difference == vertical_difference:
                        current_move |= 1 << (current_square + offset)

            moves.append(current_move)

    return moves


# pieces
'''
white pawn = 0b1000
black pawn = 0b0000

knight = 0b1001

bishop = 0b1010

rook = 0b1011

queen = 0b1100

king = 0b1101
'''