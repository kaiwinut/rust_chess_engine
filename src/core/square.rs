pub struct Square(pub usize);

impl Square {
    pub fn new(s: usize) -> Self {
        Square(s)
    }

    pub fn from(row: usize, col: usize) -> Self {
        Square(row * 8 + col)
    }

    pub fn to_usize(self) -> usize {
        self.0
    }
}

pub fn grid_to_string<'a, F: Fn(Square) -> &'a str>(char_at: F, raw: u64) -> String {
    let mut string = format!("\n{}\n\nBitBoard: {:#018x}\n\n", "=".repeat(28), raw);
    let row_head = ['1', '2', '3', '4', '5', '6', '7', '8'];

    for row in (0..8).rev() {
        string += &format!("   {} | ", row_head[row]);

        for col in 0..8 {
            string += char_at(Square::from(row as usize, col));
        }

        string += "\n";
    }

    string += "       - - - - - - - -\n";
    string + &"       A B C D E F G H\n\n============================\n".to_string()
}
