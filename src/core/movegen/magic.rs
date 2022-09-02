use crate::core::{masks, Square, BitBoard};
use super::*;

#[allow(dead_code)]
pub fn find_rook_magics() {
    println!("[");
    for index in 0..64 {
        let magic = find_magic_for_rook_at_sqaure(Square(index));
        println!("{:#018x},", magic.0);
    }
    println!("]");
}

#[allow(dead_code)]
pub fn find_bishop_magics() {
    println!("[");
    for index in 0..64 {
        let magic = find_magic_for_bishop_at_sqaure(Square(index));
        println!("{:#018x},", magic.0);
    }
    println!("]");
}

#[allow(dead_code)]
fn find_magic_for_bishop_at_sqaure(sq: Square) -> BitBoard {
    let shift = BISHOP_SHIFTS[sq.to_usize()];
    let count = (1 << shift) as usize;
    let mask = generate_relevant_occupancy_mask_at_square(sq, true);

    let mut relevant_occupancies = Vec::with_capacity(count as usize);
    let mut attacks = Vec::with_capacity(count as usize);

    for index in 0..count {
        let occupancy = generate_occupancy(mask, index);
        let attack = generate_bishop_attacks(sq, occupancy);
        relevant_occupancies.push(occupancy);
        attacks.push(attack);
    }

    find_magic_for_square(&relevant_occupancies, &attacks, count, shift)
}

#[allow(dead_code)]
fn find_magic_for_rook_at_sqaure(sq: Square) -> BitBoard {
    let shift = ROOK_SHIFTS[sq.to_usize()];
    let count = (1 << shift) as usize;
    let mask = generate_relevant_occupancy_mask_at_square(sq, false);

    let mut relevant_occupancies = Vec::with_capacity(count as usize);
    let mut attacks = Vec::with_capacity(count as usize);

    for index in 0..count {
        let occupancy = generate_occupancy(mask, index);
        let attack = generate_rook_attacks(sq, occupancy);
        relevant_occupancies.push(occupancy);
        attacks.push(attack);
    }

    find_magic_for_square(&relevant_occupancies, &attacks, count, shift)
}

#[allow(dead_code)]
fn find_magic_for_square(
    relevant_occupancies: &[BitBoard],
    attacks: &[BitBoard],
    count: usize,
    shift: u8,
) -> BitBoard {
    let mut hashed_attacks = Vec::with_capacity(count);
    hashed_attacks.resize(count, BitBoard(masks::EMPTY));

    let mut found = false;
    let mut magic = BitBoard(masks::EMPTY);

    while !found {
        found = true;
        magic = BitBoard(
            fastrand::u64(1..u64::MAX) & fastrand::u64(1..u64::MAX) & fastrand::u64(1..u64::MAX),
        );

        for index in 0..count {
            let hash = (relevant_occupancies[index] * magic) >> (64 - shift);

            if hashed_attacks[hash.to_usize()] == BitBoard(masks::EMPTY)
                || hashed_attacks[hash.to_usize()] == attacks[index]
            {
                hashed_attacks[hash.to_usize()] = attacks[index];
            } else {
                found = false;
                break;
            }
        }

        if found {
            break;
        }

        hashed_attacks.clear();
        hashed_attacks.resize(count, BitBoard(masks::EMPTY));
    }

    magic
}