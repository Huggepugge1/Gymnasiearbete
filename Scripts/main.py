from board import print_board, compute_board, precompute_all_moves

white_pawn_moves = precompute_all_moves("white_pawn")
black_pawn_moves = precompute_all_moves("black_pawn")
knight_moves = precompute_all_moves("knight")
bishop_moves = precompute_all_moves("bishop")
rook_moves = precompute_all_moves("rook")
queen_moves = precompute_all_moves("queen")
king_moves = precompute_all_moves("king")

print_board(compute_board())