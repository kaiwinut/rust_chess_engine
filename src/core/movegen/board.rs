use std::fmt;

use crate::core::utils::grid_to_string;

use super::{
    super::super::core::{masks, BitBoard, Square},
    movescan::{scan_pawn_moves, scan_piece_moves, Move, MoveFlags},
};

pub struct Board {
    pub state: [Piece; 64],
    pub pieces: [BitBoard; 12],
    pub occupancy: [BitBoard; 2],
    pub color_to_move: Color,
    pub captured_pieces: Vec<Piece>,
}

impl Board {
    #[allow(dead_code)]
    pub fn new(empty: bool) -> Self {
        if empty {
            Board {
                state: [Piece::EMPTY; 64],
                pieces: [BitBoard(masks::EMPTY); 12],
                occupancy: [BitBoard(masks::EMPTY); 2],
                color_to_move: Color::WHITE,
                captured_pieces: Vec::new(),
            }
        } else {
            Board {
                #[rustfmt::skip]
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
                #[rustfmt::skip]
                pieces: [
                    BitBoard(0x0000_0000_0000_ff00),
                    BitBoard(0x0000_0000_0000_0042),
                    BitBoard(0x0000_0000_0000_0024),
                    BitBoard(0x0000_0000_0000_0081),
                    BitBoard(0x0000_0000_0000_0010),
                    BitBoard(0x0000_0000_0000_0008),
                    BitBoard(0x00ff_0000_0000_0000),
                    BitBoard(0x4200_0000_0000_0000),
                    BitBoard(0x2400_0000_0000_0000),
                    BitBoard(0x8100_0000_0000_0000),
                    BitBoard(0x1000_0000_0000_0000),
                    BitBoard(0x0800_0000_0000_0000),
                ],
                #[rustfmt::skip]
                occupancy: [
                    BitBoard(0x0000_0000_0000_ffff),
                    BitBoard(0xffff_0000_0000_0000),
                ],
                color_to_move: Color::WHITE,
                captured_pieces: Vec::with_capacity(16),
            }
        }
    }

    #[allow(dead_code)]
    pub fn get_moves(&self, moves: &mut [Move], color: Color) -> usize {
        let mut index = 0;

        index = scan_pawn_moves(self, color, moves, index);

        if color == Color::WHITE {
            index = scan_piece_moves(self, color, Piece::WN, moves, index);
            index = scan_piece_moves(self, color, Piece::WB, moves, index);
            index = scan_piece_moves(self, color, Piece::WR, moves, index);
            index = scan_piece_moves(self, color, Piece::WQ, moves, index);
            index = scan_piece_moves(self, color, Piece::WK, moves, index);
        } else {
            index = scan_piece_moves(self, color, Piece::BN, moves, index);
            index = scan_piece_moves(self, color, Piece::BB, moves, index);
            index = scan_piece_moves(self, color, Piece::BR, moves, index);
            index = scan_piece_moves(self, color, Piece::BQ, moves, index);
            index = scan_piece_moves(self, color, Piece::BK, moves, index);
        }

        index
    }

    #[allow(dead_code)]
    pub fn make_move(&mut self, m: &Move) {
        let from = m.from();
        let to = m.to();
        let flags = m.flags();
        let piece = self.piece_at_square(from);

        match flags {
            MoveFlags::QUIET => {
                self.move_piece(from, to, piece);
            }
            MoveFlags::DOUBLE_PUSH => {
                self.move_piece(from, to, piece);
            }
            MoveFlags::CAPTURE => {
                let captured_piece = self.piece_at_square(to);
                assert_ne!(captured_piece, Piece::EMPTY);

                self.captured_pieces.push(captured_piece);

                self.remove_piece_from_square(piece, to);
                self.move_piece(from, to, piece);
            }
            _ => panic!("Invalid flag: {:?}", flags),
        }

        self.color_to_move = self.color_to_move.enemy();
    }

    #[allow(dead_code)]
    pub fn undo_move(&mut self, m: &Move) {
        let from = m.from();
        let to = m.to();
        let flags = m.flags();
        let piece = self.piece_at_square(to);

        match flags {
            MoveFlags::QUIET => {
                self.move_piece(to, from, piece);
            }
            MoveFlags::DOUBLE_PUSH => {
                self.move_piece(to, from, piece);
            }
            MoveFlags::CAPTURE => {
                let captured_piece = self.captured_pieces.pop().unwrap();
                assert_ne!(captured_piece, Piece::EMPTY);

                self.move_piece(to, from, piece);
                self.add_piece_to_square(captured_piece, to);
            }
            _ => panic!("Invalid flag: {:?}", flags),
        }

        self.color_to_move = self.color_to_move.enemy();
    }

    #[allow(dead_code)]
    pub fn piece_at_square(&self, sq: Square) -> Piece {
        self.state[sq.to_usize()]
    }

    #[allow(dead_code)]
    pub fn add_piece_to_square(&mut self, piece: Piece, square: Square) {
        let color = if (0..=5).contains(&piece.to_u8()) {Color::WHITE} else {Color::BLACK};

        self.pieces[piece.to_usize()] |= BitBoard::new(square);
        self.occupancy[color.to_usize()] |= BitBoard::new(square);
        self.state[square.to_usize()] = piece;
    }

    #[allow(dead_code)]
    pub fn remove_piece_from_square(&mut self, piece: Piece, square: Square) {
        let color = if (0..=5).contains(&piece.to_u8()) {Color::WHITE} else {Color::BLACK};

        self.pieces[piece.to_usize()] &= !BitBoard::new(square);
        self.occupancy[color.to_usize()] &= !BitBoard::new(square);
        self.state[square.to_usize()] = Piece::EMPTY;
    }

    #[allow(dead_code)]
    pub fn move_piece(&mut self, from: Square, to: Square, piece: Piece) {
        let color = if (0..=5).contains(&piece.to_u8()) {Color::WHITE} else {Color::BLACK};

        self.pieces[piece.to_usize()] ^= BitBoard::new(from) | BitBoard::new(to);
        self.occupancy[color.to_usize()] ^= BitBoard::new(from) | BitBoard::new(to);
        self.state[to.to_usize()] = piece;
        self.state[from.to_usize()] = Piece::EMPTY;
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", grid_to_string(|square: Square| -> &str {
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
        }, None))
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
