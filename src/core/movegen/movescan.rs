use crate::core::{masks, square, BitBoard, Square};
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

        match piece {
            Piece::WK => {
                let can_short_castle = board
                    .castling_rights
                    .contains(CastlingRights::WHITE_SHORT_CASTLE);
                let can_long_castle = board
                    .castling_rights
                    .contains(CastlingRights::WHITE_LONG_CASTLE);

                if can_short_castle
                    && (all_occupancy & (BitBoard::new(square::F1) | BitBoard::new(square::G1)))
                        == BitBoard(masks::EMPTY)
                {
                    let is_rook_exist = board.piece_at_square(square::H1) == Piece::WR;
                    let is_in_attack = board.is_sqaure_attacked(square::E1, Color::WHITE)
                        || board.is_sqaure_attacked(square::F1, Color::WHITE)
                        || board.is_sqaure_attacked(square::G1, Color::WHITE);

                    if !is_in_attack && is_rook_exist {
                        moves[index] = Move::new(square::E1, square::F1, MoveFlags::SHORT_CASTLE);
                        index += 1;
                    }
                }

                if can_long_castle
                    && (all_occupancy
                        & (BitBoard::new(square::D1)
                            | BitBoard::new(square::C1)
                            | BitBoard::new(square::B1)))
                        == BitBoard(masks::EMPTY)
                {
                    let is_rook_exist = board.piece_at_square(square::A1) == Piece::WR;
                    let is_in_attack = board.is_sqaure_attacked(square::E1, Color::WHITE)
                        || board.is_sqaure_attacked(square::D1, Color::WHITE)
                        || board.is_sqaure_attacked(square::C1, Color::WHITE);

                    if !is_in_attack && is_rook_exist {
                        moves[index] = Move::new(square::E1, square::C1, MoveFlags::LONG_CASTLE);
                        index += 1;
                    }
                }
            }
            Piece::BK => {
                let can_short_castle = board
                    .castling_rights
                    .contains(CastlingRights::BLACK_SHORT_CASTLE);
                let can_long_castle = board
                    .castling_rights
                    .contains(CastlingRights::BLACK_LONG_CASTLE);

                if can_short_castle
                    && (all_occupancy & (BitBoard::new(square::F8) | BitBoard::new(square::G8)))
                        == BitBoard(masks::EMPTY)
                {
                    let is_rook_exist = board.piece_at_square(square::H8) == Piece::BR;
                    let is_in_attack = board.is_sqaure_attacked(square::E8, Color::BLACK)
                        || board.is_sqaure_attacked(square::F8, Color::BLACK)
                        || board.is_sqaure_attacked(square::G8, Color::BLACK);

                    if !is_in_attack && is_rook_exist {
                        moves[index] = Move::new(square::E8, square::F8, MoveFlags::SHORT_CASTLE);
                        index += 1;
                    }
                }

                if can_long_castle
                    && (all_occupancy
                        & (BitBoard::new(square::D8)
                            | BitBoard::new(square::C8)
                            | BitBoard::new(square::B8)))
                        == BitBoard(masks::EMPTY)
                {
                    let is_rook_exist = board.piece_at_square(square::A8) == Piece::BR;
                    let is_in_attack = board.is_sqaure_attacked(square::E8, Color::BLACK)
                        || board.is_sqaure_attacked(square::D8, Color::BLACK)
                        || board.is_sqaure_attacked(square::C8, Color::BLACK);

                    if !is_in_attack && is_rook_exist {
                        moves[index] = Move::new(square::E8, square::C8, MoveFlags::LONG_CASTLE);
                        index += 1;
                    }
                }
            }
            _ => {}
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

    let enemy_backrank = if is_white {
        BitBoard(masks::RANK_8)
    } else {
        BitBoard(masks::RANK_1)
    };

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

        if to_bb & enemy_backrank != BitBoard(masks::EMPTY) {
            moves[index] = Move::new(from_square, to_square, MoveFlags::PROMOTE_TO_KNIGHT);
            index += 1;
            moves[index] = Move::new(from_square, to_square, MoveFlags::PROMOTE_TO_BISHOP);
            index += 1;
            moves[index] = Move::new(from_square, to_square, MoveFlags::PROMOTE_TO_ROOK);
            index += 1;
            moves[index] = Move::new(from_square, to_square, MoveFlags::PROMOTE_TO_QUEEN);
            index += 1;
        } else {
            moves[index] = Move::new(from_square, to_square, MoveFlags::QUIET);
            index += 1;
        }
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

    let enemy_backrank = if is_white {
        BitBoard(masks::RANK_8)
    } else {
        BitBoard(masks::RANK_1)
    };

    let mut pawn_moves = match color {
        Color::WHITE => (piece_bb & not_on_file) << (shift as u8),
        Color::BLACK => (piece_bb & not_on_file) >> (shift as u8),
        _ => panic!(
            "Invalid color when getting pawn diagonal attacks, color : {}",
            color
        ),
    } & (board.occupancy[color.enemy().to_usize()] | board.en_passant);

    let shift = if is_white { shift } else { -shift };

    while pawn_moves != BitBoard(masks::EMPTY) {
        let to_bb = pawn_moves.lsb();
        let to_square = Square(to_bb.bit_scan());
        let from_square = Square((to_bb.bit_scan() as i8 - shift) as u8);
        pawn_moves = pawn_moves.pop_lsb();

        if to_bb & enemy_backrank != BitBoard(masks::EMPTY) {
            moves[index] = Move::new(from_square, to_square, MoveFlags::PROMOTE_CAPTURE_TO_KNIGHT);
            index += 1;
            moves[index] = Move::new(from_square, to_square, MoveFlags::PROMOTE_CAPTURE_TO_BISHOP);
            index += 1;
            moves[index] = Move::new(from_square, to_square, MoveFlags::PROMOTE_CAPTURE_TO_ROOK);
            index += 1;
            moves[index] = Move::new(from_square, to_square, MoveFlags::PROMOTE_CAPTURE_TO_QUEEN);
            index += 1;
        } else {
            let is_en_passant = (to_bb & board.en_passant) != BitBoard(masks::EMPTY);

            moves[index] = Move::new(
                from_square,
                to_square,
                if is_en_passant {
                    MoveFlags::EN_PASSANT
                } else {
                    MoveFlags::CAPTURE
                },
            );
            index += 1;
        }
    }

    index
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
