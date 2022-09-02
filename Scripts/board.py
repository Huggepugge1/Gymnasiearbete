def printBoard(board):
    print("  -------------------------------")
    for i in range(8):
        for j in range(8):
            print(" | " + str(board[i][j]), end="")
        print(" | ")
        print("  -------------------------------")


board = []

for i in range(8):
    board.append([])
    for j in range(8):
        board[i].append(0)