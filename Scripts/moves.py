from board import set_board, get_board


def check_type(start_mask, end_mask):
    white_pieces, black_pieces, pawns, knights, bishops, rooks, queens, kings, white_en_passants, black_en_passants = get_board()
    if start_mask & white_pieces > 0:
        color = 0
    elif start_mask & black_pieces > 0:
        color = 1
    else:
        color = -1
    if start_mask & pawns > 0:
        start_piece = 0
    elif start_mask & knights > 0:
        start_piece = 1
    elif start_mask & bishops > 0:
        start_piece = 2
    elif start_mask & rooks > 0:
        start_piece = 3
    elif start_mask & queens > 0:
        start_piece = 4
    elif start_mask & kings > 0:
        start_piece = 5
    else:
        start_piece = -1

    if end_mask & pawns > 0:
        end_piece = 0
    elif end_mask & knights > 0:
        end_piece = 1
    elif end_mask & bishops > 0:
        end_piece = 2
    elif end_mask & rooks > 0:
        end_piece = 3
    elif end_mask & queens > 0:
        end_piece = 4
    elif end_mask & kings > 0:
        end_piece = 5
    else:
        end_piece = -1
    return start_piece, end_piece, color


def get_start(start, end):
    # Converts chess format to a number between 0 and 64
    start_row = int(start[1]) - 1
    start_rank = (ord(start[0]) - 97)
    start_pos = (start_row * 8) + start_rank
    start_mask = 1 << start_pos

    end_row = int(end[1]) - 1
    end_rank = (ord(end[0]) - 97)
    end_pos = (end_row * 8) + end_rank
    end_mask = 1 << end_pos
    return start_pos, start_mask, end_pos, end_mask

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
                if 64 >= (current_square + offset) >= 0:
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
                if 64 >= (current_square + offset) >= 0:
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
                if 64 >= (current_square + offset) >= 0:
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
                if 64 >= (current_square + offset) >= 0:
                    horizontal_difference = abs((current_square % 8) - ((current_square + offset) % 8))
                    vertical_difference = abs((current_square // 8) - ((current_square + offset) // 8))
                    if horizontal_difference == 0 or vertical_difference == 0:
                        current_move |= 1 << (current_square + offset)

            for offset in queen_diagonal_offsets:
                if 64 >= (current_square + offset) >= 0:
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
                if 64 >= (current_square + offset) >= 0:
                    horizontal_difference = abs((current_square % 8) - ((current_square + offset) % 8))
                    vertical_difference = abs((current_square // 8) - ((current_square + offset) // 8))
                    if horizontal_difference == 0 or vertical_difference == 0:
                        current_move |= 1 << (current_square + offset)

            for offset in king_diagonal_offsets:
                if 64 >= (current_square + offset) >= 0:
                    horizontal_difference = abs((current_square % 8) - ((current_square + offset) % 8))
                    vertical_difference = abs((current_square // 8) - ((current_square + offset) // 8))
                    if horizontal_difference == vertical_difference:
                        current_move |= 1 << (current_square + offset)

            moves.append(current_move)

    return moves


def make_move(start, end, turn):
    if not validate_move(start, end, turn):
        return False

    white_pieces, black_pieces, pawns, knights, bishops, rooks, queens, kings, white_en_passants, black_en_passants = get_board()

    start_pos, start_mask, end_pos, end_mask = get_start(start, end)

    if start_mask & white_pieces > 0:
        white_pieces &= ((1 << 64) - 1) - start_mask
        white_pieces |= end_mask
        white_en_passants = 0
        if black_pieces & end_mask > 0:
            black_pieces -= end_mask

    elif start_mask & black_pieces > 0:
        black_pieces &= ((1 << 64) - 1) - start_mask
        black_pieces |= end_mask
        black_en_passants = 0
        if white_pieces & end_mask > 0:
            white_pieces -= end_mask
    else:
        return False

    start_piece, end_piece, color = check_type(start_mask, end_mask)

    if start_piece == 0:
        pawns &= ((1 << 64) - 1) - start_mask
        pawns |= end_mask

        if color == 0:
            if black_en_passants & (1 << end_pos) > 0:
                pawns &= ((1 << 64) - 1) - (1 << (end_pos - 8))
        if color == 1:
            if white_en_passants & (1 << end_pos) > 0:
                pawns &= ((1 << 64) - 1) - (1 << (end_pos + 8))

        if abs(start_pos - end_pos) == 16 and color == 0:
            white_en_passants |= 1 << (start_pos + 8)
        if abs(start_pos - end_pos) == 16 and color == 1:
            black_en_passants |= 1 << (start_pos - 8)

        if end_piece == 1:
            knights &= ((1 << 64) - 1) - end_mask
        elif end_piece == 2:
            bishops &= ((1 << 64) - 1) - end_mask
        elif end_piece == 3:
            rooks &= ((1 << 64) - 1) - end_mask
        elif end_piece == 4:
            queens &= ((1 << 64) - 1) - end_mask
        elif end_piece == 5:
            kings &= ((1 << 64) - 1) - end_mask

    if start_piece == 1:
        knights &= ((1 << 64) - 1) - start_mask
        knights |= end_mask

        if end_piece == 0:
            pawns &= ((1 << 64) - 1) - end_mask
        elif end_piece == 2:
            bishops &= ((1 << 64) - 1) - end_mask
        elif end_piece == 3:
            rooks &= ((1 << 64) - 1) - end_mask
        elif end_piece == 4:
            queens &= ((1 << 64) - 1) - end_mask
        elif end_piece == 5:
            kings &= ((1 << 64) - 1) - end_mask

    if start_piece == 2:
        bishops &= ((1 << 64) - 1) - start_mask
        bishops |= end_mask

        if end_piece == 0:
            pawns &= ((1 << 64) - 1) - end_mask
        elif end_piece == 1:
            knights &= ((1 << 64) - 1) - end_mask
        elif end_piece == 3:
            rooks &= ((1 << 64) - 1) - end_mask
        elif end_piece == 4:
            queens &= ((1 << 64) - 1) - end_mask
        elif end_piece == 5:
            kings &= ((1 << 64) - 1) - end_mask

    if start_piece == 3:
        rooks &= ((1 << 64) - 1) - start_mask
        rooks |= end_mask

        if end_piece == 0:
            pawns &= ((1 << 64) - 1) - end_mask
        elif end_piece == 1:
            knights &= ((1 << 64) - 1) - end_mask
        elif end_piece == 2:
            bishops &= ((1 << 64) - 1) - end_mask
        elif end_piece == 4:
            queens &= ((1 << 64) - 1) - end_mask
        elif end_piece == 5:
            kings &= ((1 << 64) - 1) - end_mask

    if start_piece == 4:
        queens &= ((1 << 64) - 1) - start_mask
        queens |= end_mask

        if end_piece == 0:
            pawns &= ((1 << 64) - 1) - end_mask
        elif end_piece == 1:
            knights &= ((1 << 64) - 1) - end_mask
        elif end_piece == 2:
            bishops &= ((1 << 64) - 1) - end_mask
        elif end_piece == 3:
            rooks &= ((1 << 64) - 1) - end_mask
        elif end_piece == 5:
            kings &= ((1 << 64) - 1) - end_mask

    if start_piece == 5:
        kings &= ((1 << 64) - 1) - start_mask
        kings |= end_mask

        if end_piece == 0:
            pawns &= ((1 << 64) - 1) - end_mask
        elif end_piece == 1:
            knights &= ((1 << 64) - 1) - end_mask
        elif end_piece == 2:
            bishops &= ((1 << 64) - 1) - end_mask
        elif end_piece == 3:
            rooks &= ((1 << 64) - 1) - end_mask
        elif end_piece == 4:
            queens &= ((1 << 64) - 1) - end_mask

    set_board(white_pieces, black_pieces, pawns, knights, bishops, rooks, queens, kings, white_en_passants, black_en_passants)
    return True


def validate_capture(start_pos, end_pos, start_mask, end_mask, start_piece, end_piece, color):
    white_pieces, black_pieces, pawns, knights, bishops, rooks, queens, kings, white_en_passants, black_en_passants = get_board()
    move = 0
    if start_piece == 0:
        if color == 0:
            white_pawn_offsets = [7, 9]
            for offset in white_pawn_offsets:
                if start_pos + offset < 64 and abs((start_pos % 8) - ((start_pos + offset) % 8)) == 1:
                    move |= 1 << (start_pos + offset)

            white_en_passants_offsets = [7, 9]
            for offset in white_en_passants_offsets:
                if start_pos + offset < 64 and abs((start_pos % 8) - ((start_pos + offset) % 8)) == 1:
                    if (1 << (start_pos + offset)) & black_en_passants > 0:
                        move |= 1 << (start_pos + offset)

        if color == 1:
            move = 0
            black_pawn_offsets = [-9, -7]
            for offset in black_pawn_offsets:
                if start_pos + offset >= 0 and abs((start_pos % 8) - ((start_pos + offset) % 8)) == 1:
                    move |= 1 << (start_pos + offset)

            black_en_passants_offsets = [-9, -7]
            for offset in black_en_passants_offsets:
                if start_pos + offset >= 0 and abs((start_pos % 8) - ((start_pos + offset) % 8)) == 1:
                    if (1 << (start_pos + offset)) & white_en_passants > 0:
                        move |= 1 << (start_pos + offset)
    else:
        assert False, "Not implemented"

    if move & end_mask > 0:
        return True
    else:
        return False


def validate_move(start, end, turn):
    white_pieces, black_pieces, pawns, knights, bishops, rooks, queens, kings, white_en_passants, black_en_passants = get_board()
    # Converts chess format to a number between 0 and 64
    start_pos, start_mask, end_pos, end_mask = get_start(start, end)
    start_piece, end_piece, color = check_type(start_mask, end_mask)

    if not (color == 0) and turn == 0:
        return False
    elif not (color == 1) and turn == 1:
        return False
    elif color == -1:
        return False

    if color == 0 and black_pieces & end_mask > 0:
        return validate_capture(start_pos, end_pos, start_mask, end_mask, start_piece, end_piece, color)
    elif color == 1 and white_pieces & end_mask > 0:
        return validate_capture(start_pos, end_pos, start_mask, end_mask, start_piece, end_piece, color)

    if start_piece == 0:
        if color == 0:
            if end_mask & black_en_passants > 0:
                return validate_capture(start_pos, end_pos, start_mask, end_mask, start_piece, end_piece, color)
            elif white_pawn_moves[start_pos] & end_mask > 0 and white_pawn_moves[start_pos] & black_pieces == 0:
                return True
        elif color == 1:
            if end_mask & white_en_passants > 0:
                print("test")
                return validate_capture(start_pos, end_pos, start_mask, end_mask, start_piece, end_piece, color)
            elif black_pawn_moves[start_pos] & end_mask > 0 and black_pawn_moves[start_pos] & white_pieces == 0:
                return True
        else:
            return False
    else:
        assert False, "Not implemented"


white_pawn_moves = precompute_all_moves("white_pawn")
black_pawn_moves = precompute_all_moves("black_pawn")
knight_moves = precompute_all_moves("knight")
bishop_moves = precompute_all_moves("bishop")
rook_moves = precompute_all_moves("rook")
queen_moves = precompute_all_moves("queen")
king_moves = precompute_all_moves("king")
