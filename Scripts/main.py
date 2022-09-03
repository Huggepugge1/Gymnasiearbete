from board import print_board, compute_board
from moves import precompute_all_moves, make_move

print(make_move("e7", "e5", 1))
print(make_move("e2", "e3", 0))
print(make_move("e5", "e4", 1))
print(make_move("d2", "d4", 0))
print(make_move("e4", "d3", 1))
make_move("c2", "d3", 0)
print_board(compute_board())
