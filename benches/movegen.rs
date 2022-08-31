use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_chess_engine::core::{movegen, masks, BitBoard, Square};

fn movegen_benchmark(c: &mut Criterion) {
    movegen::init();

    c.bench_function("get_rook_attacks", |b| {
        let mut bb = BitBoard(masks::EMPTY);
        let mut index : usize = 0;

        b.iter(|| {
            movegen::get_rook_attacks(black_box(Square((index % 64) as u8)), black_box(bb));

            bb = bb + BitBoard(1);
            index += 1;
        })
    });

    c.bench_function("get_bishop_attacks", |b| {
        let mut bb = BitBoard(masks::EMPTY);
        let mut index : usize = 0;

        b.iter(|| {
            movegen::get_bishop_attacks(black_box(Square((index % 64) as u8)), black_box(bb));

            bb = bb + BitBoard(1);
            index += 1;
        })
    });
}

criterion_group!(benches, movegen_benchmark);
criterion_main!(benches);