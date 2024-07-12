// a criterion benchmark for p2, p2_reverse, and p2_maps

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use day12::{p1, p2};
use std::fs::File;
use std::io::Read;

fn bench_p1(c: &mut Criterion) {
    let mut g = c.benchmark_group("criterion");
    g.bench_function("part1", |b| {
        b.iter_batched(
            || {
                let mut f = File::open("input.txt").expect("can't open file");
                let mut buf = String::new();
                f.read_to_string(&mut buf).expect("can't read file");
                buf
            },
            |f| p1(&f),
            BatchSize::SmallInput,
        )
    });
    g.finish();
}

fn bench_p1_cpp(c: &mut Criterion) {
    let mut g = c.benchmark_group("criterion");
    g.bench_function("part1_cpp", |b| {
        b.iter_batched(
            || {
                let mut f = File::open("input.txt").expect("can't open file");
                let mut buf = String::new();
                f.read_to_string(&mut buf).expect("can't read file");
                buf
            },
            |f| day12::cpp::p1(&f),
            BatchSize::SmallInput,
        )
    });
    g.finish();
}

#[cfg(feature = "swift")]
fn bench_p1_swift(c: &mut Criterion) {
    use std::time::Duration;
    let mut g = c.benchmark_group("criterion");
    g.measurement_time(Duration::from_secs(10));
    g.bench_function("part1_swift", |b| {
        b.iter_batched(
            || {
                let mut f = File::open("input.txt").expect("can't open file");
                let mut buf = String::new();
                f.read_to_string(&mut buf).expect("can't read file");
                buf
            },
            |f| day12::swift::p1(&f),
            BatchSize::SmallInput,
        )
    });
    g.finish();
}

fn bench_p2(c: &mut Criterion) {
    let mut g = c.benchmark_group("criterion");
    g.bench_function("part2", |b| {
        b.iter_batched(
            || {
                let mut f = File::open("input.txt").expect("can't open file");
                let mut buf = String::new();
                f.read_to_string(&mut buf).expect("can't read file");
                buf
            },
            |f| p2(&f),
            BatchSize::SmallInput,
        )
    });
    g.finish()
}

fn bench_p2_cpp(c: &mut Criterion) {
    let mut g = c.benchmark_group("criterion");
    g.bench_function("part2_cpp", |b| {
        b.iter_batched(
            || {
                let mut f = File::open("input.txt").expect("can't open file");
                let mut buf = String::new();
                f.read_to_string(&mut buf).expect("can't read file");
                buf
            },
            |f| day12::cpp::p2(&f),
            BatchSize::SmallInput,
        )
    });
    g.finish()
}

#[cfg(feature = "swift")]
fn bench_p2_swift(c: &mut Criterion) {
    use std::time::Duration;
    let mut g = c.benchmark_group("criterion");
    g.measurement_time(Duration::from_secs(10));
    g.bench_function("part2_swift", |b| {
        b.iter_batched(
            || {
                let mut f = File::open("input.txt").expect("can't open file");
                let mut buf = String::new();
                f.read_to_string(&mut buf).expect("can't read file");
                buf
            },
            |f| day12::swift::p2(&f),
            BatchSize::SmallInput,
        )
    });
    g.finish()
}
criterion_group!(benches, bench_p1, bench_p1_cpp, bench_p2, bench_p2_cpp);
#[cfg(feature = "swift")]
criterion_group!(benches_swift, bench_p1_swift, bench_p2_swift);
#[cfg(feature = "swift")]
criterion_main!(benches, benches_swift);
#[cfg(not(feature = "swift"))]
criterion_main!(benches);