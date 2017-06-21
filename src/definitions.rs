use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Square {
    pub number: u8
}

#[derive(Clone, Copy, PartialEq)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub special: Option<char>,
}

impl fmt::Debug for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let letters = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

        let i0 = letters[(self.from.number % 8) as usize];
        let j0 = (self.from.number / 8) + 1;

        let i1 = letters[(self.to.number % 8) as usize];
        let j1 = (self.to.number / 8) + 1;


        match self.special {
            Some(x) => write!(f, "{}{:?}{}{:?}{}", i0, j0, i1, j1, x),
            None => write!(f, "{}{:?}{}{:?}", i0, j0, i1, j1)
        }
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let letters = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

        let i0 = letters[(self.from.number % 8) as usize];
        let j0 = (self.from.number / 8) + 1;

        let i1 = letters[(self.to.number % 8) as usize];
        let j1 = (self.to.number / 8) + 1;

        let special = self.special.unwrap_or(' ');

        write!(f, "{}{:?}{}{:?}{}", i0, j0, i1, j1, special)
    }
}

impl Move {
    pub fn from_num(n1: u8, n2: u8) -> Move {
        Move { from: Square { number: n1 }, to: Square { number: n2 }, special: None}
    }

    pub fn from_num_special(n1: u8, n2: u8, c: char) -> Move {
        Move { from: Square { number: n1 }, to: Square { number: n2 }, special: Some(c)}
    }

    pub fn from_str(str:String) -> Move {
        let possibles = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        let mut chars = str.chars();

        let char_1 = chars.next().unwrap();
        let from_i = possibles.iter().position(|&c| c == char_1).unwrap();

        let from_j = chars.next().unwrap().to_digit(10).unwrap() as usize;

        let char_2 = chars.next().unwrap();
        let to_i = possibles.iter().position(|&c| c == char_2).unwrap();

        let to_j = chars.next().unwrap().to_digit(10).unwrap() as usize;

        match chars.next() {
            Some(x) => Move { from: Square { number: (from_i + (from_j - 1) * 8) as u8 }, to: Square { number: (to_i + (to_j - 1) * 8) as u8 }, special: Some(x) },
            None => Move { from: Square { number: (from_i + (from_j - 1) * 8) as u8 }, to: Square { number: (to_i + (to_j - 1) * 8) as u8 }, special: None },
        }


    }
}

#[allow(dead_code)]
pub fn show_board(board:u64) {
    for row in 0..8 {
        for col in 0..8 {
            let square:u32 = ((7 - row) * 8 + col) as u32;
            let digit = 2u64.pow(square);


            if digit & board > 0 {
                print!("• ");
            } else {
                print!("o ");
            }
        }
        println!("");
    }
    println!("");
}

pub const FILE_H:u64 = 0b1000000010000000100000001000000010000000100000001000000010000000;
pub const FILE_A:u64 = 0b0000000100000001000000010000000100000001000000010000000100000001;
pub const RANK_8:u64 = 0b1111111100000000000000000000000000000000000000000000000000000000;
pub const RANK_1:u64 = 0b0000000000000000000000000000000000000000000000000000000011111111;
pub const RANK_4:u64 = 0b0000000000000000000000000000000011111111000000000000000000000000;
pub const RANK_5:u64 = 0b0000000000000000000000001111111100000000000000000000000000000000;
pub const CASTLING_BITBOARD:[u64; 4] = [
    0b0000000000000000000000000000000000000000000000000000000001100000,
    0b0000000000000000000000000000000000000000000000000000000000001110,
    0b0110000000000000000000000000000000000000000000000000000000000000,
    0b0000111000000000000000000000000000000000000000000000000000000000,
];

pub const ALL:u64 = 0xFFFFFFFFFFFFFFFF;

pub const KNIGHT_BITBOARDS:[u64; 64] = [
    0x20400,
    0x50800,
    0xa1100,
    0x142200,
    0x284400,
    0x508800,
    0xa01000,
    0x402000,
    0x2040004,
    0x5080008,
    0xa110011,
    0x14220022,
    0x28440044,
    0x50880088,
    0xa0100010,
    0x40200020,
    0x204000402,
    0x508000805,
    0xa1100110a,
    0x1422002214,
    0x2844004428,
    0x5088008850,
    0xa0100010a0,
    0x4020002040,
    0x20400040200,
    0x50800080500,
    0xa1100110a00,
    0x142200221400,
    0x284400442800,
    0x508800885000,
    0xa0100010a000,
    0x402000204000,
    0x2040004020000,
    0x5080008050000,
    0xa1100110a0000,
    0x14220022140000,
    0x28440044280000,
    0x50880088500000,
    0xa0100010a00000,
    0x40200020400000,
    0x204000402000000,
    0x508000805000000,
    0xa1100110a000000,
    0x1422002214000000,
    0x2844004428000000,
    0x5088008850000000,
    0xa0100010a0000000,
    0x4020002040000000,
    0x400040200000000,
    0x800080500000000,
    0x1100110a00000000,
    0x2200221400000000,
    0x4400442800000000,
    0x8800885000000000,
    0x100010a000000000,
    0x2000204000000000,
    0x4020000000000,
    0x8050000000000,
    0x110a0000000000,
    0x22140000000000,
    0x44280000000000,
    0x88500000000000,
    0x10a00000000000,
    0x20400000000000
];


pub const KING_BITBOARDS:[u64; 64] = [
    0x302,
    0x705,
    0xe0a,
    0x1c14,
    0x3828,
    0x7050,
    0xe0a0,
    0xc040,
    0x30203,
    0x70507,
    0xe0a0e,
    0x1c141c,
    0x382838,
    0x705070,
    0xe0a0e0,
    0xc040c0,
    0x3020300,
    0x7050700,
    0xe0a0e00,
    0x1c141c00,
    0x38283800,
    0x70507000,
    0xe0a0e000,
    0xc040c000,
    0x302030000,
    0x705070000,
    0xe0a0e0000,
    0x1c141c0000,
    0x3828380000,
    0x7050700000,
    0xe0a0e00000,
    0xc040c00000,
    0x30203000000,
    0x70507000000,
    0xe0a0e000000,
    0x1c141c000000,
    0x382838000000,
    0x705070000000,
    0xe0a0e0000000,
    0xc040c0000000,
    0x3020300000000,
    0x7050700000000,
    0xe0a0e00000000,
    0x1c141c00000000,
    0x38283800000000,
    0x70507000000000,
    0xe0a0e000000000,
    0xc040c000000000,
    0x302030000000000,
    0x705070000000000,
    0xe0a0e0000000000,
    0x1c141c0000000000,
    0x3828380000000000,
    0x7050700000000000,
    0xe0a0e00000000000,
    0xc040c00000000000,
    0x203000000000000,
    0x507000000000000,
    0xa0e000000000000,
    0x141c000000000000,
    0x2838000000000000,
    0x5070000000000000,
    0xa0e0000000000000,
    0x40c0000000000000
];
