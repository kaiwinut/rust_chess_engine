use super::super::masks;
use super::super::BitBoard;
use super::super::Square;

static mut KING_ATTACKS_TABLE: [BitBoard; 64] = [BitBoard(masks::EMPTY); 64];
static mut KNIGHT_ATTACKS_TABLE: [BitBoard; 64] = [BitBoard(masks::EMPTY); 64];

pub fn init_non_sliding_attacks() {
    generate_king_attacks();
    generate_knight_attacks();
}

pub fn get_king_attacks(sq: Square) -> BitBoard {
    unsafe { KING_ATTACKS_TABLE[sq.to_usize()] }
}

pub fn get_knight_attacks(sq: Square) -> BitBoard {
    unsafe { KNIGHT_ATTACKS_TABLE[sq.to_usize()] }
}

fn generate_king_attacks() {
    unsafe {
        for index in 0..64 {
            let bb = BitBoard::new(Square(index));

            KING_ATTACKS_TABLE[index as usize] = BitBoard(masks::EMPTY)
                | ((bb & BitBoard(!masks::FILE_A)) >> 1u8)
                | ((bb & BitBoard(!masks::FILE_A)) >> 9u8)
                | ((bb & BitBoard(!masks::FILE_A)) << 7u8)
                | ((bb & BitBoard(!masks::FILE_H)) << 1u8)
                | ((bb & BitBoard(!masks::FILE_H)) << 9u8)
                | ((bb & BitBoard(!masks::FILE_H)) >> 7u8)
                | ((bb & BitBoard(!masks::RANK_1)) >> 8u8)
                | ((bb & BitBoard(!masks::RANK_8)) << 8u8);
        }
    }
}

fn generate_knight_attacks() {
    unsafe {
        for index in 0..64 {
            let bb = BitBoard::new(Square(index));

            KNIGHT_ATTACKS_TABLE[index as usize] = BitBoard(masks::EMPTY)
                | ((bb & BitBoard(!masks::FILE_G) & BitBoard(!masks::FILE_H)) >> 6u8)
                | ((bb & BitBoard(!masks::FILE_G) & BitBoard(!masks::FILE_H)) << 10u8)
                | ((bb & BitBoard(!masks::FILE_A) & BitBoard(!masks::FILE_B)) >> 10u8)
                | ((bb & BitBoard(!masks::FILE_A) & BitBoard(!masks::FILE_B)) << 6u8)
                | ((bb & BitBoard(!masks::FILE_A)) >> 17u8)
                | ((bb & BitBoard(!masks::FILE_A)) << 15u8)
                | ((bb & BitBoard(!masks::FILE_H)) >> 15u8)
                | ((bb & BitBoard(!masks::FILE_H)) << 17u8);
        }
    }
}
