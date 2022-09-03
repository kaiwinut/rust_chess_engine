use bitflags::bitflags;
use std::fmt;

use super::movegen::{self, movescan, Move, MoveFlags};
use super::utils::grid_to_string;
use super::{fen, square, BitBoard, Square};

#[derive(Debug, PartialEq, Eq)]
pub struct Board {
    pub state: [Piece; 64],
    pub pieces: [BitBoard; 12],
    pub occupancy: [BitBoard; 2],
    pub color_to_move: Color,
    pub en_passant: BitBoard,
    pub castling_rights: CastlingRights,
    pub captured_pieces_stack: Vec<Piece>,
    pub en_passant_stack: Vec<BitBoard>,
    pub castling_rights_stack: Vec<CastlingRights>,
    pub half_move_clock: usize,
    pub full_move_number: usize,
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Board {
    #[allow(dead_code)]
    #[rustfmt::skip]
    pub fn new() -> Self {
        Board {
            state: [
                Piece::WR, Piece::WN, Piece::WB, Piece::WQ, Piece::WK, Piece::WB, Piece::WN, Piece::WR,
                Piece::WP, Piece::WP, Piece::WP, Piece::WP, Piece::WP, Piece::WP, Piece::WP, Piece::WP,
                Piece::EMPTY, Piece::EMPTY, Piece::EMPTY, Piece::EMPTY, Piece::EMPTY, Piece::EMPTY, Piece::EMPTY, Piece::EMPTY,
                Piece::EMPTY, Piece::EMPTY, Piece::EMPTY, Piece::EMPTY, Piece::EMPTY, Piece::EMPTY, Piece::EMPTY, Piece::EMPTY,
                Piece::EMPTY, Piece::EMPTY, Piece::EMPTY, Piece::EMPTY, Piece::EMPTY, Piece::EMPTY, Piece::EMPTY, Piece::EMPTY,
                Piece::EMPTY, Piece::EMPTY, Piece::EMPTY, Piece::EMPTY, Piece::EMPTY, Piece::EMPTY, Piece::EMPTY, Piece::EMPTY,
                Piece::BP, Piece::BP, Piece::BP, Piece::BP, Piece::BP, Piece::BP, Piece::BP, Piece::BP,
                Piece::BR, Piece::BN, Piece::BB, Piece::BQ, Piece::BK, Piece::BB, Piece::BN, Piece::BR,
            ],
            pieces: [
                BitBoard::WHITE_PAWN_INIT,
                BitBoard::WHITE_KNIGHT_INIT,
                BitBoard::WHITE_BISHOP_INIT,
                BitBoard::WHITE_ROOK_INIT,
                BitBoard::WHITE_QUEEN_INIT,
                BitBoard::WHITE_KING_INIT,
                BitBoard::BLACK_PAWN_INIT,
                BitBoard::BLACK_KNIGHT_INIT,
                BitBoard::BLACK_BISHOP_INIT,
                BitBoard::BLACK_ROOK_INIT,
                BitBoard::BLACK_QUEEN_INIT,
                BitBoard::BLACK_KING_INIT,
            ],
            occupancy: [
                BitBoard::WHITE_OCCUPANY_INIT,
                BitBoard::BLACK_OCCUPANY_INIT,
            ],
            color_to_move: Color::WHITE,
            en_passant: BitBoard::EMPTY,
            castling_rights: CastlingRights::ALL,
            captured_pieces_stack: Vec::with_capacity(16),
            en_passant_stack: Vec::with_capacity(16),
            castling_rights_stack: Vec::with_capacity(16),
            half_move_clock: 0,
            full_move_number: 1,
        }
    }

    pub fn new_empty() -> Self {
        Board {
            state: [Piece::EMPTY; 64],
            pieces: [BitBoard::EMPTY; 12],
            occupancy: [BitBoard::EMPTY; 2],
            color_to_move: Color::WHITE,
            en_passant: BitBoard::EMPTY,
            castling_rights: CastlingRights::NONE,
            captured_pieces_stack: Vec::with_capacity(16),
            en_passant_stack: Vec::with_capacity(16),
            castling_rights_stack: Vec::with_capacity(16),
            half_move_clock: 0,
            full_move_number: 0,
        }
    }

    #[allow(dead_code)]
    pub fn from(fen: &str) -> Result<Self, &'static str> {
        fen::fen_to_board(fen)
    }

    #[allow(dead_code)]
    pub fn from_moves(moves: &[&str]) -> Result<Self, &'static str> {
        let mut board = Board::new();
        for move_str in moves {
            let m = Move::from_string(move_str.trim(), &board)?;
            board.make_move(&m);
        }

