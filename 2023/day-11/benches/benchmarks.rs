use day11::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    part1::process(divan::black_box(include_str!("../input.txt",))).unwrap();
}

#[divan::bench(consts = [
    2,
    10,
    100,
    1000,
    100000,
    1000000
])]
fn part2<const N: i64>() {
    part2::process(
        divan::black_box(include_str!("../input.txt")),
        divan::black_box(N),
    )
    .unwrap();
}
