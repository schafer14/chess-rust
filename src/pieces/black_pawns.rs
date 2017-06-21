use definitions;

pub fn moves(bitboard:u64, empty:u64, opponents:u64, opponents_pawns:u64, last:Option<definitions::Move>) -> Vec<definitions::Move> {
    let mut moves: Vec<definitions::Move> = Vec::new();

    //    Capture to the left
    let cap_right_x = (bitboard >> 7) & opponents & !definitions::FILE_A;

    let end = 64 - cap_right_x.leading_zeros();
    let start = cap_right_x.trailing_zeros();

    for i in start..end {
        let digit = 1u64 << i;

        if digit & cap_right_x & !definitions::RANK_1 > 0{
            moves.push(definitions::Move::from_num((i + 7) as u8, i as u8));
        }

            else if digit & cap_right_x & definitions::RANK_1 > 0 {
                moves.push(definitions::Move::from_num_special((i + 7) as u8, i as u8, 'n'));
                moves.push(definitions::Move::from_num_special((i + 7) as u8, i as u8, 'b'));
                moves.push(definitions::Move::from_num_special((i + 7) as u8, i as u8, 'r'));
                moves.push(definitions::Move::from_num_special((i + 7) as u8, i as u8, 'q'));
            }
    }


    //    Capture to the right
    let cap_left = (bitboard >> 9) & opponents & !definitions::FILE_H;

    let end = 64 - cap_left.leading_zeros();
    let start = cap_left.trailing_zeros();

    for i in start..end {
        let digit = 1u64 << i;

        if digit & cap_left & !definitions::RANK_1 > 0 {
            moves.push(definitions::Move::from_num((i + 9) as u8, i as u8));
        }
            else if digit & cap_left & definitions::RANK_1 > 0 {
                moves.push(definitions::Move::from_num_special((i + 9) as u8, i as u8, 'n'));
                moves.push(definitions::Move::from_num_special((i + 9) as u8, i as u8, 'b'));
                moves.push(definitions::Move::from_num_special((i + 9) as u8, i as u8, 'r'));
                moves.push(definitions::Move::from_num_special((i + 9) as u8, i as u8, 'q'));
            }
    }

    //    Move forward 1
    let forward_1 = (bitboard >> 8) & empty;
    let end = 64 - forward_1.leading_zeros();
    let start = forward_1.trailing_zeros();

    for i in start..end {
        let digit = 1u64 << i;

        if digit & forward_1 & !definitions::RANK_1 > 0 {
            moves.push(definitions::Move::from_num((i + 8) as u8, i as u8));
        } else if digit & forward_1 & definitions::RANK_1 > 0 {
            moves.push(definitions::Move::from_num_special((i + 8) as u8, i as u8, 'n'));
            moves.push(definitions::Move::from_num_special((i + 8) as u8, i as u8, 'b'));
            moves.push(definitions::Move::from_num_special((i + 8) as u8, i as u8, 'r'));
            moves.push(definitions::Move::from_num_special((i + 8) as u8, i as u8, 'q'));
        }
    }

    //    Move forward 2
    let forward_2 = (bitboard >> 16) & empty & (empty >> 8) & definitions::RANK_5;
    let end = 64 - forward_2.leading_zeros();
    let start = forward_2.trailing_zeros();

    for i in start..end {
        let digit = 1u64 << i;

        if digit & forward_2 > 0 {
            let m = definitions::Move::from_num((i + 16) as u8, i as u8);
            moves.push(m);
        }
    }

    //    EN PASSANT
    if last.is_some(){
        let last_move = last.unwrap();

        let last_move_to_square = 1u64 << last_move.to.number;

        if last_move.to.number > 16 && last_move.from.number == last_move.to.number - 16 &&
            last_move_to_square & opponents_pawns > 0 {
            let dest = last_move.from.number + 8;

            let left = 1u64 << (last_move.to.number + 1);
            let right = 1u64 << (last_move.to.number - 1);

            if right & bitboard > 0 && !(definitions::FILE_A & last_move_to_square > 0) {
                moves.push(definitions::Move::from_num_special(last_move.to.number - 1, dest, 'E'))
            }

            if left & bitboard > 0 && !(definitions::FILE_H & last_move_to_square > 0) {
                moves.push(definitions::Move::from_num_special(last_move.to.number + 1, dest, 'E'))
            }
        }
    }

    moves
}