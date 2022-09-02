#[cfg(test)]
mod square_tests {
    use rust_chess_engine::core::square;
    use rust_chess_engine::core::Square;

    #[test]
    fn test_square_from() {
        let file1 = 0;
        let file2 = -1;
        let rank1 = 5;

        assert_eq!(Some(square::A6), Square::from(file1, rank1));
        assert_eq!(None, Square::from(file2, rank1));
    }

    #[test]
    fn test_square_from_string() {
        assert_eq!(Some(square::D4), Square::from_string("d4"));
        assert_eq!(Some(square::E5), Square::from_string("e5"));
    }
}
