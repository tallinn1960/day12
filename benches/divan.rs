use day12::{p1, p2};
use divan::AllocProfiler;

#[cfg(not(feature = "par"))]
#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    p1(divan::black_box(include_str!(
        "../input.txt",
    )));
}

#[divan::bench]
fn part2() {
    p2(divan::black_box(include_str!(
        "../input.txt",
    )));
}
