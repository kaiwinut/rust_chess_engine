use core::fmt;
use super::square::*;

pub struct BitBoard (pub u64);

impl BitBoard {
    pub fn new(square: Square) -> Self {
        BitBoard(1u64 << square.to_usize())
    }

    pub fn from(raw: u64) -> Self {
        BitBoard(raw)
    }

    pub fn to_u64(self) -> u64 {
        self.0
    }

    pub fn is_set(&self, square: Square) -> bool {
        (self.0 >> square.to_usize()) & 1 != 0
    }
}

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