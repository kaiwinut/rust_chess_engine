mod core;

use crate::core::masks;
use crate::core::movegen::slider::*;
use crate::core::square;
use crate::core::BitBoard;

use chrono::*;

fn main() {
    let start = Utc::now();
    fastrand::seed(9000);
    init_magic();

    let interval = Utc::now() - start;
    println!("\nTime: {} ms\n", interval.num_milliseconds());

    let sq = square::D4;
    let occ = BitBoard::new(square::B2)
        | BitBoard::new(square::E5)
        | BitBoard::new(square::C5)
        | BitBoard::new(square::F2);
    println!("{}", get_bishop_attacks(sq, occ));
}
