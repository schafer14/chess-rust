mod bitboard;
mod definitions;
mod moves;
use std::io;

mod pieces {
    pub mod white_pawns;
    pub mod black_pawns;
}

mod ai {
    pub mod molly;
}

use ai::molly;

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

fn main() {
    let reader = io::stdin();
    let human_only = false;

    let mut board = bitboard::BitBoard::new(INIT_BOARD);

    let fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq");
//    let fen = String::from("n1n5/PPPk4/8/8/8/8/4Kppp/5N1N b");
    let mut board = bitboard::BitBoard::from_fen(fen);

    println!("{}", board);

    #[allow(while_true)]
    while true {
        if board.turn() || human_only {
            let mut input_text = String::new();

            #[allow(unused_must_use)]
            reader.read_line(&mut input_text);

            match input_text.trim() {
                "q" => break,
                "h" => println!("{:?}", board.moves()),
                "hl" => println!("{:?}", board.moves().len()),
                "u" => println!("Undo not supported yet"),
                _ => {
                    let moove = definitions::Move::from_str(input_text);
                    board.make_move(moove);
                    println!("{}", board);

                }
            }
        } else {
            match molly::gen_move(board.clone()) {
                Some(moove) => {
                    board.make_move(moove);
                    println!("{}", board);
                },
                _ => {
                    println!("No moves available");
                    break;
                }
            }
        }
    }


}
