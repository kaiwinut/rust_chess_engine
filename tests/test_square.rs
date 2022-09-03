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

    #[test]
    fn test_square_get_neighbors() {
        assert_eq!(Some(square::D5), square::D4.north());
        assert_eq!(Some(square::E5), square::D4.north_east());
        assert_eq!(Some(square::E4), square::D4.east());
        assert_eq!(Some(square::E3), square::D4.south_east());
        assert_eq!(Some(square::D3), square::D4.south());
        assert_eq!(Some(square::C3), square::D4.south_west());
        assert_eq!(Some(square::C4), square::D4.west());
        assert_eq!(Some(square::C5), square::D4.north_west());

        assert_eq!(None, square::A1.south());
        assert_eq!(None, square::H5.east());
        assert_eq!(Some(square::C8), square::C7.north());
        assert_eq!(None, square::C8.north());
    }
}
