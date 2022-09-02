use crate::core::{masks, BitBoard, Square};
use bitflags::bitflags;
use std::fmt;

use super::board::*;
use super::*;

#[derive(Clone, Copy)]
pub struct Move(pub u16);

impl Move {
    #[allow(dead_code)]
    pub fn new(from: Square, to: Square, flags: MoveFlags) -> Self {
        Move(((flags.bits as u16) << 12) | (to.to_u16() << 6) | from.to_u16())
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

pub fn scan_piece_moves(
    board: &Board,
    piece: Piece,
    moves: &mut [Move],
    mut index: usize,
) -> usize {
    let mut piece_bb = board.pieces[piece.to_usize()];
    let enemy = piece.color().enemy();

    while piece_bb != BitBoard(masks::EMPTY) {
        let from_bb = piece_bb.lsb();
        let from_square = Square(from_bb.bit_scan());
        piece_bb = piece_bb.pop_lsb();

        let all_occupancy =
            board.occupancy[Color::WHITE.to_usize()] | board.occupancy[Color::BLACK.to_usize()];
        let mut piece_moves = match piece {
            Piece::WN | Piece::BN => get_knight_attacks(from_square),
            Piece::WB | Piece::BB => get_bishop_attacks(from_square, all_occupancy),
            Piece::WR | Piece::BR => get_rook_attacks(from_square, all_occupancy),
            Piece::WQ | Piece::BQ => get_queen_attacks(from_square, all_occupancy),
            Piece::WK | Piece::BK => get_king_attacks(from_square),
            _ => panic!("Invalid piece when getting moves, piece : {}", piece),
        } & !board.occupancy[piece.color().to_usize()];

        while piece_moves != BitBoard(masks::EMPTY) {
            let to_bb = piece_moves.lsb();
            let to_square = Square(to_bb.bit_scan());
            piece_moves = piece_moves.pop_lsb();

            let is_capture = (to_bb & board.occupancy[enemy.to_usize()]) != BitBoard(masks::EMPTY);
            let flags = if is_capture {
                MoveFlags::CAPTURE
            } else {
                MoveFlags::QUIET
            };

            moves[index] = Move::new(from_square, to_square, flags);
            index += 1;
        }
    }

    index
}

pub fn scan_pawn_moves(board: &Board, color: Color, moves: &mut [Move], mut index: usize) -> usize {
    index = scan_pawn_single_push(board, color, moves, index);
    index = scan_pawn_double_push(board, color, moves, index);
    index = scan_pawn_diagonal_attacks(board, color, true, moves, index);
    index = scan_pawn_diagonal_attacks(board, color, false, moves, index);

    index
}

pub fn scan_pawn_single_push(
    board: &Board,
    color: Color,
    moves: &mut [Move],
    mut index: usize,
) -> usize {
    let is_white = color == Color::WHITE;
    let piece_bb = if is_white {
        board.pieces[Piece::WP.to_usize()]
    } else {
        board.pieces[Piece::BP.to_usize()]
    };
    let all_occupancy =
        board.occupancy[Color::WHITE.to_usize()] | board.occupancy[Color::BLACK.to_usize()];
    let shift = if is_white { 8 } else { -8 };

    let mut pawn_moves = match color {
        Color::WHITE => piece_bb << 8u8,
        Color::BLACK => piece_bb >> 8u8,
        _ => panic!(
            "Invalid color when getting pawn single push moves, color : {}",
            color
        ),
    } & !all_occupancy;

    while pawn_moves != BitBoard(masks::EMPTY) {
        let to_bb = pawn_moves.lsb();
        let to_square = Square(to_bb.bit_scan());
        let from_square = Square((to_bb.bit_scan() as i8 - shift) as u8);
        pawn_moves = pawn_moves.pop_lsb();

        moves[index] = Move::new(from_square, to_square, MoveFlags::QUIET);
        index += 1;
    }

    index
}

pub fn scan_pawn_double_push(
    board: &Board,
    color: Color,
    moves: &mut [Move],
    mut index: usize,
) -> usize {
    let is_white = color == Color::WHITE;
    let piece_bb = if is_white {
        board.pieces[Piece::WP.to_usize()]
    } else {
        board.pieces[Piece::BP.to_usize()]
    };
    let all_occupancy =
        board.occupancy[Color::WHITE.to_usize()] | board.occupancy[Color::BLACK.to_usize()];
    let shift = if is_white { 16 } else { -16 };

    let mut pawn_moves = match color {
        Color::WHITE => (((piece_bb & BitBoard(masks::RANK_2)) << 8u8) & !all_occupancy) << 8u8,
        Color::BLACK => (((piece_bb & BitBoard(masks::RANK_7)) >> 8u8) & !all_occupancy) >> 8u8,
        _ => panic!(
            "Invalid color when getting pawn double push moves, color : {}",
            color
        ),
    } & !all_occupancy;

    while pawn_moves != BitBoard(masks::EMPTY) {
        let to_bb = pawn_moves.lsb();
        let to_square = Square(to_bb.bit_scan());
        let from_square = Square((to_bb.bit_scan() as i8 - shift) as u8);
        pawn_moves = pawn_moves.pop_lsb();

        moves[index] = Move::new(from_square, to_square, MoveFlags::DOUBLE_PUSH);
        index += 1;
    }

    index
}

pub fn scan_pawn_diagonal_attacks(
    board: &Board,
    color: Color,
    is_left: bool,
    moves: &mut [Move],
    mut index: usize,
) -> usize {
    let is_white = color == Color::WHITE;
    let piece_bb = if is_white {
        board.pieces[Piece::WP.to_usize()]
    } else {
        board.pieces[Piece::BP.to_usize()]
    };

    let shift = if is_left ^ is_white { 9 } else { 7 };
    let not_on_file = !BitBoard(if is_left {
        masks::FILE_A
    } else {
        masks::FILE_H
    });

    let mut pawn_moves = match color {
        Color::WHITE => (piece_bb & not_on_file) << (shift as u8),
        Color::BLACK => (piece_bb & not_on_file) >> (shift as u8),
        _ => panic!(
            "Invalid color when getting pawn diagonal attacks, color : {}",
            color
        ),
    } & board.occupancy[color.enemy().to_usize()];

    let shift = if is_white { shift } else { -shift };

    while pawn_moves != BitBoard(masks::EMPTY) {
        let to_bb = pawn_moves.lsb();
        let to_square = Square(to_bb.bit_scan());
        let from_square = Square((to_bb.bit_scan() as i8 - shift) as u8);
        pawn_moves = pawn_moves.pop_lsb();

        moves[index] = Move::new(from_square, to_square, MoveFlags::CAPTURE);
        index += 1;
    }

    index
}

bitflags! {
    pub struct MoveFlags: u8 {
        const QUIET = 0;
        const DOUBLE_PUSH = 1;
        const CASTLE_KING = 2;
        const CASTLE_QUEEN = 3;
        const CAPTURE = 4;
        const EN_PASSANT = 5;
        const UNDEFINED1 = 6;
        const UNDEFINED2 = 7;
        const PROMOTE_TO_KNIGHT = 8;
        const PROMOTE_TO_BISHOP = 9;
        const PROMOTE_TO_ROOK = 10;
        const PROMOTE_TO_QUEEN = 11;
        const PROMOTE_CAPTURE_TO_KNIGHT = 12;
        const PROMOTE_CAPTURE_TO_BISHOP = 13;
        const PROMOTE_CAPTURE_TO_ROOK = 14;
        const PROMOTE_CAPTURE_TO_QUEEN = 15;
    }
}
