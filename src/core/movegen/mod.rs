pub mod movescan;
pub mod non_sliding;
pub mod sliding;

pub use non_sliding::{get_king_attacks, get_knight_attacks, init_non_sliding_attacks};
pub use sliding::{get_bishop_attacks, get_queen_attacks, get_rook_attacks, init_magic};
pub use movescan::{scan_pawn_moves, scan_piece_moves, Move, MoveFlags};

pub fn init() {
    init_magic();
    init_non_sliding_attacks();
}
