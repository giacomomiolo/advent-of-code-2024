use advent_of_code_2024::solutions::day01;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_day01(c: &mut Criterion) {
    let input = day01::read_input().unwrap();

    let mut group = c.benchmark_group("day01");

    group.bench_function("part1", |b| {
        b.iter(|| day01::part1::solve_for_input(black_box(&input)))
    });

    group.bench_function("part2", |b| {
        b.iter(|| day01::part2::solve_for_input(black_box(&input)))
    });

    group.finish();
}

criterion_group!(benches, benchmark_day01);
criterion_main!(benches);
