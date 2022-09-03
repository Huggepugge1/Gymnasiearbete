from fen_parser import fen_parser

boards = fen_parser(input("Input your FEN, (nothing for normal starting position)"))

white_pieces = boards[0]
black_pieces = boards[1]

pawns = boards[2]
knights = boards[3]
bishops = boards[4]
rooks = boards[5]
queens = boards[6]
kings = boards[7]