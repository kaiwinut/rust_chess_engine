mod constants;
pub mod magic;
pub mod r#move;
pub mod movescan;
pub mod non_sliding;
pub mod sliding;

pub use non_sliding::*;
pub use r#move::{Move, MoveFlags};
pub use sliding::*;

pub fn init() {
    init_magic();
    init_non_sliding_attacks();
}
