mod core;

// use crate::core::masks;
use crate::core::movegen;
use crate::core::square;
use crate::core::BitBoard;

use chrono::*;

fn main() {
    let start = Utc::now();
    fastrand::seed(9000);
    movegen::init();

    let interval = Utc::now() - start;
    println!("\nTime: {} ms\n", interval.num_milliseconds());

    // let sq = square::A2;
    // let occ = BitBoard::new(square::B2)
    //     | BitBoard::new(square::E5)
    //     | BitBoard::new(square::C5)
    //     | BitBoard::new(square::F2)
    //     | BitBoard::new(square::D7)
    //     | BitBoard::new(square::A5)
    //     | BitBoard::new(square::C4);
    let sq = square::B4;
    let occ = BitBoard(0x0008001404002200);
    println!("{}", occ);
    println!("{}", movegen::get_rook_attacks(sq, occ));
    println!("{}", movegen::get_bishop_attacks(sq, occ));
    println!("{}", movegen::get_queen_attacks(sq, occ));
    println!("{}", movegen::get_king_attacks(sq));
    println!("{}", movegen::get_knight_attacks(sq));
}
