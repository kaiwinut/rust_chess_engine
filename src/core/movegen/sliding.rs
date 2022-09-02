use crate::core::masks;
use crate::core::BitBoard;
use crate::core::Square;
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
            relevant_occupancy_mask: BitBoard(masks::EMPTY),
            shift: 0,
            magic: BitBoard(masks::EMPTY),
            attacks: Vec::new(),
        }
    }
}

pub fn get_rook_attacks(sq: Square, occ: BitBoard) -> BitBoard {
    unsafe {
        let table = &ROOK_ATTACKS_TABLE[sq.to_usize()];
        let hash = ((occ & table.relevant_occupancy_mask) * table.magic) >> (64 - table.shift);
        table.attacks[hash.to_usize()]
    }
}

pub fn get_bishop_attacks(sq: Square, occ: BitBoard) -> BitBoard {
    unsafe {
        let table = &BISHOP_ATTACKS_TABLE[sq.to_usize()];
        let hash = ((occ & table.relevant_occupancy_mask) * table.magic) >> (64 - table.shift);
        table.attacks[hash.to_usize()]
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
            BISHOP_SHIFTS[sq.to_usize()]
        } else {
            ROOK_SHIFTS[sq.to_usize()]
        };
        let count = (1 << shift) as usize;
        let mask = generate_relevant_occupancy_mask_at_square(sq, is_bishop);
        let magic = if is_bishop {
            BitBoard(BISHOP_MAGIC_NUMBERS[sq.to_usize()])
        } else {
            BitBoard(ROOK_MAGICS_NUMBERS[sq.to_usize()])
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
            .resize(count, BitBoard(masks::EMPTY));

        for index in 0..count {
            let occupancy = relevant_occupancies[index];
            let attack = attacks[index];

            let hash = (occupancy * magic) >> (64 - shift);
            if attacks_table[sq.to_usize()].attacks[hash.to_usize()] != BitBoard(masks::EMPTY) {
                panic!("Conflict occurred while initializing magic attack table!");
            }

            attacks_table[sq.to_usize()].attacks[hash.to_usize()] = attack;
        }
    }
}

pub fn generate_occupancy(mut mask: BitBoard, mut index: usize) -> BitBoard {
    let mut result = BitBoard(masks::EMPTY);

    while mask != BitBoard(masks::EMPTY) {
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
        generate_bishop_attacks(sq, BitBoard(masks::EMPTY)) & BitBoard(!masks::EDGES)
    } else {
        ((generate_slider_attacks_in_direction(sq, NORTH, BitBoard(masks::EMPTY))
            | generate_slider_attacks_in_direction(sq, SOUTH, BitBoard(masks::EMPTY)))
            & !BitBoard(masks::RANK_1 | masks::RANK_8))
            | ((generate_slider_attacks_in_direction(sq, EAST, BitBoard(masks::EMPTY))
                | generate_slider_attacks_in_direction(sq, WEST, BitBoard(masks::EMPTY)))
                & !BitBoard(masks::FILE_A | masks::FILE_H))
    }
}

pub fn generate_bishop_attacks(sq: Square, occ: BitBoard) -> BitBoard {
    BitBoard(masks::EMPTY)
        | generate_slider_attacks_in_direction(sq, NORTH_EAST, occ)
        | generate_slider_attacks_in_direction(sq, SOUTH_EAST, occ)
        | generate_slider_attacks_in_direction(sq, SOUTH_WEST, occ)
        | generate_slider_attacks_in_direction(sq, NOTH_WEST, occ)
}

pub fn generate_rook_attacks(sq: Square, occ: BitBoard) -> BitBoard {
    BitBoard(masks::EMPTY)
        | generate_slider_attacks_in_direction(sq, NORTH, occ)
        | generate_slider_attacks_in_direction(sq, EAST, occ)
        | generate_slider_attacks_in_direction(sq, SOUTH, occ)
        | generate_slider_attacks_in_direction(sq, WEST, occ)
}

pub fn generate_slider_attacks_in_direction(
    sq: Square,
    direction: Direction,
    occ: BitBoard,
) -> BitBoard {
    let mut result = BitBoard(masks::EMPTY);
    let mut new_sq = sq;

    while let Some(current_square) = Square::from(
        new_sq.file() as i8 + direction.0,
        new_sq.rank() as i8 + direction.1,
    ) {
        result |= BitBoard::new(current_square);
        if result & occ != BitBoard(masks::EMPTY) {
            break;
        }
        new_sq = current_square;
    }

    result
}

