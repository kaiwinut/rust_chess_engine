use super::square::Square;
use super::utils::*;
use core::fmt;
use std::ops::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct BitBoard(pub u64);

impl BitBoard {
    #[allow(dead_code)]
    pub fn new(square: Square) -> Self {
        BitBoard(1u64 << square.to_usize())
    }

    #[allow(dead_code)]
    pub fn to_u64(self) -> u64 {
        self.0
    }

    #[allow(dead_code)]
    pub fn to_usize(self) -> usize {
        self.0 as usize
    }

    #[allow(dead_code)]
    pub fn is_set(&self, square: Square) -> bool {
        (self.0 >> square.to_usize()) & 1 != 0
    }

    #[allow(dead_code)]
    pub fn pop_count(&self) -> u8 {
        self.0.count_ones() as u8
    }

    #[allow(dead_code)]
    pub fn pop_lsb(&self) -> BitBoard {
        BitBoard(self.0 & (self.0 - 1))
    }

    #[allow(dead_code)]
    pub fn lsb(&self) -> BitBoard {
        BitBoard(self.0 & self.0.wrapping_neg())
    }

    #[allow(dead_code)]
    pub fn bit_scan(&self) -> u8 {
        self.0.trailing_zeros() as u8
    }
}

impl Shl<usize> for BitBoard {
    type Output = BitBoard;

    fn shl(self, rhs: usize) -> Self::Output {
        BitBoard(self.0 << rhs)
    }
}

impl Shr<usize> for BitBoard {
    type Output = BitBoard;

    fn shr(self, rhs: usize) -> Self::Output {
        BitBoard(self.0 >> rhs)
    }
}

impl Shr<u8> for BitBoard {
    type Output = BitBoard;

    fn shr(self, rhs: u8) -> Self::Output {
        BitBoard(self.0 >> rhs)
    }
}

impl Shl<u8> for BitBoard {
    type Output = BitBoard;

    fn shl(self, rhs: u8) -> Self::Output {
        BitBoard(self.0 << rhs)
    }
}

impl Not for BitBoard {
    type Output = BitBoard;

    fn not(self) -> Self::Output {
        BitBoard(!self.0)
    }
}

impl BitAnd for BitBoard {
    type Output = BitBoard;

    fn bitand(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 & rhs.0)
    }
}

impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
    }
}

impl BitOr for BitBoard {
    type Output = BitBoard;

    fn bitor(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 | rhs.0)
    }
}

impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
    }
}

impl BitXor for BitBoard {
    type Output = BitBoard;

    fn bitxor(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for BitBoard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0
    }
}

impl Sub for BitBoard {
    type Output = BitBoard;

    fn sub(self, rhs: Self) -> Self::Output {
        BitBoard(self.0.wrapping_sub(rhs.0))
    }
}

impl Add for BitBoard {
    type Output = BitBoard;

    fn add(self, rhs: Self) -> Self::Output {
        BitBoard(self.0.wrapping_add(rhs.0))
    }
}

impl Mul for BitBoard {
    type Output = BitBoard;

    fn mul(self, rhs: Self) -> Self::Output {
        BitBoard(self.0.wrapping_mul(rhs.0))
    }
}

impl Neg for BitBoard {
    type Output = BitBoard;

    fn neg(self) -> Self::Output {
        BitBoard(self.0.wrapping_neg())
    }
}

impl fmt::Display for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            grid_to_string(
                |square: Square| -> &str {
                    if self.is_set(square) {
                        "# "
                    } else {
                        ". "
                    }
                },
                Some(self.0)
            )
        )
    }
}
