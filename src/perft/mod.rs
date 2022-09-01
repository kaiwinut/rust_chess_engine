#![allow(clippy::uninit_assumed_init)]

use super::core::movegen::board::*;
use super::core::movegen::movescan::*;
use std::mem::MaybeUninit;

pub fn run(depth: u8) -> u32 {
    let mut board = Board::new(false);

    search_depth(depth, &mut board, Color::WHITE)
}

fn search_depth(depth: u8, board: &mut Board, color: Color) -> u32 {
    if depth == 0 {
        return 1;
    }

    let mut moves: [Move; 218] = unsafe { MaybeUninit::uninit().assume_init() };
    let moves_count = board.get_moves(&mut moves, color);

    let mut count = 0;
    for m in moves.iter().take(moves_count) {
        // println!("depth {} | no. {} / {}", depth, i+1, moves_count);
        // println!("Move {}", m);

        board.make_move(m);
        // println!("{}", board);
        count += match color {
            Color::WHITE => search_depth(depth - 1, board, Color::BLACK),
            Color::BLACK => search_depth(depth - 1, board, Color::WHITE),
            _ => panic!("Invalid color value when running perft, color : {}", color),
        };
        board.undo_move(m);
    }

    count
}
