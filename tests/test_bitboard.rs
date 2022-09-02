#[cfg(test)]
mod bitboard_tests {
    use rust_chess_engine::core::{masks, square, BitBoard};

    #[test]
    fn test_lsb() {
        let bitboard = BitBoard::new(square::D4);
        assert_eq!(BitBoard(1u64) << square::D4.to_u8(), bitboard.lsb());
    }

    #[test]
    fn test_pop_lsb() {
        let bitboard = BitBoard::new(square::D4);
        assert_eq!(BitBoard(masks::EMPTY), bitboard.pop_lsb());
    }

    #[test]
    fn test_pop_count() {
        let bitboard = BitBoard::new(square::D4);
        assert_eq!(1, bitboard.pop_count());
    }

    #[test]
    fn test_bit_scan() {
        let bitboard = BitBoard::new(square::D4);
        assert_eq!(square::D4.to_u8(), bitboard.bit_scan());
    }
}