        Ok(board)
    }

    #[allow(dead_code)]
    pub fn to_fen(&self) -> String {
        fen::board_to_fen(self)
    }

    pub fn get_moves(&self, moves: &mut [Move]) -> usize {
        self.get_moves_for_color(moves, self.color_to_move)
    }

    #[allow(dead_code)]
    fn get_moves_for_color(&self, moves: &mut [Move], color: Color) -> usize {
        assert_eq!(self.color_to_move, color);

        let mut index = 0;

        index = movescan::scan_pawn_moves(self, color, moves, index);

        if color == Color::WHITE {
            index = movescan::scan_piece_moves(self, Piece::WN, moves, index);
            index = movescan::scan_piece_moves(self, Piece::WB, moves, index);
            index = movescan::scan_piece_moves(self, Piece::WR, moves, index);
            index = movescan::scan_piece_moves(self, Piece::WQ, moves, index);
            index = movescan::scan_piece_moves(self, Piece::WK, moves, index);
        } else {
            index = movescan::scan_piece_moves(self, Piece::BN, moves, index);
            index = movescan::scan_piece_moves(self, Piece::BB, moves, index);
            index = movescan::scan_piece_moves(self, Piece::BR, moves, index);
            index = movescan::scan_piece_moves(self, Piece::BQ, moves, index);
            index = movescan::scan_piece_moves(self, Piece::BK, moves, index);
        }

        index
    }

    #[allow(dead_code)]
    pub fn make_move(&mut self, m: &Move) {
        let from = m.from();
        let to = m.to();
        let flags = m.flags();
        let piece = self.piece_at_square(from);
        let is_white = piece.color() == Color::WHITE;

        self.castling_rights_stack.push(self.castling_rights);
        self.en_passant_stack.push(self.en_passant);
        self.en_passant = BitBoard::EMPTY;

        match flags {
            MoveFlags::QUIET => {
                self.move_piece(from, to, piece);
            }
            MoveFlags::DOUBLE_PUSH => {
                self.move_piece(from, to, piece);
                self.en_passant =
                    BitBoard::new(Square((to.to_i8() + if is_white { -8 } else { 8 }) as u8))
            }
            MoveFlags::CAPTURE => {
                let captured_piece = self.piece_at_square(to);
                assert_ne!(captured_piece, Piece::EMPTY);

                self.captured_pieces_stack.push(captured_piece);

                self.remove_piece_from_square(captured_piece, to);
                self.move_piece(from, to, piece);
            }
            MoveFlags::EN_PASSANT => {
                self.move_piece(from, to, piece);
                self.remove_piece_from_square(
                    if is_white { Piece::BP } else { Piece::WP },
                    Square((to.to_i8() + if is_white { -8 } else { 8 }) as u8),
                );
            }
            MoveFlags::SHORT_CASTLE => match is_white {
                true => {
                    self.move_piece(square::E1, square::G1, Piece::WK);
                    self.move_piece(square::H1, square::F1, Piece::WR);
                }
                false => {
                    self.move_piece(square::E8, square::G8, Piece::BK);
                    self.move_piece(square::H8, square::F8, Piece::BR);
                }
            },
            MoveFlags::LONG_CASTLE => match is_white {
                true => {
                    self.move_piece(square::E1, square::C1, Piece::WK);
                    self.move_piece(square::A1, square::D1, Piece::WR);
                }
                false => {
                    self.move_piece(square::E8, square::C8, Piece::BK);
                    self.move_piece(square::A8, square::D8, Piece::BR);
                }
            },
            _ => {
                // All promotions contains this bit
                if flags.contains(MoveFlags::PROMOTE_TO_KNIGHT) {
                    let promotion = m.promotion(is_white);

                    // All promotion captures contains this bit
                    if flags.contains(MoveFlags::CAPTURE) {
                        let captured_piece = self.piece_at_square(to);
                        assert_ne!(captured_piece, Piece::EMPTY);
                        self.captured_pieces_stack.push(captured_piece);
                        self.remove_piece_from_square(captured_piece, to);
                    }

                    self.remove_piece_from_square(piece, from);
                    self.add_piece_to_square(promotion, to);
                } else {
                    panic!("Ivalid flag: {:?}", flags);
                }
            }
        }

        if piece == Piece::WK {
            self.castling_rights &= !CastlingRights::WHITE_CASTLE;
        }
        if piece == Piece::BK {
            self.castling_rights &= !CastlingRights::BLACK_CASTLE;
        }
        if piece == Piece::WR {
            match from {
                square::A1 => {
                    self.castling_rights &= !CastlingRights::WHITE_LONG_CASTLE;
                }
                square::H1 => {
                    self.castling_rights &= !CastlingRights::WHITE_SHORT_CASTLE;
                }
                _ => {}
            }
        }
        if piece == Piece::BR {
            match from {
                square::A8 => {
                    self.castling_rights &= !CastlingRights::BLACK_LONG_CASTLE;
                }
                square::H8 => {
                    self.castling_rights &= !CastlingRights::BLACK_SHORT_CASTLE;
                }
                _ => {}
            }
        }

        self.color_to_move = self.color_to_move.enemy();
    }

    #[allow(dead_code)]
    pub fn undo_move(&mut self, m: &Move) {
        let from = m.from();
        let to = m.to();
        let flags = m.flags();
        let piece = self.piece_at_square(to);
        let is_white = piece.color() == Color::WHITE;

        match flags {
            MoveFlags::QUIET => {
                self.move_piece(to, from, piece);
            }
            MoveFlags::DOUBLE_PUSH => {
                self.move_piece(to, from, piece);
            }
            MoveFlags::CAPTURE => {
                self.move_piece(to, from, piece);

                let captured_piece = self.captured_pieces_stack.pop().unwrap();
                assert_ne!(captured_piece, Piece::EMPTY);
                self.add_piece_to_square(captured_piece, to);
            }
            MoveFlags::EN_PASSANT => {
                self.move_piece(to, from, piece);
                self.add_piece_to_square(
                    if is_white { Piece::BP } else { Piece::WP },
                    Square((to.to_i8() + if is_white { -8 } else { 8 }) as u8),
                );
            }
            MoveFlags::SHORT_CASTLE => match is_white {
                true => {
                    self.move_piece(square::G1, square::E1, Piece::WK);
                    self.move_piece(square::F1, square::H1, Piece::WR);
                }
                false => {
                    self.move_piece(square::G8, square::E8, Piece::BK);
                    self.move_piece(square::F8, square::H8, Piece::BR);
                }
            },
            MoveFlags::LONG_CASTLE => match is_white {
                true => {
                    self.move_piece(square::C1, square::E1, Piece::WK);
                    self.move_piece(square::D1, square::A1, Piece::WR);
                }
                false => {
                    self.move_piece(square::C8, square::E8, Piece::BK);
                    self.move_piece(square::D8, square::A8, Piece::BR);
                }
            },
            _ => {
                // All promotions contains this bit
                if flags.contains(MoveFlags::PROMOTE_TO_KNIGHT) {
                    let promotion = piece;
                    self.remove_piece_from_square(promotion, to);
                    self.add_piece_to_square(if is_white { Piece::WP } else { Piece::BP }, from);

                    // All promotion captures contains this bit
                    if flags.contains(MoveFlags::CAPTURE) {
                        let captured_piece = self.captured_pieces_stack.pop().unwrap();
                        assert_ne!(captured_piece, Piece::EMPTY);
                        self.add_piece_to_square(captured_piece, to);
                    }
                } else {
                    panic!("Ivalid flag: {:?}", flags);
                }
            }
        }

        self.castling_rights = self.castling_rights_stack.pop().unwrap();
        self.en_passant = self.en_passant_stack.pop().unwrap();
        self.color_to_move = self.color_to_move.enemy();
    }

    #[allow(dead_code)]
    pub fn is_king_in_check(&self, color: Color) -> bool {
        assert_eq!(color.enemy(), self.color_to_move);
        let king_square = if color == Color::WHITE {
            Square(self.pieces[Piece::WK.to_usize()].bit_scan())
        } else {
            Square(self.pieces[Piece::BK.to_usize()].bit_scan())
        };
        self.is_sqaure_attacked(king_square, color)
    }

    #[allow(dead_code)]
    pub fn all_occupancy(&self) -> BitBoard {
        self.occupancy[Color::WHITE.to_usize()] | self.occupancy[Color::BLACK.to_usize()]
    }

    #[allow(dead_code)]
    pub fn are_squares_empty(&self, squares: &[Square]) -> bool {
        (self.all_occupancy() & BitBoard::from_squares(squares)).is_empty()
    }

    #[allow(dead_code)]
    pub fn can_castle_short(&self) -> bool {
        self.can_castle_short_for_color(self.color_to_move)
    }

    #[allow(dead_code)]
    pub fn can_castle_long(&self) -> bool {
        self.can_castle_long_for_color(self.color_to_move)
    }

    #[allow(dead_code)]
    fn can_castle_short_for_color(&self, color: Color) -> bool {
        assert_eq!(color, self.color_to_move);
        match color {
            Color::WHITE => self
                .castling_rights
                .contains(CastlingRights::WHITE_SHORT_CASTLE),
            Color::BLACK => self
                .castling_rights
                .contains(CastlingRights::BLACK_SHORT_CASTLE),
            _ => panic!("Invalid color when checking can castle short"),
        }
    }

    #[allow(dead_code)]
    fn can_castle_long_for_color(&self, color: Color) -> bool {
        assert_eq!(color, self.color_to_move);
        match color {
            Color::WHITE => self
                .castling_rights
                .contains(CastlingRights::WHITE_LONG_CASTLE),
            Color::BLACK => self
                .castling_rights
                .contains(CastlingRights::BLACK_LONG_CASTLE),
            _ => panic!("Invalid color when checking can castle long"),
        }
    }

    #[allow(dead_code)]
    pub fn piece_at_square(&self, sq: Square) -> Piece {
        self.state[sq.to_usize()]
    }

    #[allow(dead_code)]
    pub fn add_piece_to_square(&mut self, piece: Piece, square: Square) {
        let color = piece.color();

        self.pieces[piece.to_usize()] |= BitBoard::new(square);
        self.occupancy[color.to_usize()] |= BitBoard::new(square);
        self.state[square.to_usize()] = piece;
    }

    #[allow(dead_code)]
    pub fn remove_piece_from_square(&mut self, piece: Piece, square: Square) {
        let color = piece.color();

        self.pieces[piece.to_usize()] &= !BitBoard::new(square);
        self.occupancy[color.to_usize()] &= !BitBoard::new(square);
        self.state[square.to_usize()] = Piece::EMPTY;
    }

    #[allow(dead_code)]
    pub fn move_piece(&mut self, from: Square, to: Square, piece: Piece) {
        let color = piece.color();

        self.pieces[piece.to_usize()] ^= BitBoard::new(from) | BitBoard::new(to);
        self.occupancy[color.to_usize()] ^= BitBoard::new(from) | BitBoard::new(to);
        self.state[to.to_usize()] = piece;
        self.state[from.to_usize()] = Piece::EMPTY;
    }

    #[allow(dead_code)]
    pub fn are_squares_attacked(&self, squares: &[Square], color: Color) -> bool {
        for sq in squares {
            if self.is_sqaure_attacked(*sq, color) {
                return true;
            }
        }
        false
    }

    #[allow(dead_code)]
    fn is_sqaure_attacked(&self, square: Square, color: Color) -> bool {
        let enemy = color.enemy();
        let enemy_rook = if enemy == Color::WHITE {
            Piece::WR.to_usize()
        } else {
            Piece::BR.to_usize()
        };
        let enemy_bishop = if enemy == Color::WHITE {
            Piece::WB.to_usize()
        } else {
            Piece::BB.to_usize()
        };
        let enemy_queen = if enemy == Color::WHITE {
            Piece::WQ.to_usize()
        } else {
            Piece::BQ.to_usize()
        };
        let enemy_knight = if enemy == Color::WHITE {
            Piece::WN.to_usize()
        } else {
            Piece::BN.to_usize()
        };
        let enemy_king = if enemy == Color::WHITE {
            Piece::WK.to_usize()
        } else {
            Piece::BK.to_usize()
        };
        let enemy_pawn = if enemy == Color::WHITE {
            Piece::WP.to_usize()
        } else {
            Piece::BP.to_usize()
        };

        let all_occupancy =
            self.occupancy[Color::WHITE.to_usize()] | self.occupancy[Color::BLACK.to_usize()];

        if (movegen::get_rook_attacks(square, all_occupancy)
            & (self.pieces[enemy_rook] | self.pieces[enemy_queen]))
            .is_not_empty()
        {
            return true;
        }
        if (movegen::get_bishop_attacks(square, all_occupancy)
            & (self.pieces[enemy_bishop] | self.pieces[enemy_queen]))
            .is_not_empty()
        {
            return true;
        }
        if (movegen::get_knight_attacks(square) & self.pieces[enemy_knight]).is_not_empty() {
            return true;
        }
        if (movegen::get_king_attacks(square) & self.pieces[enemy_king]).is_not_empty() {
            return true;
        }

        let potential_attacking_pawns = movegen::get_king_attacks(square) & self.pieces[enemy_pawn];
        let attacking_pawns = match enemy {
            Color::WHITE => {
                BitBoard::new(square)
                    & ((potential_attacking_pawns << 7u8) | (potential_attacking_pawns << 9u8))
            }
            Color::BLACK => {
                BitBoard::new(square)
                    & ((potential_attacking_pawns >> 7u8) | (potential_attacking_pawns >> 9u8))
            }
            _ => panic!(
                "Invalid color when checking attacked squares, color : {}",
                color
            ),
        };

        if attacking_pawns.is_not_empty() {
            return true;
        }

        false
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            grid_to_string(
                |square: Square| -> &str {
                    match self.piece_at_square(square) {
                        Piece::WP => "♙ ",
                        Piece::WN => "♘ ",
                        Piece::WB => "♗ ",
                        Piece::WR => "♖ ",
                        Piece::WQ => "♕ ",
                        Piece::WK => "♔ ",
                        Piece::BP => "♟ ",
                        Piece::BN => "♞ ",
                        Piece::BB => "♝ ",
                        Piece::BR => "♜ ",
                        Piece::BQ => "♛ ",
                        Piece::BK => "♚ ",
                        _ => ". ",
                    }
                },
                None
            )
        )
    }
}

