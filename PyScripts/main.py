from board import print_board, compute_board
from moves import precompute_all_moves, make_move
from moves import queen_moves

print(len(bin(queen_moves[32])[2:]))

print_board("0000" + bin(queen_moves[32])[2:])
