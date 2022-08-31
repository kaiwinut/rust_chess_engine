use core::fmt;
use super::square::Square;
use super::utils::*;
use super::masks;

#[derive(Debug, PartialEq, Clone, Copy)]
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

    pub fn pop_count(&self) -> usize {
        self.0.count_ones() as usize
    }

    pub fn pop_lsb(&self) -> BitBoard {
        BitBoard(self.0 & self.0 - 1)
    }

    pub fn lsb(&self) -> BitBoard {
        BitBoard(self.0 & 0u64.wrapping_sub(self.0))
    }

    pub fn bit_scan(&self) -> usize {
        self.0.trailing_zeros() as usize
    }
}

pub const EMPTY: BitBoard = BitBoard(0u64);
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

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::square;

    #[test]
    fn test_lsb() {
        let bitboard = BitBoard::new(square::D4);
        assert_eq!(bitboard.lsb(), BitBoard(1u64 << square::D4.to_usize()));
    }

    #[test]
    fn test_pop_lsb() {
        let bitboard = BitBoard::new(square::D4);
        assert_eq!(bitboard.pop_lsb(), BitBoard(masks::EMPTY));
    } 

    #[test]
    fn test_pop_count() {
        let bitboard = BitBoard::new(square::D4);
        assert_eq!(bitboard.pop_count(), 1);
    } 

    #[test]
    fn test_bit_scan() {
        let bitboard = BitBoard::new(square::D4);
        assert_eq!(bitboard.bit_scan(), square::D4.0);
    } 
}