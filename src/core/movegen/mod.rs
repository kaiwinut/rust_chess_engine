pub mod movescan;
pub mod non_sliding;
pub mod sliding;

pub use movescan::{scan_pawn_moves, scan_piece_moves, Move, MoveFlags};
pub use non_sliding::*;
pub use sliding::*;

pub fn init() {
    init_magic();
    init_non_sliding_attacks();
}
