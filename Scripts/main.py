from board import print_board, compute_board
from moves import precompute_all_moves, make_move

make_move("e2", "e4", 0)
make_move("e1", "e2", 0)
make_move("e2", "f3", 0)
make_move("f3", "f4", 0)
make_move("f4", "f5", 0)
make_move("f5", "f6", 0)
make_move("f6", "e7", 0)
make_move("e7", "e8", 0)
print_board(compute_board())
