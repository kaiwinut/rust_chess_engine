#[allow(dead_code)]
pub const EMPTY: u64 = 0u64;

#[allow(dead_code)]
pub const FILE_A: u64 = 0x0101_0101_0101_0101;
#[allow(dead_code)]
pub const FILE_B: u64 = FILE_A << 1;
#[allow(dead_code)]
pub const FILE_C: u64 = FILE_A << 2;
#[allow(dead_code)]
pub const FILE_D: u64 = FILE_A << 3;
#[allow(dead_code)]
pub const FILE_E: u64 = FILE_A << 4;
#[allow(dead_code)]
pub const FILE_F: u64 = FILE_A << 5;
#[allow(dead_code)]
pub const FILE_G: u64 = FILE_A << 6;
#[allow(dead_code)]
pub const FILE_H: u64 = FILE_A << 7;

#[allow(dead_code)]
pub const RANK_1: u64 = 0x0000_0000_0000_00FF;
#[allow(dead_code)]
pub const RANK_2: u64 = RANK_1 << 8;
#[allow(dead_code)]
pub const RANK_3: u64 = RANK_1 << (8 * 2);
#[allow(dead_code)]
pub const RANK_4: u64 = RANK_1 << (8 * 3);
#[allow(dead_code)]
pub const RANK_5: u64 = RANK_1 << (8 * 4);
#[allow(dead_code)]
pub const RANK_6: u64 = RANK_1 << (8 * 5);
#[allow(dead_code)]
pub const RANK_7: u64 = RANK_1 << (8 * 6);
#[allow(dead_code)]
pub const RANK_8: u64 = RANK_1 << (8 * 7);

#[allow(dead_code)]
pub const EDGES: u64 = FILE_A | FILE_H | RANK_1 | RANK_8;
