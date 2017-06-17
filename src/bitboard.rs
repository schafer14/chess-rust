use pieces::white_pawns;
use pieces::black_pawns;
use moves;
use definitions;

use std::fmt;
use std::process;

#[derive(Clone)]
pub struct BitBoard {
    pub bp: u64,
    pub wp: u64,
    pub bn: u64,
    pub wn: u64,
    pub bb: u64,
    pub wb: u64,
    pub br: u64,
    pub wr: u64,
    pub bq: u64,
    pub wq: u64,
    pub bk: u64,
    pub wk: u64,
    castling: [bool; 4],
    pub turn: bool,
    history: Vec<definitions::Move>
}

impl fmt::Display for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..8 {

            write!(f, "{} ", 8 - row)?;

            for col in 0..8 {
                let square:u32 = ((7 - row) * 8 + col) as u32;
                let digit = 2u64.pow(square);

                if digit & self.bp > 0 {
                    write!(f, "♙ ")?;
                } else if digit & self.wp > 0 {
                    write!(f, "♟ ")?;
                } else if digit & self.bn > 0 {
                    write!(f, "♘ ")?;
                } else if digit & self.wn > 0 {
                    write!(f, "♞ ")?;
                } else if digit & self.bb > 0 {
                    write!(f, "♗ ")?;
                } else if digit & self.wb > 0 {
                    write!(f, "♝ ")?;
                } else if digit & self.br > 0 {
                    write!(f, "♖ ")?;
                } else if digit & self.wr > 0 {
                    write!(f, "♜ ")?;
                } else if digit & self.bq > 0 {
                    write!(f, "♕ ")?;
                } else if digit & self.wq > 0 {
                    write!(f, "♛ ")?;
                } else if digit & self.bk > 0 {
                    write!(f, "♔ ")?;
                } else if digit & self.wk > 0 {
                    write!(f, "♚ ")?;
                } else {
                    write!(f, "• ")?;
                }

            }
            write!(f, "\n")?
        }

        if self.turn { write!(f, "•") } else { write!(f, "o") };
        write!(f, " a b c d e f g h\n")
    }
}

impl BitBoard {
    pub fn new(init_board:[[char; 8]; 8]) -> BitBoard {
        let mut bitboard = BitBoard { bp: 0, wp: 0, bn: 0, wn: 0, bb: 0, wb: 0, br: 0, wr: 0, bq: 0,
            wq: 0, bk: 0, wk: 0, turn: true, castling: [true, true, true, true], history: Vec::new() };

        for row in 0..8 {
            for col in 0..8 {
                let square:u32 = ((7 - row) * 8 + col) as u32;
                let digit = 2u64.pow(square);

                if 'p' == init_board[row][col] {
                    bitboard.bp +=  digit
                };

                if 'P' == init_board[row][col] {
                    bitboard.wp +=  digit
                };

                if 'n' == init_board[row][col] {
                    bitboard.bn +=  digit
                };

                if 'N' == init_board[row][col] {
                    bitboard.wn +=  digit
                };

                if 'b' == init_board[row][col] {
                    bitboard.bb +=  digit
                };

                if 'B' == init_board[row][col] {
                    bitboard.wb +=  digit
                };

                if 'r' == init_board[row][col] {
                    bitboard.br +=  digit
                };

                if 'R' == init_board[row][col] {
                    bitboard.wr +=  digit
                };

                if 'q' == init_board[row][col] {
                    bitboard.bq +=  digit
                };

                if 'Q' == init_board[row][col] {
                    bitboard.wq +=  digit
                };

                if 'k' == init_board[row][col] {
                    bitboard.bk +=  digit
                };

                if 'K' == init_board[row][col] {
                    bitboard.wk +=  digit
                };
            }

        }

        bitboard


    }

    pub fn from_fen(fen:String) -> BitBoard {
        let parts = fen.trim().split(" ").collect::<Vec<&str>>();

        let mut board_array = [[' '; 8]; 8];
        let mut i = 0; let mut j = 0;
        for c in parts[0].chars() {
            match c {
                '/' => { i += 1; j = 0; },
                'p' => { board_array[i][j] = 'p'; j += 1; },
                'P' => { board_array[i][j] = 'P'; j += 1; },
                'n' => { board_array[i][j] = 'n'; j += 1; },
                'N' => { board_array[i][j] = 'N'; j += 1; },
                'b' => { board_array[i][j] = 'b'; j += 1; },
                'B' => { board_array[i][j] = 'B'; j += 1; },
                'r' => { board_array[i][j] = 'r'; j += 1; },
                'R' => { board_array[i][j] = 'R'; j += 1; },
                'q' => { board_array[i][j] = 'q'; j += 1; },
                'Q' => { board_array[i][j] = 'Q'; j += 1; },
                'k' => { board_array[i][j] = 'k'; j += 1; },
                'K' => { board_array[i][j] = 'K'; j += 1; },
                _ => { let skip = c.to_digit(10).unwrap() as usize; j += skip; }
            }
        }

        let mut bb = BitBoard::new(board_array);
        bb.castling = [false, false, false, false];

        if parts.len() > 1 && parts[1] == "b" { bb.turn = false }

        if parts.len() > 2 {
            if parts[2].contains("K") { bb.castling[0] = true }
            if parts[2].contains("Q") { bb.castling[1] = true }
            if parts[2].contains("k") { bb.castling[2] = true }
            if parts[2].contains("q") { bb.castling[3] = true }
        }

        bb
    }
}

