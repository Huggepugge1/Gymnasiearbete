def fen_parser(fen_string):
    # If no starting position is inserted, use normal starting position
    if fen_string == "":
        fen_string = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"
    # All board (white pieces, black pieces, pawns, knights, bishops, rooks, queens, kings)
    boards = [0 for i in range(8)]
    # Remove all "/" from string
    fen_string = "".join(reversed(fen_string.split("/")))
    # Current position on board
    square = 0
    print(fen_string)
    for c in fen_string:
        try:
            c = int(c)
            square += c - 1

        except ValueError:
            if c == "P":
                boards[0] += (1 << square)
                boards[2] += (1 << square)
            if c == "p":
                boards[1] += (1 << square)
                boards[2] += (1 << square)

            if c == "N":
                boards[0] += (1 << square)
                boards[3] += (1 << square)
            if c == "n":
                boards[1] += (1 << square)
                boards[3] += (1 << square)

            if c == "B":
                boards[0] += (1 << square)
                boards[4] += (1 << square)
            if c == "b":
                boards[1] += (1 << square)
                boards[4] += (1 << square)

            if c == "R":
                boards[0] += (1 << square)
                boards[5] += (1 << square)
            if c == "r":
                boards[1] += (1 << square)
                boards[5] += (1 << square)

            if c == "Q":
                boards[0] += (1 << square)
                boards[6] += (1 << square)
            if c == "q":
                boards[1] += (1 << square)
                boards[6] += (1 << square)

            if c == "K":
                boards[0] += (1 << square)
                boards[7] += (1 << square)
            if c == "k":
                boards[1] += (1 << square)
                boards[7] += (1 << square)

        square += 1

    return boards
