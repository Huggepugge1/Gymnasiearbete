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
