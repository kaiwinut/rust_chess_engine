pub mod sliding;
pub mod non_sliding;

pub use sliding::{init_magic, get_rook_attacks, get_bishop_attacks, get_queen_attacks};
pub use non_sliding::{init_non_sliding_attacks, get_king_attacks, get_knight_attacks};

pub fn init() {
    init_magic();
    init_non_sliding_attacks();
}