//extern crate rand;
//use self::rand::thread_rng;
//use self::rand::sample;

use bitboard;
use definitions;

pub fn gen_move(main_board:bitboard::BitBoard) -> Option<definitions::Move> {
    let mut moove:Option<definitions::Move> = None;
    let mut score = 0;

    for depth in 0..6 {
        let board = main_board.clone();
        let turn = board.turn;
        let (s, m) = minimax(board, depth, true, turn);
        moove = m;
        score = s;
    };

    println!("Move: {:?}\nScore: {:?}", moove, score);
    moove
}

fn minimax(original_board:bitboard::BitBoard, depth:usize, maximizing:bool, player:bool) -> (isize, Option<definitions::Move>) {
    let board = original_board.clone();

    if depth < 1 {
        return (utility(board, player), None);
    }

    if maximizing {
        let mut best = <isize>::min_value();
        let mut best_moove:Option<definitions::Move> = None;

        for moove in board.moves() {
            let mut child = board.clone();
            child.make_move(moove.clone());

            let (value, _) = minimax(child, depth - 1, false, player);

            if value > best {
                best_moove = Some(moove);
                best = value;
            }
        }

        (best, best_moove)
    } else {
        let mut best = <isize>::max_value();
        let mut best_moove:Option<definitions::Move> = None;

        for moove in board.moves() {
            let mut child = board.clone();
            child.make_move(moove.clone());

            let (value, _) = minimax(child, depth - 1, true, player);

            if value < best {
                best_moove = Some(moove);
                best = value;
            }
        }

        (best, best_moove)
    }


}

fn utility(original_board:bitboard::BitBoard, player:bool) -> isize {
    let board = original_board.clone();

    let white_score =
        board.wp.count_ones() +
        board.wn.count_ones() * 3 +
        board.wb.count_ones() * 3 +
        board.wr.count_ones() * 5 +
        board.wq.count_ones() * 9;

    let black_score =
        board.bp.count_ones() +
        board.bn.count_ones() * 3 +
        board.bb.count_ones() * 3 +
        board.br.count_ones() * 5 +
        board.bq.count_ones() * 9;

    if player {
        white_score as isize - black_score as isize
    } else {
        black_score as isize - white_score as isize
    }
}