extern crate time;

use bitboard;
use definitions;

const VERBOSE:bool = true;
const DEPTH:usize = 5;
const ALPHA_BETA:bool = false;

pub fn gen_move(main_board:bitboard::BitBoard) -> Option<definitions::Move> {
    let start_time = time::precise_time_s();
    let mut moove:Option<definitions::Move> = None;
    let mut score = 0;
    let mut expanded = 0;

    for depth in 1..DEPTH {
        let board = main_board.clone();
        let turn = board.turn;
        let (s, m, e) = minimax(board, depth, true, turn, <isize>::min_value(), <isize>::max_value(), ALPHA_BETA);
        if VERBOSE {
            println!("Level {:?}: Expanded: {:?}", depth, e);
        }
        moove = m;
        score = s;
        expanded += e;
    };

    let end_time = time::precise_time_s();

    if VERBOSE {
        println!("Move: {:?} Score: {:?}", moove, score);
        println!("Expanded: {:?} nodes in {:?} seconds", expanded, end_time - start_time);
        println!("Nodes per second: {:?}", expanded as f64 / (end_time - start_time));
    }

    moove
}

fn minimax(original_board:bitboard::BitBoard, depth:usize, maximizing:bool, player:bool, mut alpha:isize, mut beta:isize, use_alpha_beta:bool) -> (isize, Option<definitions::Move>, usize) {
    let board = original_board.clone();
    let mut expanded = 0;

    if depth < 1 {
        return (utility(board, player), None, 1);
    }

    if maximizing {
        let mut best = <isize>::min_value();
        let mut best_moove:Option<definitions::Move> = None;

        for moove in board.moves() {
            let mut child = board.clone();
            child.make_move(moove.clone());

            let (value, _, e) = minimax(child, depth - 1, false, player, alpha, beta, use_alpha_beta);
            expanded += e;

            if value > best {
                best_moove = Some(moove);
                best = value;
            }

            if value >= beta && use_alpha_beta {
                return (best, best_moove, expanded);
            }

            alpha = if alpha > value { alpha } else { value };
        }

        (best, best_moove, expanded)
    } else {
        let mut best = <isize>::max_value();
        let mut best_moove:Option<definitions::Move> = None;

        for moove in board.moves() {
            let mut child = board.clone();
            child.make_move(moove.clone());

            let (value, _, e) = minimax(child, depth - 1, true, player, alpha, beta, use_alpha_beta);
            expanded += e;

            if value < best {
                best_moove = Some(moove);
                best = value;
            }

            if value <= alpha && use_alpha_beta {
                return (best, best_moove, expanded);
            }

            beta = if beta < value { beta } else { value };
        }

        (best, best_moove, expanded)
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

