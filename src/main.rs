mod core;
use rust_chess_engine::core::masks::FILE_A;

use crate::core::BitBoard;
use crate::core::masks;

fn main() {
    let bitboard = BitBoard(masks::RANK_5);
    println!("{}", bitboard);
}
