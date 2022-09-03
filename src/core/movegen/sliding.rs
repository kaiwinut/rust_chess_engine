use super::constants::{self, Direction};
use crate::core::{BitBoard, Square};
use arr_macro::arr;

pub struct MagicAttackTable {
    pub relevant_occupancy_mask: BitBoard,
    pub shift: u8,
    pub magic: BitBoard,
    pub attacks: Vec<BitBoard>,
}

impl MagicAttackTable {
    pub const fn new() -> MagicAttackTable {
        MagicAttackTable {
            relevant_occupancy_mask: BitBoard::EMPTY,
            shift: 0,
            magic: BitBoard::EMPTY,
            attacks: Vec::new(),
        }
    }

    pub fn generate_hash(&self, occ: BitBoard) -> usize {
        (((occ & self.relevant_occupancy_mask) * self.magic) >> (64 - self.shift)).to_usize()
    }
}

pub fn get_rook_attacks(sq: Square, occ: BitBoard) -> BitBoard {
    unsafe {
        let table = &ROOK_ATTACKS_TABLE[sq.to_usize()];
        let hash = table.generate_hash(occ);
        table.attacks[hash]
    }
}

pub fn get_bishop_attacks(sq: Square, occ: BitBoard) -> BitBoard {
    unsafe {
        let table = &BISHOP_ATTACKS_TABLE[sq.to_usize()];
        let hash = table.generate_hash(occ);
        table.attacks[hash]
    }
}

pub fn get_queen_attacks(sq: Square, occ: BitBoard) -> BitBoard {
    get_rook_attacks(sq, occ) | get_bishop_attacks(sq, occ)
}

pub fn init_magic() {
    for index in 0..64 {
        init_attacks_table_for_square(Square(index), false);
        init_attacks_table_for_square(Square(index), true);
    }
}

static mut ROOK_ATTACKS_TABLE: [MagicAttackTable; 64] = arr![MagicAttackTable::new(); 64];
static mut BISHOP_ATTACKS_TABLE: [MagicAttackTable; 64] = arr![MagicAttackTable::new(); 64];

fn init_attacks_table_for_square(sq: Square, is_bishop: bool) {
    unsafe {
        let shift = if is_bishop {
            constants::BISHOP_SHIFTS[sq.to_usize()]
        } else {
            constants::ROOK_SHIFTS[sq.to_usize()]
        };
        let count = (1 << shift) as usize;
        let mask = generate_relevant_occupancy_mask_at_square(sq, is_bishop);
        let magic = if is_bishop {
            BitBoard(constants::BISHOP_MAGIC_NUMBERS[sq.to_usize()])
        } else {
            BitBoard(constants::ROOK_MAGICS_NUMBERS[sq.to_usize()])
        };

        let mut relevant_occupancies = Vec::with_capacity(count as usize);
        let mut attacks = Vec::with_capacity(count as usize);

        for index in 0..count {
            let occupancy = generate_occupancy(mask, index);
            let attack = if is_bishop {
                generate_bishop_attacks(sq, occupancy)
            } else {
                generate_rook_attacks(sq, occupancy)
            };
            relevant_occupancies.push(occupancy);
            attacks.push(attack);
        }

        let mut attacks_table = if is_bishop {
            &mut BISHOP_ATTACKS_TABLE
        } else {
            &mut ROOK_ATTACKS_TABLE
        };

        attacks_table[sq.to_usize()].shift = shift;
        attacks_table[sq.to_usize()].relevant_occupancy_mask = mask;
        attacks_table[sq.to_usize()].magic = magic;
        attacks_table[sq.to_usize()].attacks = Vec::with_capacity(count);
        attacks_table[sq.to_usize()]
            .attacks
            .resize(count, BitBoard::EMPTY);

        for index in 0..count {
            let occupancy = relevant_occupancies[index];
            let attack = attacks[index];

            let hash = (occupancy * magic) >> (64 - shift);
            if attacks_table[sq.to_usize()].attacks[hash.to_usize()].is_not_empty() {
                panic!("Conflict occurred while initializing magic attack table!");
            }

            attacks_table[sq.to_usize()].attacks[hash.to_usize()] = attack;
        }
    }
}

pub fn generate_occupancy(mut mask: BitBoard, mut index: usize) -> BitBoard {
    let mut result = BitBoard::EMPTY;

    while mask.is_not_empty() {
        let lsb = mask.lsb();
        let lsb_index = lsb.bit_scan();
        mask = mask.pop_lsb();

        result |= BitBoard(index as u64 & 1) << lsb_index;
        index >>= 1;
    }

    result
}

pub fn generate_relevant_occupancy_mask_at_square(sq: Square, is_bishop: bool) -> BitBoard {
    if is_bishop {
        generate_bishop_attacks(sq, BitBoard::EMPTY).pop_edges()
    } else {
        let vertical =
            (generate_slider_attacks_in_direction(sq, BitBoard::EMPTY, Direction::North)
                | generate_slider_attacks_in_direction(sq, BitBoard::EMPTY, Direction::South))
            .pop_north_south_edges();
        let horizontal =
            (generate_slider_attacks_in_direction(sq, BitBoard::EMPTY, Direction::East)
                | generate_slider_attacks_in_direction(sq, BitBoard::EMPTY, Direction::West))
            .pop_east_west_edges();

        vertical | horizontal
    }
}

pub fn generate_bishop_attacks(sq: Square, occ: BitBoard) -> BitBoard {
    BitBoard::EMPTY
        | generate_slider_attacks_in_direction(sq, occ, Direction::NorthEast)
        | generate_slider_attacks_in_direction(sq, occ, Direction::SouthEast)
        | generate_slider_attacks_in_direction(sq, occ, Direction::SouthWest)
        | generate_slider_attacks_in_direction(sq, occ, Direction::NorthWest)
}

pub fn generate_rook_attacks(sq: Square, occ: BitBoard) -> BitBoard {
    BitBoard::EMPTY
        | generate_slider_attacks_in_direction(sq, occ, Direction::North)
        | generate_slider_attacks_in_direction(sq, occ, Direction::East)
        | generate_slider_attacks_in_direction(sq, occ, Direction::South)
        | generate_slider_attacks_in_direction(sq, occ, Direction::West)
}

fn generate_slider_attacks_in_direction(
    sq: Square,
    occ: BitBoard,
    direction: Direction,
) -> BitBoard {
    let mut result = BitBoard::EMPTY;
    let mut new_sq = sq;

    while let Some(current_square) = match direction {
        Direction::North => new_sq.north(),
        Direction::NorthEast => new_sq.north_east(),
        Direction::East => new_sq.east(),
        Direction::SouthEast => new_sq.south_east(),
        Direction::South => new_sq.south(),
        Direction::SouthWest => new_sq.south_west(),
        Direction::West => new_sq.west(),
        Direction::NorthWest => new_sq.north_west(),
    } {
        result |= BitBoard::new(current_square);
        if (result & occ).is_not_empty() {
            break;
        }
        new_sq = current_square;
    }

    result
}
