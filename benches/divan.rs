use day12::{p1, p2};


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

#[divan::bench]
fn part1_cpp() {
    day12::cpp::p1(divan::black_box(include_str!(
        "../input.txt",
    )));
}

#[divan::bench]
fn part2_cpp() {
    day12::cpp::p2(divan::black_box(include_str!(
        "../input.txt",
    )));
}
#[cfg(feature = "swift")]
#[divan::bench]
fn part1_swift() {
    day12::swift::p1(divan::black_box(include_str!(
        "../input.txt",
    )));
}
#[cfg(feature = "swift")]
#[divan::bench]
fn part2_swift() {
    day12::swift::p2(divan::black_box(include_str!(
        "../input.txt",
    )));
}
