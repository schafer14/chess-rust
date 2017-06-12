use pieces::white_pawns;
use pieces::black_pawns;
use moves;
use definitions;

use std::fmt;

const INIT_BOARD: [[char; 8]; 8] = [
    ['r', 'n', 'b', 'q', 'k', 'b', 'n', 'r'],
    ['p', 'p', 'p', 'p', 'p', 'p', 'p', 'p'],
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    ['P', 'P', 'P', 'P', 'P', 'P', 'P', 'P'],
    ['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R']
];

#[derive(Clone)]
pub struct BitBoard {
    bp: u64,
    wp: u64,
    bn: u64,
    wn: u64,
    bb: u64,
    wb: u64,
    br: u64,
    wr: u64,
    bq: u64,
    wq: u64,
    bk: u64,
    wk: u64,
    castling: [bool; 4],
    turn: bool,
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

        write!(f, "  a b c d e f g h\n")
    }
}

impl BitBoard {
    pub fn new() -> BitBoard {
        let mut bitboard = BitBoard { bp: 0, wp: 0, bn: 0, wn: 0, bb: 0, wb: 0, br: 0, wr: 0, bq: 0,
            wq: 0, bk: 0, wk: 0, turn: true, castling: [true, true, true, true], history: Vec::new() };

        for row in 0..8 {
            for col in 0..8 {
                let square:u32 = ((7 - row) * 8 + col) as u32;
                let digit = 2u64.pow(square);

                if 'p' == INIT_BOARD[row][col] {
                    bitboard.bp +=  digit
                };

                if 'P' == INIT_BOARD[row][col] {
                    bitboard.wp +=  digit
                };

                if 'n' == INIT_BOARD[row][col] {
                    bitboard.bn +=  digit
                };

                if 'N' == INIT_BOARD[row][col] {
                    bitboard.wn +=  digit
                };

                if 'b' == INIT_BOARD[row][col] {
                    bitboard.bb +=  digit
                };

                if 'B' == INIT_BOARD[row][col] {
                    bitboard.wb +=  digit
                };

                if 'r' == INIT_BOARD[row][col] {
                    bitboard.br +=  digit
                };

                if 'R' == INIT_BOARD[row][col] {
                    bitboard.wr +=  digit
                };

                if 'q' == INIT_BOARD[row][col] {
                    bitboard.bq +=  digit
                };

                if 'Q' == INIT_BOARD[row][col] {
                    bitboard.wq +=  digit
                };

                if 'k' == INIT_BOARD[row][col] {
                    bitboard.bk +=  digit
                };

                if 'K' == INIT_BOARD[row][col] {
                    bitboard.wk +=  digit
                };
            }

        }

        bitboard


    }
}

impl BitBoard {
//    TODO BBS: En Passant bitboard
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
        }


        moves_list
    }

    pub fn castling_white(&self, empty:u64) -> Vec<definitions::Move> {
        let mut moves = Vec::new();

        let could_castle_right:bool = empty & definitions::CASTLING_BITBOARD[0] > 0
            && self.castling[0];

        let could_castle_left:bool = empty & definitions::CASTLING_BITBOARD[1] > 0
            && self.castling[1];

        //    Castle right
//        Only do costly operations once
        if  could_castle_left || could_castle_right {
            //        Check attack span
            let opponents = self.wp | self.wn | self.wb | self.wr | self.wq | self.wk;
            let mut opponent_attacks: u64 = 0;
            let protected_squares_right: u64 = 0b1110000;
            let protected_squares_left: u64 = 0b11110;

            //            TODO BBS: Check one bitboard at a time starting with most powerful pieces
            let mut moves_list = black_pawns::moves(self.bp, empty, opponents, self.wp, Vec::new());
            moves_list.append(&mut moves::straight(self.br, empty, opponents));
            moves_list.append(&mut moves::straight(self.bq, empty, opponents));
            moves_list.append(&mut moves::diagonal(self.bb, empty, opponents));
            moves_list.append(&mut moves::diagonal(self.bq, empty, opponents));
            moves_list.append(&mut moves::knight(self.bn, empty, opponents));
            moves_list.append(&mut moves::king(self.bk, empty, opponents));

            for m in moves_list {
                opponent_attacks = opponent_attacks | 1u64 << m.to.number;
            };

            if protected_squares_right & opponent_attacks == 0 && could_castle_right {
                moves.push(definitions::Move::from_num_special(0, 0, 'o'));
            }

            if protected_squares_left & opponent_attacks == 0 && could_castle_left {
                moves.push(definitions::Move::from_num_special(0, 0, 'O'));
            }
        }

        moves
    }

    pub fn castling_black(&self, empty:u64) -> Vec<definitions::Move> {
        let mut moves = Vec::new();

        let could_castle_right:bool = empty & definitions::CASTLING_BITBOARD[2] > 0
            && self.castling[2];

        let could_castle_left:bool = empty & definitions::CASTLING_BITBOARD[3] > 0
            && self.castling[3];

        //    Castle right
        //        Only do costly operations once
        if  could_castle_left || could_castle_right {
            //        Check attack span
            let opponents = self.bp | self.bn | self.bb | self.br | self.bq | self.bk;
            let mut opponent_attacks: u64 = 0;
            let protected_squares_right: u64 = 0b0111000000000000000000000000000000000000000000000000000000000000;
            let protected_squares_left: u64 = 0b0001111000000000000000000000000000000000000000000000000000000000;

            //            TODO BBS: Check one bitboard at a time starting with most powerful pieces
            let mut moves_list = black_pawns::moves(self.wp, empty, opponents, self.bp, Vec::new());
            moves_list.append(&mut moves::straight(self.wr, empty, opponents));
            moves_list.append(&mut moves::straight(self.wq, empty, opponents));
            moves_list.append(&mut moves::diagonal(self.wb, empty, opponents));
            moves_list.append(&mut moves::diagonal(self.wq, empty, opponents));
            moves_list.append(&mut moves::knight(self.wn, empty, opponents));
            moves_list.append(&mut moves::king(self.wk, empty, opponents));

            for m in moves_list {
                opponent_attacks = opponent_attacks | 1u64 << m.to.number;
            };

            if protected_squares_right & opponent_attacks == 0 && could_castle_right {
                moves.push(definitions::Move::from_num_special(0, 0, 'o'));
            }

            if protected_squares_left & opponent_attacks == 0 && could_castle_left {
                moves.push(definitions::Move::from_num_special(0, 0, 'O'));
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
                    } else {
                        self.bk = self.bk - (1u64 << 60);
                        self.bk = self.bk + (1u64 << 62);
                        self.br = self.br - (1u64 << 63);
                        self.br = self.br + (1u64 << 61);
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
                    } else {
                        self.bk = self.bk - (1u64 << 60);
                        self.bk = self.bk + (1u64 << 58);
                        self.br = self.br - (1u64 << 56);
                        self.br = self.br + (1u64 << 59);
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
        }
        if self.bk & from > 0 {
            self.bk = self.bk - from;
            self.bk = self.bk + to;
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
        }
        if self.br & from > 0 {
            self.br = self.br - from;
            self.br = self.br + to;
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
}