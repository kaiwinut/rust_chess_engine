pub const EMPTY : u64 = 0u64;

pub const FILE_A: u64 = 0x0101_0101_0101_0101;
pub const FILE_B: u64 = FILE_A << 1;
pub const FILE_C: u64 = FILE_A << 2;
pub const FILE_D: u64 = FILE_A << 3;
pub const FILE_E: u64 = FILE_A << 4;
pub const FILE_F: u64 = FILE_A << 5;
pub const FILE_G: u64 = FILE_A << 6;
pub const FILE_H: u64 = FILE_A << 7;

pub const RANK_1: u64 = 0x0000_0000_0000_00FF;
pub const RANK_2: u64 = RANK_1 << 8;
pub const RANK_3: u64 = RANK_1 << 8 * 2;
pub const RANK_4: u64 = RANK_1 << 8 * 3;
pub const RANK_5: u64 = RANK_1 << 8 * 4;
pub const RANK_6: u64 = RANK_1 << 8 * 5;
pub const RANK_7: u64 = RANK_1 << 8 * 6;
pub const RANK_8: u64 = RANK_1 << 8 * 7;