#![allow(clippy::uninit_assumed_init)]

use bitflags::bitflags;
use std::fmt;
use std::mem::MaybeUninit;

use crate::core::{Board, Piece, Square};

#[derive(Clone, Copy)]
pub struct Move(pub u16);

impl Move {
    #[allow(dead_code)]
    pub fn new(from: Square, to: Square, flags: MoveFlags) -> Self {
        Move(((flags.bits as u16) << 12) | (to.to_u16() << 6) | from.to_u16())
    }

    #[allow(dead_code)]
    pub fn from_string(string: &str, board: &Board) -> Result<Move, &'static str> {
        if string.len() < 4 {
            return Err("Invalid move: move string is too short");
        }

        let from = Square::from_string(&string[0..2]).unwrap();
        let to = Square::from_string(&string[2..4]).unwrap();

        let mut moves: [Move; 218] = unsafe { MaybeUninit::uninit().assume_init() };
        let moves_count = board.get_moves(&mut moves, board.color_to_move);

        for m in moves.iter().take(moves_count) {
            if m.from() == from && m.to() == to {
                return Ok(*m);
            }
        }

        Err("Invalid move: move not found in position")
    }

    #[allow(dead_code)]
    pub fn from(&self) -> Square {
        Square((self.0 & 63) as u8)
    }

    #[allow(dead_code)]
    pub fn to(&self) -> Square {
        Square(((self.0 >> 6) & 63) as u8)
    }

    #[allow(dead_code)]
    pub fn flags(&self) -> MoveFlags {
        unsafe { MoveFlags::from_bits_unchecked(((self.0 >> 12) & 15) as u8) }
    }

    #[allow(dead_code)]
    pub fn promotion(&self, is_white: bool) -> Piece {
        match self.flags() {
            MoveFlags::PROMOTE_TO_KNIGHT | MoveFlags::PROMOTE_CAPTURE_TO_KNIGHT => {
                if is_white {
                    Piece::WN
                } else {
                    Piece::BN
                }
            }
            MoveFlags::PROMOTE_TO_BISHOP | MoveFlags::PROMOTE_CAPTURE_TO_BISHOP => {
                if is_white {
                    Piece::WB
                } else {
                    Piece::BB
                }
            }
            MoveFlags::PROMOTE_TO_ROOK | MoveFlags::PROMOTE_CAPTURE_TO_ROOK => {
                if is_white {
                    Piece::WR
                } else {
                    Piece::BR
                }
            }
            MoveFlags::PROMOTE_TO_QUEEN | MoveFlags::PROMOTE_CAPTURE_TO_QUEEN => {
                if is_white {
                    Piece::WQ
                } else {
                    Piece::BQ
                }
            }
            _ => panic!("Move is not promotion, flag : {:?}", self.flags()),
        }
    }

    #[allow(dead_code)]
    pub fn as_string(&self) -> String {
        let from = self.from();
        let to = self.to();

        let string = vec![from.as_string(), to.as_string()];

        string.into_iter().collect()
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({:?})",
            self.from().as_string().to_string() + self.to().as_string(),
            self.flags()
        )
    }
}

bitflags! {
    pub struct MoveFlags: u8 {
        const QUIET = 0b0000;
        const DOUBLE_PUSH = 0b0001;
        const SHORT_CASTLE = 0b0010;
        const LONG_CASTLE = 0b0011;
        const CAPTURE = 0b0100;
        const EN_PASSANT = 0b0101;
        const UNDEFINED1 = 0b0110;
        const UNDEFINED2 = 0b0111;
        const PROMOTE_TO_KNIGHT = 0b1000;
        const PROMOTE_TO_BISHOP = 0b1001;
        const PROMOTE_TO_ROOK = 0b1010;
        const PROMOTE_TO_QUEEN = 0b1011;
        const PROMOTE_CAPTURE_TO_KNIGHT = 0b1100;
        const PROMOTE_CAPTURE_TO_BISHOP = 0b1101;
        const PROMOTE_CAPTURE_TO_ROOK = 0b1110;
        const PROMOTE_CAPTURE_TO_QUEEN = 0b1111;
    }
}
