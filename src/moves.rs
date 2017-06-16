use definitions;

pub fn straight(bitboard:u64, empty:u64, opponent:u64) -> Vec<definitions::Move> {
    let mut moves = Vec::new();

    let mut bitboard_copy = bitboard.clone();

    //    Depending on how trailing_zeros is implemented this could be sped up with bitscans
    //    Which require 2s complemented (signed) bitboards
    loop {
        //        Come back to this an implement it correctly
        if bitboard_copy == 0 { break }

        let square:u64 = bitboard_copy.trailing_zeros() as u64;

        for direction in vec![1, 8] {
            let mut i = 0;
//            Moving Left / Down
            loop {
                i += 1;

                if i * direction > 63 { break; }

                let digit:u64 = (1u64 << square) >> (i * direction);

                if digit & definitions::ALL == 0 || (direction == 1 && (digit &  definitions::FILE_H > 0)) {
                    break;
                }

                if empty & digit > 0 {
                    moves.push(definitions::Move::from_num(square as u8, (square - direction * i) as u8));
                }

                else if opponent & digit > 0 {
                    moves.push(definitions::Move::from_num(square as u8, (square - direction * i) as u8));
                    break;
                }

                else {
                    break;
                }

            }

            let mut i = 0;
//            Moving Right / Up
            loop {
                i += 1;

                if i * direction > 63 { break; }

                let digit:u64 = (1u64 << square) << (i * direction);

                if digit & definitions::ALL == 0 || (direction == 1 && (digit &  definitions::FILE_A > 0)) {
                    break;
                }

                if empty & digit > 0 {
                    moves.push(definitions::Move::from_num(square as u8, (square + direction * i) as u8));
                }

                else if opponent & digit > 0 {
                    moves.push(definitions::Move::from_num(square as u8, (square + direction * i) as u8));
                    break;
                }

                else {
                    break;
                }

            }
        }

        bitboard_copy = bitboard_copy - (1u64 << square);
    }

    moves
}

pub fn diagonal(bitboard:u64, empty:u64, opponent:u64) -> Vec<definitions::Move> {
    let mut moves = Vec::new();

    let mut bitboard_copy = bitboard;

    //    Depending on how trailing_zeros is implemented this could be sped up with bitscans
    //    Which require 2s complemented (signed) bitboards
    loop {
        //        Come back to this an implement it correctly
        if bitboard_copy == 0 { break }

        let square = bitboard_copy.trailing_zeros();

        for direction in vec![7, 9] {
            let mut i = 0;
            loop {
                i += 1;
                let digit:u64 = (1u64 << square) << (i * direction);

                if digit & definitions::ALL == 0 || (direction == 9 && (digit &  definitions::FILE_A> 0) || (direction == 7 && (digit &  definitions::FILE_H> 0))) {
                    break;
                }

                if empty & digit > 0 {
                    moves.push(definitions::Move::from_num(square as u8, (square + direction * i) as u8));
                }

                else if opponent & digit > 0 {
                    moves.push(definitions::Move::from_num(square as u8, (square + direction * i) as u8));
                    break;
                }

                else {
                    break;
                }

            }

            let mut i = 0;
            loop {
                i += 1;
                if square < direction * i { break; }

                let digit:u64 = 1u64 << (square - i * direction);


                if digit & definitions::ALL == 0 || (direction == 7 && (digit &  definitions::FILE_A> 0) || (direction == 9 && (digit &  definitions::FILE_H> 0))) {
                    break;
                }


                if empty & digit > 0 {
                    moves.push(definitions::Move::from_num(square as u8, (square - direction * i) as u8));
                }

                else if opponent & digit > 0 {
                    moves.push(definitions::Move::from_num(square as u8, (square - direction * i) as u8));
                    break;
                }

                else {
                    break;
                }

            }
        }

        bitboard_copy = bitboard_copy - (1u64 << square);
    }

    moves
}

// TODO BBS: shift knight bitboard instead of storing all possibilities
pub fn knight(bitboard:u64, empty:u64, opponent:u64) -> Vec<definitions::Move> {
    let mut moves = Vec::new();
    let moveable = empty | opponent;

    let mut bitboard_copy = bitboard;

    loop {
        if bitboard_copy == 0 { break }

        let square = bitboard_copy.trailing_zeros();

        let mut bitboard_moves = moveable & definitions::KNIGHT_BITBOARDS[square as usize];

        loop {
            if bitboard_moves == 0 { break }

            let dest = bitboard_moves.trailing_zeros();

            moves.push(definitions::Move::from_num(square as u8, dest as u8));

            bitboard_moves = bitboard_moves - (1u64 << dest);
        }

        bitboard_copy = bitboard_copy - (1u64 << square);
    }

    moves
}

// TODO BBS: shift king bitboard instead of storing all possibilities
pub fn king(bitboard:u64, empty:u64, opponent:u64) -> Vec<definitions::Move> {
    let mut moves = Vec::new();
    let moveable = empty | opponent;

    let mut bitboard_copy = bitboard;

    loop {
        if bitboard_copy == 0 { break }

        let square = bitboard_copy.trailing_zeros();

        let mut bitboard_moves = moveable & definitions::KING_BITBOARDS[square as usize];

        loop {
            if bitboard_moves == 0 { break }

            let dest = bitboard_moves.trailing_zeros();

            moves.push(definitions::Move::from_num(square as u8, dest as u8));

            bitboard_moves = bitboard_moves - (1u64 << dest);
        }

        bitboard_copy = bitboard_copy - (1u64 << square);
    }

    moves
}