bitflags! {
    pub struct CastlingRights: u8 {
        const NONE = 0b0000;
        const WHITE_SHORT_CASTLE = 0b0001;
        const WHITE_LONG_CASTLE = 0b0010;
        const BLACK_SHORT_CASTLE = 0b0100;
        const BLACK_LONG_CASTLE = 0b1000;

        const WHITE_CASTLE = Self::WHITE_SHORT_CASTLE.bits | Self::WHITE_LONG_CASTLE.bits;
        const BLACK_CASTLE = Self::BLACK_SHORT_CASTLE.bits | Self::BLACK_LONG_CASTLE.bits;
        const ALL = Self::WHITE_CASTLE.bits | Self::BLACK_CASTLE.bits;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color(u8);

impl Color {
    #[allow(dead_code)]
    pub const WHITE: Color = Color(0);
    #[allow(dead_code)]
    pub const BLACK: Color = Color(1);

    #[allow(dead_code)]
    pub fn to_u8(self) -> u8 {
        self.0
    }

    #[allow(dead_code)]
    pub fn to_usize(self) -> usize {
        self.0 as usize
    }

    #[allow(dead_code)]
    pub fn enemy(self) -> Color {
        if self.0 == 0 {
            Color::BLACK
        } else {
            Color::WHITE
        }
    }

    #[allow(dead_code)]
    pub fn as_string(&self) -> &str {
        match self.0 {
            0 => "White",
            1 => "Black",
            _ => "Invalid Color",
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Piece(u8);

impl Piece {
    #[allow(dead_code)]
    pub const WP: Piece = Piece(0);
    #[allow(dead_code)]
    pub const WN: Piece = Piece(1);
    #[allow(dead_code)]
    pub const WB: Piece = Piece(2);
    #[allow(dead_code)]
    pub const WR: Piece = Piece(3);
    #[allow(dead_code)]
    pub const WQ: Piece = Piece(4);
    #[allow(dead_code)]
    pub const WK: Piece = Piece(5);
    #[allow(dead_code)]
    pub const BP: Piece = Piece(6);
    #[allow(dead_code)]
    pub const BN: Piece = Piece(7);
    #[allow(dead_code)]
    pub const BB: Piece = Piece(8);
    #[allow(dead_code)]
    pub const BR: Piece = Piece(9);
    #[allow(dead_code)]
    pub const BQ: Piece = Piece(10);
    #[allow(dead_code)]
    pub const BK: Piece = Piece(11);
    #[allow(dead_code)]
    pub const EMPTY: Piece = Piece(u8::MAX);

    #[allow(dead_code)]
    pub fn to_u8(self) -> u8 {
        self.0
    }

    #[allow(dead_code)]
    pub fn to_usize(self) -> usize {
        self.0 as usize
    }

    #[allow(dead_code)]
    pub fn color(self) -> Color {
        if (0..=5).contains(&self.0) {
            Color::WHITE
        } else {
            Color::BLACK
        }
    }

    #[allow(dead_code)]
    pub fn as_string(&self) -> &str {
        match self.0 {
            0 => "White Pawn",
            1 => "White Knight",
            2 => "White Bishop",
            3 => "White Rook",
            4 => "White Queen",
            5 => "White King",
            6 => "Black Pawn",
            7 => "Black Knight",
            8 => "Black Bishop",
            9 => "Black Rook",
            10 => "Black Queen",
            11 => "Black King",
            _ => "Empty Sqaure",
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_string())
    }
}
