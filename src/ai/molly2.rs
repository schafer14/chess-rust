extern crate time;

use std::thread;
use std::cmp::Ordering;

use bitboard::BitBoard;
use definitions::Move;
use definitions::Square;

use std;

const MIN:isize = std::isize::MIN;
const MAX:isize = std::isize::MAX;

pub fn gen_move(bb:BitBoard) -> Option<Move> {
    let start_time = time::precise_time_s();

    let mut depth = 0;
    let mut master_plan = Vec::new();

    while time::precise_time_s() - start_time < 3.0 {
        depth = depth + 1;
        let color_multiplier = if bb.turn { 1 } else { -1 };
        let (plan, score) = negamax(&bb, depth, color_multiplier, MIN + 1, MAX);

        master_plan = plan;
        println!("{} {} {:?}", depth, score, master_plan);
        if score > 1000 || score < -1000 { break; }
    }

    Some(master_plan.last().unwrap().clone())
}

fn negamax(bb:&BitBoard, depth:usize, color:isize, mut alpha:isize, beta:isize) -> (Vec<Move>, isize) {
    if depth == 0 || is_terminal(bb) {
        return (Vec::new(), color * utility(bb));
    }

//    Look for taking moves first
    let opponents = if color == 1 {
        bb.bq | bb.br | bb.bb | bb.bn | bb.bp | bb.bk
    } else {
        bb.wq | bb.wr | bb.wb | bb.wn | bb.wp | bb.wk
    };

    let mut best = MIN;
    let mut best_plan = Vec::new();
    let mut moves = bb.moves();

    moves.sort_by(|a, b| -> Ordering {
        let a_takes = opponents & 1u64 << a.to.number > 0;
        let b_takes = opponents & 1u64 << b.to.number > 0;

        match (a_takes, b_takes) {
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            _ => Ordering::Equal,
        }
    });

    for moove in moves {
        let mut child = bb.clone();
        child.make_move(moove);

        let (mut plan, mut value) = negamax(&child, depth - 1, -color, -beta, -alpha);
        value = -value;

        if value > best {
            plan.push(moove);
            best = value;
            best_plan = plan;
        }

        alpha = if value > alpha { value } else { alpha };

        if alpha > beta { break; }
    }

    (best_plan, best)
}

fn is_terminal(bb:&BitBoard) -> bool {
    if bb.wk.count_ones() == 0 || bb.bk.count_ones() == 0 {
        true
    } else {
        false
    }
}

fn utility(bb:&BitBoard) -> isize {
    10000 * (bb.wk.count_ones() as isize - bb.bk.count_ones() as isize) +
        9 * (bb.wq.count_ones() as isize - bb.bq.count_ones() as isize) +
        5 * (bb.wr.count_ones() as isize - bb.br.count_ones() as isize) +
        3 * (bb.wb.count_ones() as isize - bb.bb.count_ones() as isize) +
        3 * (bb.wn.count_ones() as isize - bb.bn.count_ones() as isize) +
        1 * (bb.wp.count_ones() as isize - bb.bp.count_ones() as isize)
}