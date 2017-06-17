mod bitboard;
mod definitions;
mod moves;
mod perft_test;
use std::io;

mod pieces {
    pub mod white_pawns;
    pub mod black_pawns;
}

mod ai {
    pub mod molly;
    pub mod ids;
}

use ai::molly;

const PLAYERS:[bool; 2] = [true, true];


fn main() {
    let reader = io::stdin();
    
//    let fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq");
    let fen = String::from("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq");
    let mut board = bitboard::BitBoard::from_fen(fen.clone());
    let mut old_board = board.clone();

    println!("{}", board);

    #[allow(while_true)]
    while true {
        if (board.turn() && PLAYERS[0]) || (!board.turn() && PLAYERS[1]) {
            let mut input_text = String::new();

            #[allow(unused_must_use)]
            reader.read_line(&mut input_text);

            let parts = input_text.trim().split(" ").collect::<Vec<&str>>();

            match parts[0] {
                "q" => break,
                "h" => println!("{:?}", board.moves_accurate()),
                "perft" => println!("{:?}", ai::ids::perft(board.clone(), 5)),
                "test" => perft_test::test(),
                "divide" => {
                    let num = parts.clone()[1].chars().next().unwrap().to_digit(10).unwrap() as usize;
                    ai::ids::divide(board.clone(), num - 1);
                },
                "d" => println!("{}", board),
                "m" => {
                    let moove = molly::gen_move(board.clone()).unwrap();
                    old_board = board.clone();
                    board.make_move(moove);
                    println!("{}", board);
                },
                "s" => {
                    let moove = molly::gen_move(board.clone()).unwrap();
                    println!("Suggestion {:?}", moove);
                },
                "n" => {
                    board = bitboard::BitBoard::from_fen(fen.clone());
                    println!("{}", board);
                }
                "u" => {
                    board = old_board.clone();
                    println!("{}", board);
                },
                _ => {
                    let moove = definitions::Move::from_str(input_text.clone());
                    old_board = board.clone();
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
