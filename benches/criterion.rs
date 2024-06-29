// a criterion benchmark for p2, p2_reverse, and p2_maps

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use day12::{p1, p1_single_threaded, p2};
use std::fs::File;
use std::io::Read;
use std::time::Duration;

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

fn bench_p1_single_threaded(c: &mut Criterion) {
    let mut g = c.benchmark_group("criterion");
    g.bench_function("part1 Single-threaded", |b| {
        b.iter_batched(
            || {
                let mut f = File::open("input.txt").expect("can't open file");
                let mut buf = String::new();
                f.read_to_string(&mut buf).expect("can't read file");
                buf
            },
            |f| p1_single_threaded(&f),
            BatchSize::SmallInput,
        )
    });
    g.finish();
}

fn bench_p1_cpp(c: &mut Criterion) {
    let mut g = c.benchmark_group("criterion");
    g.measurement_time(Duration::from_secs(10));
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

criterion_group!(benches, bench_p1, bench_p1_single_threaded, bench_p1_cpp, bench_p2);
criterion_main!(benches);
