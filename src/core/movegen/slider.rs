use crate::core::masks;
use crate::core::square::{self, Square};
use crate::core::BitBoard;

pub struct Direction(i32, i32);

pub const NORTH: Direction = Direction(0, 1);
pub const EAST: Direction = Direction(1, 0);
pub const SOUTH: Direction = Direction(0, -1);
pub const WEST: Direction = Direction(-1, 0);
pub const NORTH_EAST: Direction = Direction(1, 1);
pub const SOUTH_EAST: Direction = Direction(1, -1);
pub const SOUTH_WEST: Direction = Direction(-1, -1);
pub const NOTH_WEST: Direction = Direction(-1, 1);

pub fn generate_occupancy(mut mask: BitBoard, mut index: u64) -> BitBoard {
    let mut result = masks::EMPTY;

    while mask != BitBoard(masks::EMPTY) {
        let lsb = mask.lsb();
        let lsb_index = lsb.bit_scan();
        mask = mask.pop_lsb();

        result |= (index & 1) << lsb_index;
        index >>= 1;
    }

    BitBoard(result)
}

fn generate_bishop_attacks(sq: Square, occ: BitBoard) -> BitBoard {
    BitBoard(
        masks::EMPTY
            | generate_slider_attacks_in_direction(sq, NORTH_EAST, occ).0
            | generate_slider_attacks_in_direction(sq, SOUTH_EAST, occ).0
            | generate_slider_attacks_in_direction(sq, SOUTH_WEST, occ).0
            | generate_slider_attacks_in_direction(sq, NOTH_WEST, occ).0,
    )
}

fn generate_rook_attacks(sq: Square, occ: BitBoard) -> BitBoard {
    BitBoard(
        masks::EMPTY
            | generate_slider_attacks_in_direction(sq, NORTH, occ).0
            | generate_slider_attacks_in_direction(sq, EAST, occ).0
            | generate_slider_attacks_in_direction(sq, SOUTH, occ).0
            | generate_slider_attacks_in_direction(sq, WEST, occ).0,
    )
}

fn generate_slider_attacks_in_direction(
    sq: Square,
    direction: Direction,
    occ: BitBoard,
) -> BitBoard {
    let mut result = masks::EMPTY;
    let mut new_sq = sq.clone();

    loop {
        if let Some(current_square) = Square::from(
            new_sq.file() as i32 + direction.0,
            new_sq.rank() as i32 + direction.1,
        ) {
            result |= 1u64 << current_square.0;
            if result & occ.0 != masks::EMPTY {
                break;
            }
            new_sq = current_square;
        } else {
            break;
        }
    }

    BitBoard(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::FILE_A;

    #[test]
    fn test_generate_slider_attacks_in_direction() {
        let sq1 = square::A1;
        let direction1 = NORTH;
        let occ1 = BitBoard(1u64 << square::A7.0);

        let sq2 = square::D4;
        let direction2 = NORTH_EAST;
        let occ2 = BitBoard(1u64 << square::G7.0);

        assert_eq!(
            generate_slider_attacks_in_direction(sq1, direction1, occ1),
            BitBoard(FILE_A.0 - (1u64 << square::A1.0) - (1u64 << square::A8.0))
        );

        assert_eq!(
            generate_slider_attacks_in_direction(sq2, direction2, occ2),
            BitBoard((1u64 << square::E5.0) + (1u64 << square::F6.0) + (1u64 << square::G7.0))
        );
    }
}
