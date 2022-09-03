from board import print_board, compute_board
from moves import precompute_all_moves, make_move

white_pawn_moves = precompute_all_moves("white_pawn")
black_pawn_moves = precompute_all_moves("black_pawn")
knight_moves = precompute_all_moves("knight")
bishop_moves = precompute_all_moves("bishop")
rook_moves = precompute_all_moves("rook")
queen_moves = precompute_all_moves("queen")
king_moves = precompute_all_moves("king")

make_move("e2", "e4")
make_move("d8", "d1")

print_board(compute_board())