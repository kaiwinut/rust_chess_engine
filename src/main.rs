mod core;
use crate::core::movegen::slider;
use crate::core::BitBoard;

fn main() {
    let bb = BitBoard(0x1010106e101000);
    println!("{}", bb);
    for i in 1..10 {
        let occ = slider::generate_occupancy(bb, i as u64);
        println!("{}", occ);
    }
}
