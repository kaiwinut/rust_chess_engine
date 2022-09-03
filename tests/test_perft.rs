#[cfg(test)]
mod perft_tests {
    use rust_chess_engine::core::movegen;
    use rust_chess_engine::core::Board;
    use rust_chess_engine::perft;
    use std::sync::Once;

    static START: Once = Once::new();

    macro_rules! test_perft {
        ($($name:ident: $depth:expr, $expected_nodes:expr,)*) => {
         $(
            #[test]
            fn $name() {
                START.call_once(|| {
                    movegen::init();
                });

                assert_eq!($expected_nodes, perft::run($depth, &mut Board::new()).unwrap());
            }
         )*
        }
    }

    test_perft! {
        test_perft_depth_1: 1, 20,
        test_perft_depth_2: 2, 400,
        test_perft_depth_3: 3, 8902,
        test_perft_depth_4: 4, 197281,
        test_perft_depth_5: 5, 4865609,
        // test_perft_depth_6: 6, 119060324,
        // test_perft_depth_7: 7, 3195901860,
    }
}

// mod perft_divided_tests {
//     use rust_chess_engine::core::movegen;
//     use rust_chess_engine::core::Board;
//     use rust_chess_engine::perft;

//     #[test]
//     fn test_perft_divided_depth_3() {
//         movegen::init();
//         println!("{:?}", perft::run_divided(3, &mut Board::new()));
//     }
// }
