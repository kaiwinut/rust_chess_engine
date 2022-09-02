#[cfg(test)]
mod movegen_tests {
    use rust_chess_engine::core::movegen::*;
    use rust_chess_engine::core::{square, BitBoard};

    #[test]
    fn test_get_non_sliding_attacks() {
        init_non_sliding_attacks();

        let sq1 = square::A4;
        let sq2 = square::B4;

        assert_eq!(BitBoard(0x0000000302030000), get_king_attacks(sq1));
        assert_eq!(BitBoard(0x0000020400040200), get_knight_attacks(sq1));

        assert_eq!(BitBoard(0x0000000705070000), get_king_attacks(sq2));
        assert_eq!(BitBoard(0x0000050800080500), get_knight_attacks(sq2));
    }

    #[test]
    fn test_get_sliding_attacks() {
        init_magic();

        assert_eq!(
            BitBoard(0x00080808f4080808),
            get_rook_attacks(square::D4, BitBoard(0x0008001404002200))
        );
        assert_eq!(
            BitBoard(0x0000001400142200),
            get_bishop_attacks(square::D4, BitBoard(0x0008001404002200))
        );
        assert_eq!(
            BitBoard(0x0008081cf41c2a08),
            get_queen_attacks(square::D4, BitBoard(0x0008001404002200))
        );

        assert_eq!(
            BitBoard(0x0000000101010201),
            get_rook_attacks(square::A2, BitBoard(0x0008001504002200))
        );
        assert_eq!(
            BitBoard(0x0000000004020002),
            get_bishop_attacks(square::A2, BitBoard(0x0008001504002200))
        );
        assert_eq!(
            BitBoard(0x0000000105030203),
            get_queen_attacks(square::A2, BitBoard(0x0008001504002200))
        );
    }
}
