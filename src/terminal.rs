use chrono::Utc;
use std::io;
use std::process;

use crate::core::{movegen, Board};
use crate::perft;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");

pub fn init() {
    println!("\n{}\n", "=".repeat(60));
    println!("Rust Chess Engine  v{} ,  by {}", VERSION, AUTHORS);
    println!("GitHub Repo: {}", REPOSITORY);
    println!("\n{}\n", "=".repeat(60));
}

pub fn run() {
    loop {
        let input_string = read_line();
        let input: Vec<&str> = input_string.split(' ').collect();
        let command = input[0].trim().to_lowercase();

        match command.as_str() {
            "help" => handle_help(),
            "magic" => handle_magic(),
            "perft" => handle_perft(input),
            "perftd" => handle_divided_perft(input),
            "quit" => handle_quit(),
            _ => handle_unknown_command(command),
        }
    }
}

fn handle_help() {
    println!("\nList of available commands:\n");
    println!("    help                          -  Show list of available commands.\n");
    println!("    magic                         -  Generate magic numbers for rooks & bishops.\n");
    println!("    perft [depth]                 -  Run perft test with [depth].\n");
    println!(
        "    perft [depth] fen [fen]       -  Run perft test at position [fen] with [depth].\n"
    );
    println!("    perft [depth] moves [moves]   -  Run perft test at position after moves [moves] with [depth].\n");
    println!("    perftd [depth]                -  Run divided perft test with [depth].\n");
    println!("    perftd [depth] fen [fen]      -  Run divided perft test at position [fen] with [depth].\n");
    println!("    perftd [depth] moves [moves]  -  Run divided perft test at position after moves [moves] with [depth].\n");
    println!("    quit                          -  Quit this program.\n");
}

fn handle_magic() {
    let start = Utc::now();

    println!("\nGenerating magic numbers for rook ...\n");
    movegen::magic::find_rook_magics();
    println!("\nGenerating magic numbers for bishop ...\n");
    movegen::magic::find_bishop_magics();

    let interval = (Utc::now() - start).num_milliseconds();
    println!("\nMagic numbers generated in {} ms", interval);
}

fn handle_perft(input: Vec<&str>) {
    if input.len() < 2 {
        println!("Depth parameter not found!");
        return;
    }

    let max_depth: u8 = match parse_max_depth(input[1]) {
        Ok(result) => result,
        Err(message) => {
            println!("{}", message);
            return;
        }
    };

    let mut board = match parse_board(&input[2..]) {
        Ok(result) => result,
        Err(message) => {
            println!("{}", message);
            return;
        }
    };

    println!("Starting perft test of max depth {} ...", max_depth);
    for depth in 1..=max_depth {
        let start = Utc::now();

        let count = match perft::run(depth, &mut board) {
            Ok(result) => result,
            Err(message) => {
                println!("{}", message);
                return;
            }
        };

        let interval = (Utc::now() - start).num_milliseconds() as f64;
        let mnps = count as f64 / interval / 1000.0;

        println!(
            "depth {} | {} nodes | {} ms | {} Mnps",
            depth, count, interval, mnps
        );
    }

    println!("Perft test was successful.");
}

fn handle_divided_perft(input: Vec<&str>) {
    if input.len() < 2 {
        println!("Depth parameter not found!");
        return;
    }

    let max_depth: u8 = match parse_max_depth(input[1]) {
        Ok(result) => result,
        Err(message) => {
            println!("{}", message);
            return;
        }
    };

    let mut board = match parse_board(&input[2..]) {
        Ok(result) => result,
        Err(message) => {
            println!("{}", message);
            return;
        }
    };

    println!("Starting divided perft test of max depth {} ...", max_depth);
    let result = match perft::run_divided(max_depth, &mut board) {
        Ok(result) => result,
        Err(message) => {
            println!("{}", message);
            return;
        }
    };
    let mut nodes = 0;
    for (m, c) in result {
        println!("{}{}{}", m, " ".repeat(5), c);
        nodes += c;
    }

    println!("\n{} nodes searched in total", nodes);
    println!("\nDivided perft test was successful.");
}

fn handle_quit() {
    process::exit(0);
}

fn handle_unknown_command(command: String) {
    println!("Unknown command: {}", command);
}

fn read_line() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    buf
}

fn parse_max_depth(param: &str) -> Result<u8, &str> {
    let max_depth: u8 = match param.trim().parse() {
        Ok(value) => value,
        Err(_) => {
            return Err("Invalid depth value");
        }
    };

    Ok(max_depth)
}

fn parse_board(param: &[&str]) -> Result<Board, &'static str> {
    if param.is_empty() {
        Ok(Board::new())
    } else {
        let method = param[0];
        match method {
            "fen" => {
                let fen = param[1..].join(" ");
                Board::from(fen.as_str())
            }
            "moves" => Board::from_moves(&param[1..]),
            _ => Err("Invalid method for board generation"),
        }
    }
}