impl BitBoard {
//    TODO BBS: En Passant bitboard
//    This allows for moving into check but provides much faster moves
    pub fn moves(&self) -> Vec<definitions::Move> {
        let turn = self.turn;

        let empty = !(self.bp | self.bn | self.bb | self.br | self.bq | self.bk |
                      self.wp | self.wn | self.wb | self.wr | self.wq | self.wk);

        let mut moves_list = Vec::new();

        if turn {
            let opponents = self.bp | self.bn | self.bb | self.br | self.bq | self.bk;

            moves_list.append(&mut white_pawns::moves(self.wp, empty, opponents, self.bp, self.history.clone()));
            moves_list.append(&mut moves::straight(self.wr, empty, opponents));
            moves_list.append(&mut moves::straight(self.wq, empty, opponents));
            moves_list.append(&mut moves::diagonal(self.wb, empty, opponents));
            moves_list.append(&mut moves::diagonal(self.wq, empty, opponents));
            moves_list.append(&mut moves::knight(self.wn, empty, opponents));
            moves_list.append(&mut moves::king(self.wk, empty, opponents));
            moves_list.append(&mut self.castling_white(empty));
//            Blacks turn
        } else {
            let opponents = self.wp | self.wn | self.wb | self.wr | self.wq | self.wk;

            moves_list.append(&mut black_pawns::moves(self.bp, empty, opponents, self.wp, self.history.clone()));
            moves_list.append(&mut moves::straight(self.br, empty, opponents));
            moves_list.append(&mut moves::straight(self.bq, empty, opponents));
            moves_list.append(&mut moves::diagonal(self.bb, empty, opponents));
            moves_list.append(&mut moves::diagonal(self.bq, empty, opponents));
            moves_list.append(&mut moves::knight(self.bn, empty, opponents));
            moves_list.append(&mut moves::king(self.bk, empty, opponents));
            moves_list.append(&mut self.castling_black(empty));
        }


        moves_list
    }

//    This will remove the ability to move into check.
    pub fn moves_accurate(&self) -> Vec<definitions::Move> {
        let mut moves = self.moves().clone();
        let mut accurate_moves = Vec::new();
        let mut bad_moves = Vec::new();

        for moove in moves.clone() {
            let mut child = self.clone();
            child.make_move(moove.clone());

            let king = if child.turn { child.bk } else { child.wk };
            let king_square = king.trailing_zeros() as u8;

            for opp_move in child.moves() {
                if opp_move.to.number == king_square {
                    bad_moves.push(moove.clone());
                    break;
                }
            }
        }

        for moove in moves.clone() {
            if !bad_moves.contains(&moove) {
                accurate_moves.push(moove);
            }
        }

        accurate_moves
    }

    pub fn castling_white(&self, empty:u64) -> Vec<definitions::Move> {
        let mut moves = Vec::new();

        let could_castle_right:bool = (!definitions::CASTLING_BITBOARD[0] | empty) == definitions::ALL && self.castling[0]
            && (self.wr & 1u64 << 7 > 0);

        let could_castle_left:bool = (!definitions::CASTLING_BITBOARD[1] | empty) == definitions::ALL && self.castling[1]
            && (self.wr & 1 > 0);

        //    Castle right
//        Only do costly operations once
        if  could_castle_left || could_castle_right {
            //        Check attack span
            let opponents = self.wp | self.wn | self.wb | self.wr | self.wq | self.wk;
            let mut opponent_attacks: u64 = 0;
            let protected_squares_right: u64 = 0b1110000;
            let protected_squares_left: u64 = 0b11100;

            //            TODO BBS: Check one bitboard at a time starting with most powerful pieces
            let mut moves_list = &mut moves::straight(self.br, empty, opponents);
            moves_list.append(&mut moves::straight(self.bq, empty, opponents));
            moves_list.append(&mut moves::diagonal(self.bb, empty, opponents));
            moves_list.append(&mut moves::diagonal(self.bq, empty, opponents));
            moves_list.append(&mut moves::knight(self.bn, empty, opponents));
            moves_list.append(&mut moves::king(self.bk, empty, opponents));

            for m in moves_list {
                opponent_attacks = opponent_attacks | 1u64 << m.to.number;
            };

            opponent_attacks = opponent_attacks | self.bp >> 7 | self.bp >> 9;

            if protected_squares_right & opponent_attacks == 0 && could_castle_right {
                moves.push(definitions::Move::from_num_special(4, 6, 'o'));
            }

            if protected_squares_left & opponent_attacks == 0 && could_castle_left {
                moves.push(definitions::Move::from_num_special(4, 2, 'O'));
            }
        }

        moves
    }

