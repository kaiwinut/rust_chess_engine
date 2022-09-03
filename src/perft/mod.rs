#![allow(clippy::uninit_assumed_init)]

use super::core::{Board, Color, Move};
use std::mem::MaybeUninit;

pub fn run(depth: u8, board: &mut Board) -> Result<usize, &'static str> {
    let color = board.color_to_move;

    Ok(run_depth(depth, depth, board, color))
}

pub fn run_divided(depth: u8, board: &mut Board) -> Result<Vec<(String, usize)>, &'static str> {
    let mut moves: [Move; 218] = unsafe { MaybeUninit::uninit().assume_init() };
    let moves_count = board.get_moves(&mut moves, board.color_to_move);

    let mut result = Vec::<(String, usize)>::new();
    for m in moves.iter().take(moves_count) {
        board.make_move(m);

        let count = match board.color_to_move {
            Color::WHITE => run_depth(depth - 1, depth, board, board.color_to_move),
            Color::BLACK => run_depth(depth - 1, depth, board, board.color_to_move),
            _ => panic!(
                "Invalid color value when running perft divided, color : {}",
                board.color_to_move
            ),
        };

        result.push((m.as_string(), count));
        board.undo_move(m);
    }

    Ok(result)
}

fn run_depth(depth: u8, max_depth: u8, board: &mut Board, color: Color) -> usize {
    if depth == 0 {
        return 1;
    }

    let mut moves: [Move; 218] = unsafe { MaybeUninit::uninit().assume_init() };
    let moves_count = board.get_moves(&mut moves, color);

    let mut count = 0;

    for m in moves.iter().take(moves_count) {
        board.make_move(m);

        if !board.is_king_in_check(color) {
            count += match color {
                Color::WHITE => run_depth(depth - 1, max_depth, board, Color::BLACK),
                Color::BLACK => run_depth(depth - 1, max_depth, board, Color::WHITE),
                _ => panic!("Invalid color value when running perft, color : {}", color),
            };
        }

        board.undo_move(m);
    }

    count
}
