use core::fmt;
use super::square::*;
use super::masks;

pub struct BitBoard (pub u64);

impl BitBoard {
    pub fn new(square: Square) -> Self {
        BitBoard(1u64 << square.to_usize())
    }

    pub fn to_u64(self) -> u64 {
        self.0
    }

    pub fn is_set(&self, square: Square) -> bool {
        (self.0 >> square.to_usize()) & 1 != 0
    }
}

pub const EMPTY: BitBoard = BitBoard(0);
pub const FILE_A: BitBoard = BitBoard(masks::FILE_A);
pub const NOT_FILE_A: BitBoard = BitBoard(!masks::FILE_A);
pub const FILE_H: BitBoard = BitBoard(masks::FILE_H);
pub const NOT_FILE_H: BitBoard = BitBoard(!masks::FILE_H);
pub const FILE_B: BitBoard = BitBoard(masks::FILE_B);
pub const FILE_G: BitBoard = BitBoard(masks::FILE_G);


impl fmt::Display for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            grid_to_string(|square: Square| -> &str {
                if self.is_set(square) {
                    "# "
                } else {
                    ". "
                }
            }, self.0)
        )
    }
}