    pub fn castling_black(&self, empty:u64) -> Vec<definitions::Move> {
        let mut moves = Vec::new();

        let could_castle_right:bool = (!definitions::CASTLING_BITBOARD[2] | empty) == definitions::ALL && self.castling[2]
            && (self.br & 1u64 << 63 > 0);

        let could_castle_left:bool = (!definitions::CASTLING_BITBOARD[3] | empty) == definitions::ALL  && self.castling[3]
            && (self.br & 1u64 << 56 > 0);

        //    Castle right
        //        Only do costly operations once
        if  could_castle_left || could_castle_right {
            //        Check attack span
            let opponents = self.bp | self.bn | self.bb | self.br | self.bq | self.bk;
            let mut opponent_attacks: u64 = 0;
            let protected_squares_right: u64 = 0b0111000000000000000000000000000000000000000000000000000000000000;
            let protected_squares_left: u64 = 0b0001110000000000000000000000000000000000000000000000000000000000;

            //            TODO BBS: Check one bitboard at a time starting with most powerful pieces
            let mut moves_list = moves::straight(self.wr, empty, opponents);
            moves_list.append(&mut moves::straight(self.wq, empty, opponents));
            moves_list.append(&mut moves::diagonal(self.wb, empty, opponents));
            moves_list.append(&mut moves::diagonal(self.wq, empty, opponents));
            moves_list.append(&mut moves::knight(self.wn, empty, opponents));
            moves_list.append(&mut moves::king(self.wk, empty, opponents));

            for m in moves_list {
                opponent_attacks = opponent_attacks | 1u64 << m.to.number;
            };

            opponent_attacks = opponent_attacks | self.wp << 7 | self.wp << 9;

            if protected_squares_right & opponent_attacks == 0 && could_castle_right {
                moves.push(definitions::Move::from_num_special(60, 62, 'o'));
            }

            if protected_squares_left & opponent_attacks == 0 && could_castle_left {
                moves.push(definitions::Move::from_num_special(60, 58, 'O'));
            }
        }

        moves
    }