// #[allow(dead_code)]
// pub fn find_magics() {
//     println!("[");
//     for index in 0..64 {
//         let magic = find_magic_for_rook_at_sqaure(Square(index));
//         println!("{:#018x},", magic.0);
//     }
//     println!("]");
//     println!("[");
//     for index in 0..64 {
//         let magic = find_magic_for_bishop_at_sqaure(Square(index));
//         println!("{:#018x},", magic.0);
//     }
//     println!("]");
// }

// #[allow(dead_code)]
// fn find_magic_for_bishop_at_sqaure(sq: Square) -> BitBoard {
//     let shift = BISHOP_SHIFTS[sq.to_usize()];
//     let count = (1 << shift) as usize;
//     let mask = generate_relevant_occupancy_mask_at_square(sq, true);

//     let mut relevant_occupancies = Vec::with_capacity(count as usize);
//     let mut attacks = Vec::with_capacity(count as usize);

//     for index in 0..count {
//         let occupancy = generate_occupancy(mask, index);
//         let attack = generate_bishop_attacks(sq, occupancy);
//         relevant_occupancies.push(occupancy);
//         attacks.push(attack);
//     }

//     find_magic_for_square(&relevant_occupancies, &attacks, count, shift)
// }

// #[allow(dead_code)]
// fn find_magic_for_rook_at_sqaure(sq: Square) -> BitBoard {
//     let shift = ROOK_SHIFTS[sq.to_usize()];
//     let count = (1 << shift) as usize;
//     let mask = generate_relevant_occupancy_mask_at_square(sq, false);

//     let mut relevant_occupancies = Vec::with_capacity(count as usize);
//     let mut attacks = Vec::with_capacity(count as usize);

//     for index in 0..count {
//         let occupancy = generate_occupancy(mask, index);
//         let attack = generate_rook_attacks(sq, occupancy);
//         relevant_occupancies.push(occupancy);
//         attacks.push(attack);
//     }

//     find_magic_for_square(&relevant_occupancies, &attacks, count, shift)
// }

// #[allow(dead_code)]
// fn find_magic_for_square(
//     relevant_occupancies: &[BitBoard],
//     attacks: &[BitBoard],
//     count: usize,
//     shift: u8,
// ) -> BitBoard {
//     let mut hashed_attacks = Vec::with_capacity(count);
//     hashed_attacks.resize(count, BitBoard(masks::EMPTY));

//     let mut found = false;
//     let mut magic = BitBoard(masks::EMPTY);

//     while !found {
//         found = true;
//         magic = BitBoard(
//             fastrand::u64(1..u64::MAX) & fastrand::u64(1..u64::MAX) & fastrand::u64(1..u64::MAX),
//         );

//         for index in 0..count {
//             let hash = (relevant_occupancies[index] * magic) >> (64 - shift);

//             if hashed_attacks[hash.to_usize()] == BitBoard(masks::EMPTY)
//                 || hashed_attacks[hash.to_usize()] == attacks[index]
//             {
//                 hashed_attacks[hash.to_usize()] = attacks[index];
//             } else {
//                 found = false;
//                 break;
//             }
//         }

//         if found {
//             break;
//         }

//         hashed_attacks.clear();
//         hashed_attacks.resize(count, BitBoard(masks::EMPTY));
//     }

//     magic
// }

pub struct Direction(i8, i8);

pub const NORTH: Direction = Direction(0, 1);
pub const EAST: Direction = Direction(1, 0);
pub const SOUTH: Direction = Direction(0, -1);
pub const WEST: Direction = Direction(-1, 0);
pub const NORTH_EAST: Direction = Direction(1, 1);
pub const SOUTH_EAST: Direction = Direction(1, -1);
pub const SOUTH_WEST: Direction = Direction(-1, -1);
pub const NOTH_WEST: Direction = Direction(-1, 1);

#[rustfmt::skip]
pub const ROOK_SHIFTS: [u8; 64] = [
    12, 11, 11, 11, 11, 11, 11, 12, 
    11, 10, 10, 10, 10, 10, 10, 11, 
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11, 
    11, 10, 10, 10, 10, 10, 10, 11, 
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11, 
    12, 11, 11, 11, 11, 11, 11, 12,
];

