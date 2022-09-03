use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_chess_engine::core::movegen;
use rust_chess_engine::core::Board;
use rust_chess_engine::perft;

fn perft_benchmark(c: &mut Criterion) {
    movegen::init();

    c.bench_function("perft", |b| {
        b.iter(|| perft::run(black_box(4), &mut Board::new()).unwrap())
    });
}

criterion_group!(benches, perft_benchmark);
criterion_main!(benches);
