use super::square::Square;

pub fn grid_to_string<'a, F: Fn(Square) -> &'a str>(char_at: F, raw: u64) -> String {
    let mut string = format!("\n{}\n\nBitBoard: {:#018x}\n\n", "=".repeat(28), raw);
    let row_head = ['1', '2', '3', '4', '5', '6', '7', '8'];

    for row in (0..8).rev() {
        string += &format!("   {} | ", row_head[row as usize]);

        for col in 0..8 {
            if let Some(sq) = Square::from(col, row) {
                string += char_at(sq);
            }
        }

        string += "\n";
    }

    string += "       - - - - - - - -\n";
    string + &"       A B C D E F G H\n\n============================\n".to_string()
}
