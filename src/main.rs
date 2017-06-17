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

//const PLAYERS:[bool; 2] = [true, false];

static START_POS: &'static str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq";


fn main() {
    let reader = io::stdin();
    
    let mut board = bitboard::BitBoard::from_fen(START_POS.to_string());

    #[allow(while_true)]
    while true {
//        if (board.turn() && PLAYERS[0]) || (!board.turn() && PLAYERS[1]) {
            let mut input_text = String::new();

            #[allow(unused_must_use)]
            reader.read_line(&mut input_text);

            let mut parts = input_text.trim().split(" ").collect::<Vec<&str>>();

            let cmd = parts.remove(0);
            match cmd {
//                "q" => break,
//                "h" => println!("{:?}", board.moves_accurate()),
//                "perft" => println!("{:?}", ai::ids::perft(board.clone(), 5)),
//                "test" => perft_test::test(),
//                "divide" => {
//                    let num = parts.clone()[1].chars().next().unwrap().to_digit(10).unwrap() as usize;
//                    ai::ids::divide(board.clone(), num - 1);
//                },
//                "d" => println!("{}", board),
//                "m" => {
//                    let moove = molly::gen_move(board.clone()).unwrap();
//                    old_board = board.clone();
//                    board.make_move(moove);
//                    println!("{}", board);
//                },
//                "s" => {
//                    let moove = molly::gen_move(board.clone()).unwrap();
//                    println!("Suggestion {:?}", moove);
//                },
//                "n" => {
//                    board = bitboard::BitBoard::from_fen(fen.clone());
//                    println!("{}", board);
//                }
//                "u" => {
//                    board = old_board.clone();
//                    println!("{}", board);
//                },
//
//                _ => {
//                    let moove = definitions::Move::from_str(input_text.clone());
//                    old_board = board.clone();
//                    board.make_move(moove);
//                    println!("{}", board);
//
//                }

//                UCI Stuff
                "uci" => {
                    println!("id name Maple Heather Bennett-Schafer");
                    println!("id author Banner B. Schafer");
                    println!("uciok");
                },

                "setoption" => {
                    println!("Not set up to handle options");
                },

                "isready" => {
                    println!("readyok");
                },

                "position" => {
//                    TODO BBS real notation does not have a space
                    if parts.remove(0) == "startpos" {
                        if parts.len() > 0 {
                            board = bitboard::BitBoard::from_fen(START_POS.to_string());
                            parts.remove(0);
                            while parts.len() > 0 {
                                let moove_str = parts.remove(0);
                                let moove = definitions::Move::from_str(String::from(moove_str));
                                board.make_move(moove);
                            }
                        }
                    } else {
//                        TODO BBS Do this
                        println!("Cannot start from FEN");
                    }
                },

                "go" => {
                    let moove = ai::molly::gen_move(board.clone());
                    println!("bestmove {}", moove.unwrap());
                },

                "quit" => break,

                _ => {
                    println!("Unknown UCI command ignoring.");
                }
            }
//        } else {
//            match molly::gen_move(board.clone()) {
//                Some(moove) => {
//                    board.make_move(moove);
//                    println!("{}", board);
//                },
//                _ => {
//                    println!("No moves available");
//                    break;
//                }
//            }
//        }
    }
}
