#[cfg(test)]
mod fen_tests {
    use rust_chess_engine::core::board::Board;

    #[test]
    fn test_fen_to_board() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

        assert_eq!(Board::new(), Board::from(fen));
    }

    #[test]
    fn test_board_to_fen() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

        assert_eq!(fen, Board::new().to_fen());
    }

    macro_rules! test_fen {
        ($($fn_name:ident: $fen:expr,)*) => {
            $(
                #[test]
                fn $fn_name() {
                    let board = Board::from($fen);
                    assert_eq!($fen, board.to_fen());
                }
            )*
        };
    }

    test_fen! {
        test_fen_default: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        test_fen_mid_game1: "5rk1/2b1qp1p/1r2p1pB/1ppnn3/3pN3/1P1P2P1/2P1QPBP/R4RK1 b - - 7 22",
        test_fen_mid_game2: "2k4r/1p3pp1/p2p2n1/2P1p2q/P1P1P3/3PBPP1/2R3Qr/5RK1 b - - 2 22",
        test_fen_mid_game3: "r6k/p1B4p/Pp3rp1/3p4/2nP4/2PQ1PPq/7P/1R3RK1 b - - 0 32",
        test_fen_en_passant1: "r3kb2/p4pp1/2q1p3/1pP1n1N1/3B2nr/1QP1P3/PP1N3P/R2R2K1 w q b6 0 2",
        test_fen_en_passant2: "rn1qkbnr/pp3ppp/4p3/3pPb2/1PpP4/4BN2/P1P1BPPP/RN1QK2R b KQkq b3 0 2",
        test_fen_en_passant3: "rnbqkbnr/pp1p1ppp/8/2pPp3/8/8/PPP1PPPP/RNBQKBNR w KQkq e6 0 2",
        test_fen_end_game1: "8/8/6Q1/8/6k1/1P2q3/7p/7K b - - 14 75",
        test_fen_end_game2: "8/8/4nPk1/8/6pK/8/1R3P1P/2B3r1 b - - 1 54",
        test_fen_end_game3: "8/7q/5K2/2q5/6k1/8/8/8 b - - 5 60",
    }
}
