use std::{
    fs::File,
    io::{BufWriter, Write},
};

use logic_building::easy::prime_number::SixKSolution;

fn main() {
    let min = 0;
    let max = 1_000_000;

    let file = File::create("data/prime.txt").unwrap();
    let mut buf_writer = BufWriter::new(file);

    let now = std::time::Instant::now();
    for n in min..=max {
        if SixKSolution::is_prime(n) {
            writeln!(buf_writer, "{n}").unwrap();
        }
    }
    let time = now.elapsed();
    println!("Time: {time:?}")
}