    pub fn make_move(&mut self, moove:definitions::Move) {
        if moove.special.is_some() {
            match moove.special {
                Some('o') => {
                    if self.turn {
                        self.wk = self.wk - (1u64 << 4);
                        self.wk = self.wk + (1u64 << 6);
                        self.wr = self.wr - (1u64 << 7);
                        self.wr = self.wr + (1u64 << 5);
                        self.castling[0] = false;
                        self.castling[1] = false;
                    } else {
                        self.bk = self.bk - (1u64 << 60);
                        self.bk = self.bk + (1u64 << 62);
                        self.br = self.br - (1u64 << 63);
                        self.br = self.br + (1u64 << 61);
                        self.castling[2] = false;
                        self.castling[3] = false;
                    };
                    self.history.push(moove.clone());
                    self.turn = if self.turn { false } else { true };
                    return;
                },
                Some('O') => {
                    if self.turn {
                        self.wk = self.wk - (1u64 << 4);
                        self.wk = self.wk + (1u64 << 2);
                        self.wr = self.wr - (1u64 << 0);
                        self.wr = self.wr + (1u64 << 3);
                        self.castling[0] = false;
                        self.castling[1] = false;
                    } else {
                        self.bk = self.bk - (1u64 << 60);
                        self.bk = self.bk + (1u64 << 58);
                        self.br = self.br - (1u64 << 56);
                        self.br = self.br + (1u64 << 59);
                        self.castling[2] = false;
                        self.castling[3] = false;
                    }
                    self.history.push(moove.clone());
                    self.turn = if self.turn { false } else { true };
                    return;
                },
                Some('E') => {
                    if self.turn {
                        let digit = 1u64 << moove.to.number;
                        let taken = digit >> 8;
                        self.bp = self.bp - taken;
                    } else {
                        let digit = 1u64 << moove.to.number;
                        let taken = digit << 8;
                        self.wp = self.wp - taken;
                    };
                },
                _ => {}
            }
        }

        let from = 1u64 << moove.from.number;
        let to = 1u64 << moove.to.number;

        if self.wk & to > 0 {
            self.wk = self.wk - to;
        }
        if self.bk & to > 0 {
            self.bk = self.bk - to;
        }
        if self.wq & to > 0 {
            self.wq = self.wq - to;
        }
        if self.bq & to > 0 {
            self.bq = self.bq - to;
        }
        if self.wr & to > 0 {
            self.wr = self.wr - to;
        }
        if self.br & to > 0 {
            self.br = self.br - to;
        }
        if self.wb & to > 0 {
            self.wb = self.wb - to;
        }
        if self.bb & to > 0 {
            self.bb = self.bb - to;
        }
        if self.wn & to > 0 {
            self.wn = self.wn - to;
        }
        if self.bn & to > 0 {
            self.bn = self.bn - to;
        }
        if self.wp & to > 0 {
            self.wp = self.wp - to;
        }
        if self.bp & to > 0 {
            self.bp = self.bp - to;
        }


        if self.wk & from > 0 {
            self.wk = self.wk - from;
            self.wk = self.wk + to;
            self.castling[0] = false;
            self.castling[1] = false;
        }
        if self.bk & from > 0 {
            self.bk = self.bk - from;
            self.bk = self.bk + to;
            self.castling[2] = false;
            self.castling[3] = false;
        }
        if self.wq & from > 0 {
            self.wq = self.wq - from;
            self.wq = self.wq + to;
        }
        if self.bq & from > 0 {
            self.bq = self.bq - from;
            self.bq = self.bq + to;
        }
        if self.wr & from > 0 {
            self.wr = self.wr - from;
            self.wr = self.wr + to;
            if from == (1u64 << 7) {
                self.castling[0] = false;
            }
            if from == 1 {
                self.castling[1] = false;
            }
        }
        if self.br & from > 0 {
            self.br = self.br - from;
            self.br = self.br + to;
            if from == (1u64 << 63) {
                self.castling[2] = false;
            }
            if from == (1u64 << 56) {
                self.castling[3] = false;
            }
        }
        if self.wb & from > 0 {
            self.wb = self.wb - from;
            self.wb = self.wb + to;
        }
        if self.bb & from > 0 {
            self.bb = self.bb - from;
            self.bb = self.bb + to;
        }
        if self.wn & from > 0 {
            self.wn = self.wn - from;
            self.wn = self.wn + to;
        }
        if self.bn & from > 0 {
            self.bn = self.bn - from;
            self.bn = self.bn + to;
        }
        if self.wp & from > 0 {

            self.wp = self.wp - from;
            self.wp = self.wp + to;
        }
        if self.bp & from > 0 {
            self.bp = self.bp - from;
            self.bp = self.bp + to;
        }

        if moove.special.is_some() {
            match moove.special {
                Some('N') => {
                    if self.turn {
                        self.wp = self.wp - (1u64 << moove.to.number);
                        self.wn = self.wn + (1u64 << moove.to.number);
                    } else {
                        self.bp = self.bp - (1u64 << moove.to.number);
                        self.bn = self.bn + (1u64 << moove.to.number);
                    };
                },
                Some('B') => {
                    if self.turn {
                        self.wp = self.wp - (1u64 << moove.to.number);
                        self.wb = self.wb + (1u64 << moove.to.number);
                    } else {
                        self.bp = self.bp - (1u64 << moove.to.number);
                        self.bb = self.bb + (1u64 << moove.to.number);
                    };
                },
                Some('R') => {
                    if self.turn {
                        self.wp = self.wp - (1u64 << moove.to.number);
                        self.wr = self.wr + (1u64 << moove.to.number);
                    } else {
                        self.bp = self.bp - (1u64 << moove.to.number);
                        self.br = self.br + (1u64 << moove.to.number);
                    };
                },
                Some('Q') => {
                    if self.turn {
                        self.wp = self.wp - (1u64 << moove.to.number);
                        self.wq = self.wq + (1u64 << moove.to.number);
                    } else {
                        self.bp = self.bp - (1u64 << moove.to.number);
                        self.bq = self.bq + (1u64 << moove.to.number);
                    };
                },
                _ => {}
            }
        }

        self.history.push(moove.clone());
        self.turn = if self.turn { false } else { true };
    }

    pub fn turn(&self) -> bool {
        self.turn
    }
}

