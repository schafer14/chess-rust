mod bitboard;
mod definitions;
mod moves;
use std::io;

mod pieces {
    pub mod white_pawns;
    pub mod black_pawns;
}

fn main() {
    let reader = io::stdin();

    let mut board = bitboard::BitBoard::new();
    println!("{}", board);

    #[allow(while_true)]
    while true {
        let mut input_text = String::new();

        #[allow(unused_must_use)]
        reader.read_line(&mut input_text);

        match input_text.trim() {
            "q" => break,
            "h" => println!("{:?}", board.moves()),
            "u" => println!("Undo not supported yet"),
            _ => {
                let moove = definitions::Move::from_str(input_text);
                board.make_move(moove);
                println!("{}", board);

            }
        }
    }


}
