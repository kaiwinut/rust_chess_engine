mod core;
use crate::core::bitboard;

fn main() {
    let bitboard = bitboard::NOT_FILE_A;
    println!("{}", bitboard);
}
