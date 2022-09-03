use super::*;
use crate::core::board::*;
use crate::core::{square, BitBoard, Square};

pub fn scan_piece_moves(
    board: &Board,
    piece: Piece,
    moves: &mut [Move],
    mut index: usize,
) -> usize {
    let mut piece_bb = board.pieces[piece.to_usize()];
    let enemy = piece.color().enemy();

    while piece_bb.is_not_empty() {
        let from_bb = piece_bb.lsb();
        let from_square = Square(from_bb.bit_scan());
        piece_bb = piece_bb.pop_lsb();

        let all_occupancy = board.all_occupancy();
        let mut piece_moves = match piece {
            Piece::WN | Piece::BN => get_knight_attacks(from_square),
            Piece::WB | Piece::BB => get_bishop_attacks(from_square, all_occupancy),
            Piece::WR | Piece::BR => get_rook_attacks(from_square, all_occupancy),
            Piece::WQ | Piece::BQ => get_queen_attacks(from_square, all_occupancy),
            Piece::WK | Piece::BK => get_king_attacks(from_square),
            _ => panic!("Invalid piece when getting moves, piece : {}", piece),
        } & !board.occupancy[piece.color().to_usize()];

        while piece_moves.is_not_empty() {
            let to_bb = piece_moves.lsb();
            let to_square = Square(to_bb.bit_scan());
            piece_moves = piece_moves.pop_lsb();

            let is_capture = (to_bb & board.occupancy[enemy.to_usize()]).is_not_empty();
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
                if is_king_side_castle_possible(board, Color::WHITE) {
                    moves[index] = Move::new(square::E1, square::F1, MoveFlags::SHORT_CASTLE);
                    index += 1;
                }

                if is_queen_side_castle_possible(board, Color::WHITE) {
                    moves[index] = Move::new(square::E1, square::C1, MoveFlags::LONG_CASTLE);
                    index += 1;
                }
            }
            Piece::BK => {
                if is_king_side_castle_possible(board, Color::BLACK) {
                    moves[index] = Move::new(square::E8, square::F8, MoveFlags::SHORT_CASTLE);
                    index += 1;
                }

                if is_queen_side_castle_possible(board, Color::BLACK) {
                    moves[index] = Move::new(square::E8, square::C8, MoveFlags::LONG_CASTLE);
                    index += 1;
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
    let all_occupancy = board.all_occupancy();

    let piece_bb = if is_white {
        board.pieces[Piece::WP.to_usize()]
    } else {
        board.pieces[Piece::BP.to_usize()]
    };

    let enemy_backrank = if is_white {
        BitBoard::RANK_8
    } else {
        BitBoard::RANK_1
    };

    let mut pawn_moves = match color {
        Color::WHITE => piece_bb << 8u8,
        Color::BLACK => piece_bb >> 8u8,
        _ => panic!(
            "Invalid color when getting pawn single push moves, color : {}",
            color
        ),
    } & !all_occupancy;

    while pawn_moves.is_not_empty() {
        let to_bb = pawn_moves.lsb();
        let to_square = Square(to_bb.bit_scan());
        let from_square = if is_white {
            to_square.south().unwrap()
        } else {
            to_square.north().unwrap()
        };
        pawn_moves = pawn_moves.pop_lsb();

        if (to_bb & enemy_backrank).is_not_empty() {
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
    let all_occupancy = board.all_occupancy();
    
    let piece_bb = if is_white {
        board.pieces[Piece::WP.to_usize()]
    } else {
        board.pieces[Piece::BP.to_usize()]
    };

    let mut pawn_moves = match color {
        Color::WHITE => (((piece_bb & BitBoard::RANK_2) << 8u8) & !all_occupancy) << 8u8,
        Color::BLACK => (((piece_bb & BitBoard::RANK_7) >> 8u8) & !all_occupancy) >> 8u8,
        _ => panic!(
            "Invalid color when getting pawn double push moves, color : {}",
            color
        ),
    } & !all_occupancy;

    while pawn_moves.is_not_empty() {
        let to_bb = pawn_moves.lsb();
        let to_square = Square(to_bb.bit_scan());
        let from_square = if is_white {
            to_square.south().unwrap().south().unwrap()
        } else {
            to_square.north().unwrap().north().unwrap()
        };
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
    let not_on_file = if is_left {
        !BitBoard::FILE_A
    } else {
        !BitBoard::FILE_H
    };

    let enemy_backrank = if is_white {
        BitBoard::RANK_8
    } else {
        BitBoard::RANK_1
    };

    let mut pawn_moves = match color {
        Color::WHITE => (piece_bb & not_on_file) << (shift as u8),
        Color::BLACK => (piece_bb & not_on_file) >> (shift as u8),
        _ => panic!(
            "Invalid color when getting pawn diagonal attacks, color : {}",
            color
        ),
    } & (board.occupancy[color.enemy().to_usize()] | board.en_passant);

    while pawn_moves.is_not_empty() {
        let to_bb = pawn_moves.lsb();
        let to_square = Square(to_bb.bit_scan());
        let from_square = if is_white && is_left {
            to_square.south_east().unwrap()
        } else if is_white && !is_left {
            to_square.south_west().unwrap()
        } else if !is_white && is_left {
            to_square.north_east().unwrap()
        } else {
            to_square.north_west().unwrap()
        };
        pawn_moves = pawn_moves.pop_lsb();

        if (to_bb & enemy_backrank).is_not_empty() {
            moves[index] = Move::new(from_square, to_square, MoveFlags::PROMOTE_CAPTURE_TO_KNIGHT);
            index += 1;
            moves[index] = Move::new(from_square, to_square, MoveFlags::PROMOTE_CAPTURE_TO_BISHOP);
            index += 1;
            moves[index] = Move::new(from_square, to_square, MoveFlags::PROMOTE_CAPTURE_TO_ROOK);
            index += 1;
            moves[index] = Move::new(from_square, to_square, MoveFlags::PROMOTE_CAPTURE_TO_QUEEN);
            index += 1;
        } else {
            let is_en_passant = (to_bb & board.en_passant).is_not_empty();

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

fn is_king_side_castle_possible(board: &Board, color: Color) -> bool {
    let is_white = color == Color::WHITE;
    let can_short_castle = board.can_castle_short();
    let is_king_side_clear = board.are_squares_empty(if is_white {
        &[square::F1, square::G1]
    } else {
        &[square::F8, square::G8]
    });
    let is_king_side_in_attack = if is_white {
        board.are_squares_attacked(&[square::E1, square::F1, square::G1], Color::WHITE)
    } else {
        board.are_squares_attacked(&[square::E8, square::F8, square::G8], Color::BLACK)
    };
    let is_king_side_rook_exist = if is_white {
        board.piece_at_square(square::H1) == Piece::WR
    } else {
        board.piece_at_square(square::H8) == Piece::BR
    };

    can_short_castle && is_king_side_clear && !is_king_side_in_attack && is_king_side_rook_exist
}

fn is_queen_side_castle_possible(board: &Board, color: Color) -> bool {
    let is_white = color == Color::WHITE;
    let can_long_castle = board.can_castle_long();
    let is_queen_side_clear = board.are_squares_empty(if is_white {
        &[square::D1, square::C1, square::B1]
    } else {
        &[square::D8, square::C8, square::B8]
    });
    let is_queen_side_in_attack = if is_white {
        board.are_squares_attacked(&[square::E1, square::D1, square::C1], Color::WHITE)
    } else {
        board.are_squares_attacked(&[square::E8, square::D8, square::C8], Color::BLACK)
    };
    let is_queen_side_rook_exist = if is_white {
        board.piece_at_square(square::A1) == Piece::WR
    } else {
        board.piece_at_square(square::A8) == Piece::BR
    };

    can_long_castle && is_queen_side_clear && !is_queen_side_in_attack && is_queen_side_rook_exist
}
