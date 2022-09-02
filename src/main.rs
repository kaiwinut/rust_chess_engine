mod core;
mod perft;
mod terminal;
use crate::core::movegen;

fn main() {
    movegen::init();
    terminal::init();
    terminal::run();
}
