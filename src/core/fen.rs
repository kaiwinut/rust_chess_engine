use super::board::*;
use super::square;
use super::{BitBoard, Square};

#[allow(dead_code)]
pub fn fen_to_board(fen: &str) -> Result<Board, &'static str> {
    let slices: Vec<&str> = fen.split(' ').collect();
    if slices.len() < 6 {
        return Err("Invalid FEN while converting fen to board");
    }

    let mut board = Board::new_empty();
    add_pieces_from_fen(&mut board, slices[0].trim())?;
    add_color_to_move_from_fen(&mut board, slices[1].trim())?;
    add_castling_rights_from_fen(&mut board, slices[2].trim())?;
    add_en_passant_from_fen(&mut board, slices[3].trim())?;
    add_half_move_clock_from_fen(&mut board, slices[4].trim())?;
    add_full_move_number_from_fen(&mut board, slices[5].trim())?;

    Ok(board)
}

#[allow(dead_code)]
fn add_pieces_from_fen(board: &mut Board, fen_str: &str) -> Result<(), &'static str> {
    let mut current_index: u8 = 56;
    for c in fen_str.chars() {
        if c == '/' {
            current_index -= 16;
        } else if c.is_ascii_digit() {
            current_index += c.to_digit(10).unwrap() as u8;
        } else {
            let piece = match c {
                'P' => Piece::WP,
                'N' => Piece::WN,
                'B' => Piece::WB,
                'R' => Piece::WR,
                'Q' => Piece::WQ,
                'K' => Piece::WK,
                'p' => Piece::BP,
                'n' => Piece::BN,
                'b' => Piece::BB,
                'r' => Piece::BR,
                'q' => Piece::BQ,
                'k' => Piece::BK,
                _ => return Err("Invalid FEN while parsing pieces"),
            };

            board.add_piece_to_square(piece, Square(current_index));
            current_index += 1;
        }
    }

    assert_eq!(current_index, 8);
    Ok(())
}

#[allow(dead_code)]
fn add_color_to_move_from_fen(board: &mut Board, fen_str: &str) -> Result<(), &'static str> {
    board.color_to_move = if fen_str.starts_with('w') {
        Color::WHITE
    } else {
        Color::BLACK
    };
    Ok(())
}

#[allow(dead_code)]
fn add_castling_rights_from_fen(board: &mut Board, fen_str: &str) -> Result<(), &'static str> {
    if fen_str == "-" {
        return Ok(());
    }

    for c in fen_str.chars() {
        board.castling_rights |= match c {
            'K' => CastlingRights::WHITE_SHORT_CASTLE,
            'Q' => CastlingRights::WHITE_LONG_CASTLE,
            'k' => CastlingRights::BLACK_SHORT_CASTLE,
            'q' => CastlingRights::BLACK_LONG_CASTLE,
            _ => CastlingRights::NONE,
        };
    }

    Ok(())
}

#[allow(dead_code)]
fn add_en_passant_from_fen(board: &mut Board, fen_str: &str) -> Result<(), &'static str> {
    if fen_str == "-" {
        return Ok(());
    }

    if let Some(square) = Square::from_string(fen_str) {
        board.en_passant = BitBoard::new(square);
    } else {
        return Err("Invalid FEN while parsing en passant");
    };

    Ok(())
}

#[allow(dead_code)]
fn add_half_move_clock_from_fen(board: &mut Board, fen_str: &str) -> Result<(), &'static str> {
    board.half_move_clock = match fen_str.parse::<usize>() {
        Ok(value) => value,
        Err(_) => return Err("Invalid FEN while parsing half move clock"),
    };

    Ok(())
}

#[allow(dead_code)]
fn add_full_move_number_from_fen(board: &mut Board, fen_str: &str) -> Result<(), &'static str> {
    board.full_move_number = match fen_str.parse::<usize>() {
        Ok(value) => value,
        Err(_) => return Err("Invalid FEN while parsing full move number"),
    };

    Ok(())
}

#[allow(dead_code)]
pub fn board_to_fen(board: &Board) -> String {
    let fen_pieces = pieces_to_fen(board);
    let fen_color_to_move = color_to_move_to_fen(board);
    let fen_castling_rights = castling_rights_to_fen(board);
    let fen_en_passant = en_passant_to_fen(board);
    let fen_half_move_clock = half_move_clock_to_fen(board);
    let fen_full_move_number = full_move_number_to_fen(board);

    format!(
        "{} {} {} {} {} {}",
        fen_pieces,
        fen_color_to_move,
        fen_castling_rights,
        fen_en_passant,
        fen_half_move_clock,
        fen_full_move_number
    )
}

#[allow(dead_code)]
fn pieces_to_fen(board: &Board) -> String {
    let mut string = String::new();
    let mut empty_squares = 0;

    for rank in (0..8).rev() {
        for file in 0..8 {
            let square = Square::from(file, rank).unwrap();
            let piece = board.piece_at_square(square);

            if piece == Piece::EMPTY {
                empty_squares += 1;
            } else {
                if empty_squares != 0 {
                    string.push(char::from_digit(empty_squares, 10).unwrap());
                    empty_squares = 0;
                }

                string.push(match piece {
                    Piece::WP => 'P',
                    Piece::WN => 'N',
                    Piece::WB => 'B',
                    Piece::WR => 'R',
                    Piece::WQ => 'Q',
                    Piece::WK => 'K',
                    Piece::BP => 'p',
                    Piece::BN => 'n',
                    Piece::BB => 'b',
                    Piece::BR => 'r',
                    Piece::BQ => 'q',
                    Piece::BK => 'k',
                    _ => panic!(
                        "Invalid piece when converting board to fen, piece : {}",
                        piece
                    ),
                });
            }

            if square.to_u8() % 8 == 7 {
                if empty_squares != 0 {
                    string.push(char::from_digit(empty_squares, 10).unwrap());
                    empty_squares = 0;
                }
                if square != square::H1 {
                    string.push('/');
                }
            }
        }
    }

    string
}

#[allow(dead_code)]
fn color_to_move_to_fen(board: &Board) -> String {
    match board.color_to_move {
        Color::WHITE => "w".to_string(),
        Color::BLACK => "b".to_string(),
        _ => panic!(
            "Invalid color to move when converting board to fen, color : {}",
            board.color_to_move
        ),
    }
}

#[allow(dead_code)]
fn castling_rights_to_fen(board: &Board) -> String {
    if board.castling_rights == CastlingRights::NONE {
        return "-".to_string();
    }

    let mut string = String::new();

    if board
        .castling_rights
        .contains(CastlingRights::WHITE_SHORT_CASTLE)
    {
        string.push('K');
    }
    if board
        .castling_rights
        .contains(CastlingRights::WHITE_LONG_CASTLE)
    {
        string.push('Q');
    }
    if board
        .castling_rights
        .contains(CastlingRights::BLACK_SHORT_CASTLE)
    {
        string.push('k');
    }
    if board
        .castling_rights
        .contains(CastlingRights::BLACK_LONG_CASTLE)
    {
        string.push('q');
    }

    string
}

#[allow(dead_code)]
fn en_passant_to_fen(board: &Board) -> String {
    if board.en_passant == BitBoard::EMPTY {
        return "-".to_string();
    }

    Square(board.en_passant.bit_scan()).as_string().to_string()
}

#[allow(dead_code)]
fn half_move_clock_to_fen(board: &Board) -> String {
    board.half_move_clock.to_string()
}

#[allow(dead_code)]
fn full_move_number_to_fen(board: &Board) -> String {
    board.full_move_number.to_string()
}
