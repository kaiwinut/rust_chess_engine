use super::square::Square;
use super::utils::*;
use core::fmt;
use std::ops::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct BitBoard(pub u64);

impl BitBoard {
    pub fn new(square: Square) -> Self {
        BitBoard(1u64 << square.to_usize())
    }

    pub fn from_squares(squares: &[Square]) -> Self {
        let mut bb = BitBoard::EMPTY;
        for sq in squares {
            bb |= BitBoard::new(*sq);
        }

        bb
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0u64
    }

    pub fn is_not_empty(&self) -> bool {
        self.0 != 0u64
    }

    pub fn is_set(&self, square: Square) -> bool {
        (self.0 >> square.to_usize()) & 1 != 0
    }

    pub fn pop_edges(self) -> Self {
        self & !Self::EDGES
    }

    pub fn pop_north_south_edges(self) -> Self {
        self & !(Self::RANK_1 | Self::RANK_8)
    }

    pub fn pop_east_west_edges(self) -> Self {
        self & !(Self::FILE_A | Self::FILE_H)
    }

    pub fn pop_lsb(&self) -> Self {
        BitBoard(self.0 & (self.0 - 1))
    }

    pub fn lsb(&self) -> Self {
        BitBoard(self.0 & self.0.wrapping_neg())
    }

    pub fn bit_scan(&self) -> u8 {
        self.0.trailing_zeros() as u8
    }

    pub fn pop_count(&self) -> u8 {
        self.0.count_ones() as u8
    }

    pub fn to_u64(self) -> u64 {
        self.0
    }

    pub fn to_usize(self) -> usize {
        self.0 as usize
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

impl BitBoard {
    #[allow(dead_code)]
    pub const EMPTY: BitBoard = BitBoard(0u64);

    #[allow(dead_code)]
    pub const FILE_A: BitBoard = BitBoard(0x0101_0101_0101_0101);
    #[allow(dead_code)]
    pub const FILE_B: BitBoard = BitBoard(0x0101_0101_0101_0101 << 1);
    #[allow(dead_code)]
    pub const FILE_C: BitBoard = BitBoard(0x0101_0101_0101_0101 << 2);
    #[allow(dead_code)]
    pub const FILE_D: BitBoard = BitBoard(0x0101_0101_0101_0101 << 3);
    #[allow(dead_code)]
    pub const FILE_E: BitBoard = BitBoard(0x0101_0101_0101_0101 << 4);
    #[allow(dead_code)]
    pub const FILE_F: BitBoard = BitBoard(0x0101_0101_0101_0101 << 5);
    #[allow(dead_code)]
    pub const FILE_G: BitBoard = BitBoard(0x0101_0101_0101_0101 << 6);
    #[allow(dead_code)]
    pub const FILE_H: BitBoard = BitBoard(0x0101_0101_0101_0101 << 7);

    #[allow(dead_code)]
    pub const RANK_1: BitBoard = BitBoard(0x0000_0000_0000_00FF);
    #[allow(dead_code)]
    pub const RANK_2: BitBoard = BitBoard(0x0000_0000_0000_00FF << 8);
    #[allow(dead_code)]
    pub const RANK_3: BitBoard = BitBoard(0x0000_0000_0000_00FF << (8 * 2));
    #[allow(dead_code)]
    pub const RANK_4: BitBoard = BitBoard(0x0000_0000_0000_00FF << (8 * 3));
    #[allow(dead_code)]
    pub const RANK_5: BitBoard = BitBoard(0x0000_0000_0000_00FF << (8 * 4));
    #[allow(dead_code)]
    pub const RANK_6: BitBoard = BitBoard(0x0000_0000_0000_00FF << (8 * 5));
    #[allow(dead_code)]
    pub const RANK_7: BitBoard = BitBoard(0x0000_0000_0000_00FF << (8 * 6));
    #[allow(dead_code)]
    pub const RANK_8: BitBoard = BitBoard(0x0000_0000_0000_00FF << (8 * 7));

    #[allow(dead_code)]
    pub const EDGES: BitBoard = BitBoard(0xFF81_8181_8181_81FF);

    #[allow(dead_code)]
    pub const WHITE_OCCUPANY_INIT: BitBoard = BitBoard(0x0000_0000_0000_FFFF);
    #[allow(dead_code)]
    pub const WHITE_PAWN_INIT: BitBoard = BitBoard(0x0000_0000_0000_FF00);
    #[allow(dead_code)]
    pub const WHITE_KNIGHT_INIT: BitBoard = BitBoard(0x0000_0000_0000_0042);
    #[allow(dead_code)]
    pub const WHITE_BISHOP_INIT: BitBoard = BitBoard(0x0000_0000_0000_0024);
    #[allow(dead_code)]
    pub const WHITE_ROOK_INIT: BitBoard = BitBoard(0x0000_0000_0000_0081);
    #[allow(dead_code)]
    pub const WHITE_QUEEN_INIT: BitBoard = BitBoard(0x0000_0000_0000_0008);
    #[allow(dead_code)]
    pub const WHITE_KING_INIT: BitBoard = BitBoard(0x0000_0000_0000_0010);

    #[allow(dead_code)]
    pub const BLACK_OCCUPANY_INIT: BitBoard = BitBoard(0xFFFF_0000_0000_0000);
    #[allow(dead_code)]
    pub const BLACK_PAWN_INIT: BitBoard = BitBoard(0x00FF_0000_0000_0000);
    #[allow(dead_code)]
    pub const BLACK_KNIGHT_INIT: BitBoard = BitBoard(0x4200_0000_0000_0000);
    #[allow(dead_code)]
    pub const BLACK_BISHOP_INIT: BitBoard = BitBoard(0x2400_0000_0000_0000);
    #[allow(dead_code)]
    pub const BLACK_ROOK_INIT: BitBoard = BitBoard(0x8100_0000_0000_0000);
    #[allow(dead_code)]
    pub const BLACK_QUEEN_INIT: BitBoard = BitBoard(0x0800_0000_0000_0000);
    #[allow(dead_code)]
    pub const BLACK_KING_INIT: BitBoard = BitBoard(0x1000_0000_0000_0000);
}
