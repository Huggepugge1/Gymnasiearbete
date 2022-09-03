def printBoard(board):
    print("  -------------------------------")
    for i in range(8):
        for j in range(8):
            print(" | " + str(board[i][j]), end="")
        print(" | ")
        print("  -------------------------------")


def precompute_all_moves(piece):
    moves = []
    if piece == "white_pawn":
        white_pawn_offsets = [8, 16]
        for i in range(64):
            if 7 < i < 16:
                moves.append(((1 << i) << white_pawn_offsets[0]) + ((1 << i) << white_pawn_offsets[1]))
            elif i < 56:
                moves.append(((1 << i) << white_pawn_offsets[0]))
            else:
                moves.append(0)
                
    if piece == "knight":
        knight_offsets = [-17, -15, -10, -6, 6, 10, 15, 17]
        for i in range(64):
            current_move = 0
            for offset in knight_offsets:
                if 63 >= (i + offset) >= 0:
                    if abs((i % 8) - (i + offset) % 8) <= 2 and abs(i // 8 - (i + offset) // 8) <= 2:
                        current_move |= 1 << (i + offset)
            
            moves.append(current_move)
    return moves


white_pieces = (1 << 16) -1
black_pieces = ((1 << 16) - 1) << 48

pawns = (((1 << 8) - 1) << 8) + (((1 << 8) - 1) << 48)

white_pawn_moves = precompute_all_moves("white_pawn")
knight_moves = precompute_all_moves("knight")

for i in range(64):
    print(bin(knight_moves[i]), i)

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