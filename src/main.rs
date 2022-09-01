mod core;
mod perft;

use crate::core::movegen;

use chrono::*;

fn main() {
    let start = Utc::now();
    fastrand::seed(9000);
    movegen::init();

    let interval = Utc::now() - start;
    println!("\nTime: {} ms\n", interval.num_milliseconds());

    let depth = 4;
    let start = Utc::now();
    let count = perft::run(depth);

    let interval = Utc::now() - start;
    println!(
        "\nPerft test :  Depth {} | {} nodes | Time {} ms\n",
        depth,
        count,
        interval.num_milliseconds()
    );
}
