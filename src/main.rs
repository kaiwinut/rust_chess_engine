mod core;
mod perft;

use std::mem::MaybeUninit;

use crate::core::movegen::{self, get_bishop_attacks, get_queen_attacks};
use crate::core::movegen::movescan::*;
use crate::core::board::*;
use crate::core::{square, BitBoard};

use chrono::*;


fn main() {
    let start = Utc::now();
    fastrand::seed(9000);
    movegen::init();

    let interval = Utc::now() - start;
    println!("\nTime: {} ms\n", interval.num_milliseconds());

    let depth = 6;
    let start = Utc::now();
    let count = perft::run(depth);
    let interval = Utc::now() - start;
    let mnps = (count as f64 / interval.num_milliseconds() as f64) / 1000.0;
    println!(
        "\nPerft test :  Depth {} | {} nodes | Time {} ms ({:.3} Mnps)\n",
        depth,
        count,
        interval.num_milliseconds(),
        mnps
    );

    // let occ = BitBoard(0x0000_0000_0000_ffff) & !BitBoard::new(square::E2) | BitBoard::new(square::E3);

    // println!(
    //     "{}",
    //     get_queen_attacks(square::D1, occ) & !occ
    // );

    // println!(
    //     "{}",
    //     get_bishop_attacks(square::F1, occ) & !occ
    // );

    // let mut board = Board::new(false);
    // board.make_move(&Move::new(square::E2, square::E3, MoveFlags::QUIET));
    // board.make_move(&Move::new(square::E7, square::E6, MoveFlags::QUIET));
    // let mut moves: [Move; 218] = unsafe { MaybeUninit::uninit().assume_init() };
    // let mut index : usize = 0;
    // index = scan_piece_moves(&board, Piece::WQ, &mut moves, index);

    // for m in moves.iter().take(index) {
    //     println!("move {}", m);
    // }
}