#[rustfmt::skip]
pub const BISHOP_SHIFTS: [u8; 64] = [
    6, 5, 5, 5, 5, 5, 5, 6, 
    5, 5, 5, 5, 5, 5, 5, 5, 
    5, 5, 7, 7, 7, 7, 5, 5, 
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5, 
    5, 5, 7, 7, 7, 7, 5, 5, 
    5, 5, 5, 5, 5, 5, 5, 5, 
    6, 5, 5, 5, 5, 5, 5, 6,
];

const ROOK_MAGICS_NUMBERS: [u64; 64] = [
    0x008010800020c000,
    0x4040011000a00942,
    0x0100304020010088,
    0x42800e8010001800,
    0x0200100508020020,
    0x1200020028940d10,
    0x0100029401001200,
    0x0100004182006100,
    0x0200800220804000,
    0x0010804001200180,
    0x1200806000100080,
    0x0010807000680080,
    0x48a1000801004410,
    0x4006000408020010,
    0x3201800100620080,
    0x0401800100104080,
    0x4002208000884000,
    0x101008404000a000,
    0x1211110040200100,
    0x4106808008025000,
    0x0004150031000800,
    0x0822008080020400,
    0xa000040009100806,
    0x400016000100a04c,
    0x0008c00480008821,
    0x0008200080804000,
    0x0040200100410010,
    0x0006412200081200,
    0x4300040080800800,
    0x0002001200100508,
    0x8000010400080250,
    0x1000800180004100,
    0x4121304002800280,
    0x100080c00f802000,
    0x0510200480801000,
    0xc801002009005000,
    0x0005080101000491,
    0x0a0c004600800480,
    0x810a000102008408,
    0x0020800040800100,
    0x0080400082208000,
    0x0910201000404002,
    0x0000204082060010,
    0xc080400a00920020,
    0x0010040008008080,
    0x2804000200048080,
    0x0040081007140002,
    0x8000148a44020011,
    0x0004800462c00480,
    0x00004202e1008600,
    0x4000900580200080,
    0x0318100008048080,
    0x000100448801b100,
    0x450a011008340200,
    0x080d000a00148100,
    0x04000504b0440200,
    0x1100302082030046,
    0x6200102102400089,
    0x8c00928900200241,
    0xa04d0008d0012025,
    0x2002000408102002,
    0x4011002400080281,
    0x0040088201100804,
    0x004080208041040a,
];

const BISHOP_MAGIC_NUMBERS: [u64; 64] = [
    0x0082044902120200,
    0x0404500208431002,
    0x4208080060804080,
    0x0004040286211402,
    0x10045040100cc000,
    0x0098862020210001,
    0x0881441008880008,
    0x20c484008a100242,
    0x0000401004010141,
    0x0000060208010104,
    0x3012100182304000,
    0x0810240502000410,
    0xc0004410a8300160,
    0x0080010120100000,
    0xa101440114114404,
    0x0000010c01040210,
    0x640814042008a202,
    0x0022201010011100,
    0x40500108018450d0,
    0x0004100801212020,
    0x1029000090400100,
    0x0810400600d00400,
    0x0280c00401083800,
    0x00b0489080480816,
    0x4842402030500200,
    0x0002200010240090,
    0x00ca90001811a1a0,
    0x8002020020088028,
    0x0803004204004040,
    0x0001004002082002,
    0x0108030404444200,
    0xc002014002030080,
    0x0001044000a01800,
    0x0000840c52105080,
    0x00c4140408420800,
    0x00119a0080280081,
    0x8020440c40240100,
    0x0084100080104810,
    0x18024244012e4401,
    0x4042040504004c44,
    0x00028420a8012000,
    0x4032009004100801,
    0x00044024100b0100,
    0x402000c022041020,
    0x008002022a018c00,
    0x4006500202008020,
    0x0042240812002184,
    0x0010111040880100,
    0x20c1041002a81028,
    0x0020821802820000,
    0x0012002205100300,
    0x0080400284041900,
    0x01008040028a0404,
    0x0004048810010211,
    0x0051026818008000,
    0x8224104400408200,
    0x2204504802101004,
    0x0000008211012000,
    0x00c41a0104016c00,
    0x0000000802420201,
    0x044001c013060200,
    0x1804082044100082,
    0x0000202004010048,
    0x0010420214002208,
];
