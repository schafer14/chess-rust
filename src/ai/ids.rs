extern crate time;

use bitboard;

pub fn perft(main_board:bitboard::BitBoard, depth:usize) -> usize {
    let board = main_board.clone();

    ldfs(board, depth)
}

pub fn divide(main_board:bitboard::BitBoard, depth:usize) {
    let board = main_board.clone();

    let mut moves = 0;
    let mut nodes = 0;
    for moove in board.moves_accurate() {
        moves = moves + 1;

        let mut child = board.clone();
        child.make_move(moove.clone());

        let expansions = ldfs(child, depth);
        nodes = nodes + expansions;
        println!("{:?} {}", moove, expansions)
    }
    println!("");
    println!("Moves: {}", moves);
    println!("Nodes: {}", nodes);
}

fn ldfs(original_board:bitboard::BitBoard, depth:usize) -> usize {
    let board = original_board.clone();
    let mut expanded = 0;

    if depth < 1 {
        return 1;
    }

    for moove in board.moves_accurate() {
        let mut child = board.clone();
        child.make_move(moove.clone());

        let e = ldfs(child, depth - 1);
        expanded += e;

    }

    expanded
}
