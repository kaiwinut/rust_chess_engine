#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Square(pub u8);

impl Square {
    pub fn new(s: u8) -> Self {
        Square(s)
    }

    pub fn from(file: i8, rank: i8) -> Option<Self> {
        if rank >= 0 && rank <= 7 && file >= 0 && file <= 7 {
            Some(Square((rank * 8 + file) as u8))
        } else {
            None
        }        
    }

    pub fn file(&self) -> u8 {
        self.0 % 8
    }

    pub fn rank(&self) -> u8 {
        self.0 / 8
    }

    pub fn to_u8(self) -> u8 {
        self.0
    }

    pub fn to_usize(self) -> usize {
        self.0 as usize
    }
}

#[allow(dead_code)]
pub const A1: Square = Square(0);
#[allow(dead_code)]
pub const A2: Square = Square(8);
#[allow(dead_code)]
pub const A3: Square = Square(16);
#[allow(dead_code)]
pub const A4: Square = Square(24);
#[allow(dead_code)]
pub const A5: Square = Square(32);
#[allow(dead_code)]
pub const A6: Square = Square(40);
#[allow(dead_code)]
pub const A7: Square = Square(48);
#[allow(dead_code)]
pub const A8: Square = Square(56);
#[allow(dead_code)]
pub const B1: Square = Square(1);
#[allow(dead_code)]
pub const B2: Square = Square(9);
#[allow(dead_code)]
pub const B3: Square = Square(17);
#[allow(dead_code)]
pub const B4: Square = Square(25);
#[allow(dead_code)]
pub const B5: Square = Square(33);
#[allow(dead_code)]
pub const B6: Square = Square(41);
#[allow(dead_code)]
pub const B7: Square = Square(49);
#[allow(dead_code)]
pub const B8: Square = Square(57);
#[allow(dead_code)]
pub const C1: Square = Square(2);
#[allow(dead_code)]
pub const C2: Square = Square(10);
#[allow(dead_code)]
pub const C3: Square = Square(18);
#[allow(dead_code)]
pub const C4: Square = Square(26);
#[allow(dead_code)]
pub const C5: Square = Square(34);
#[allow(dead_code)]
pub const C6: Square = Square(42);
#[allow(dead_code)]
pub const C7: Square = Square(50);
#[allow(dead_code)]
pub const C8: Square = Square(58);
#[allow(dead_code)]
pub const D1: Square = Square(3);
#[allow(dead_code)]
pub const D2: Square = Square(11);
#[allow(dead_code)]
pub const D3: Square = Square(19);
#[allow(dead_code)]
pub const D4: Square = Square(27);
#[allow(dead_code)]
pub const D5: Square = Square(35);
#[allow(dead_code)]
pub const D6: Square = Square(43);
#[allow(dead_code)]
pub const D7: Square = Square(51);
#[allow(dead_code)]
pub const D8: Square = Square(59);
#[allow(dead_code)]
pub const E1: Square = Square(4);
#[allow(dead_code)]
pub const E2: Square = Square(12);
#[allow(dead_code)]
pub const E3: Square = Square(20);
#[allow(dead_code)]
pub const E4: Square = Square(28);
#[allow(dead_code)]
pub const E5: Square = Square(36);
#[allow(dead_code)]
pub const E6: Square = Square(44);
#[allow(dead_code)]
pub const E7: Square = Square(52);
#[allow(dead_code)]
pub const E8: Square = Square(60);
#[allow(dead_code)]
pub const F1: Square = Square(5);
#[allow(dead_code)]
pub const F2: Square = Square(13);
#[allow(dead_code)]
pub const F3: Square = Square(21);
#[allow(dead_code)]
pub const F4: Square = Square(29);
#[allow(dead_code)]
pub const F5: Square = Square(37);
#[allow(dead_code)]
pub const F6: Square = Square(45);
#[allow(dead_code)]
pub const F7: Square = Square(53);
#[allow(dead_code)]
pub const F8: Square = Square(61);
#[allow(dead_code)]
pub const G1: Square = Square(6);
#[allow(dead_code)]
pub const G2: Square = Square(14);
#[allow(dead_code)]
pub const G3: Square = Square(22);
#[allow(dead_code)]
pub const G4: Square = Square(30);
#[allow(dead_code)]
pub const G5: Square = Square(38);
#[allow(dead_code)]
pub const G6: Square = Square(46);
#[allow(dead_code)]
pub const G7: Square = Square(54);
#[allow(dead_code)]
pub const G8: Square = Square(62);
#[allow(dead_code)]
pub const H1: Square = Square(7);
#[allow(dead_code)]
pub const H2: Square = Square(15);
#[allow(dead_code)]
pub const H3: Square = Square(23);
#[allow(dead_code)]
pub const H4: Square = Square(31);
#[allow(dead_code)]
pub const H5: Square = Square(39);
#[allow(dead_code)]
pub const H6: Square = Square(47);
#[allow(dead_code)]
pub const H7: Square = Square(55);
#[allow(dead_code)]
pub const H8: Square = Square(63);


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_from() {
        let file1 = 0;
        let file2 = -1;
        let rank1 = 5;
        
        assert_eq!(Square::from(file1, rank1), Some(A6));
        assert_eq!(Square::from(file2, rank1), None);
    }
}