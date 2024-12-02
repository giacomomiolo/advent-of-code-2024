use advent_of_code_2024::solutions::{day01, day02};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::time::Duration;

fn format_duration(d: Duration) -> String {
    if d.as_nanos() < 1_000 {
        format!("{} ns", d.as_nanos())
    } else if d.as_micros() < 1_000 {
        format!("{:.2} µs", d.as_nanos() as f64 / 1000.0)
    } else if d.as_millis() < 1_000 {
        format!("{:.2} ms", d.as_micros() as f64 / 1000.0)
    } else {
        format!("{:.2} s", d.as_millis() as f64 / 1000.0)
    }
}

#[derive(Clone)]
struct BenchStats {
    name: String,
    min: Duration,
    median: Duration,
    p95: Duration,
    p99: Duration,
}

impl BenchStats {
    fn overhead_percent(&self, duration: Duration) -> f64 {
        ((duration.as_nanos() as f64 / self.median.as_nanos() as f64) - 1.0) * 100.0
    }

    fn min_percent(&self) -> f64 {
        ((self.median.as_nanos() as f64 / self.min.as_nanos() as f64) - 1.0) * -100.0
    }
}

fn update_readme(benchmarks: &[BenchStats]) {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("README.md");
    let content = fs::read_to_string(&d).unwrap();

    let parts: Vec<&str> = content.split("<!-- BENCHMARKS -->").collect();
    let end_parts: Vec<&str> = parts[1].split("<!-- BENCHMARKS_END -->").collect();

    let mut benchmark_section = String::from("\n");
    benchmark_section.push_str("## Benchmarks\n");
    benchmark_section.push_str("Ran on MacBook Pro M1 Max 2021 with 32GB RAM and 10 cores.\n");
    benchmark_section.push_str("Benchmarks done with [criterion](https://github.com/bheisler/criterion.rs), using 200 samples per benchmark.\n\n");
    benchmark_section.push_str("| Solution | Min (-%) | Median | p95 (+%) | p99 (+%) |\n");
    benchmark_section.push_str("|----------|----------|---------|-----------|----------|\n");

    let mut sorted_benchmarks = benchmarks.to_vec();
    sorted_benchmarks.sort_by(|a, b| a.name.cmp(&b.name));

    for stats in &sorted_benchmarks {
        benchmark_section.push_str(&format!(
            "| {} | {} ({:.0}%) | {} | {} (+{:.0}%) | {} (+{:.0}%) |\n",
            stats.name,
            format_duration(stats.min),
            stats.min_percent(),
            format_duration(stats.median),
            format_duration(stats.p95),
            stats.overhead_percent(stats.p95),
            format_duration(stats.p99),
            stats.overhead_percent(stats.p99)
        ));
    }
    benchmark_section.push('\n');

    let new_content = format!(
        "{}<!-- BENCHMARKS -->{}<!-- BENCHMARKS_END -->{}",
        parts[0], benchmark_section, end_parts[1]
    );

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&d)
        .unwrap();
    file.write_all(new_content.as_bytes()).unwrap();
}

fn benchmark_day(
    c: &mut Criterion,
    results: &mut Vec<BenchStats>,
    day: u32,
    input: &str,
    parts: &[(String, fn(&str) -> anyhow::Result<i64>)],
) {
    let mut group = c.benchmark_group(format!("day{:02}", day));
    group.sample_size(200);

    for (part_name, solve_fn) in parts {
        let mut measurements = Vec::with_capacity(200);

        group.bench_function(part_name, |b| {
            b.iter_custom(|iters| {
                let start = std::time::Instant::now();
                for _ in 0..iters {
                    black_box(solve_fn(black_box(input))).unwrap();
                }
                let elapsed = start.elapsed();
                let per_iter = elapsed / iters as u32;
                measurements.push(per_iter);
                elapsed
            });
        });

        measurements.sort_unstable();
        let min = measurements[0];
        let median = measurements[measurements.len() / 2];
        let p95_idx = ((measurements.len() as f64 * 0.95) as usize).min(measurements.len() - 1);
        let p99_idx = ((measurements.len() as f64 * 0.99) as usize).min(measurements.len() - 1);
        let p95 = measurements[p95_idx];
        let p99 = measurements[p99_idx];

        results.push(BenchStats {
            name: format!("Day {:02}, {}", day, part_name),
            min,
            median,
            p95,
            p99,
        });
    }

    group.finish();
}

fn benchmark_all(c: &mut Criterion) {
    let mut results = Vec::new();
    
    // Benchmark Day 01
    if let Ok(input) = day01::read_input() {
        benchmark_day(
            c,
            &mut results,
            1,
            &input,
            &[
                ("Part 1".to_string(), day01::part1::solve_for_input),
                ("Part 2".to_string(), day01::part2::solve_for_input),
            ],
        );
    }

    // Benchmark Day 02
    if let Ok(input) = day02::read_input() {
        benchmark_day(
            c,
            &mut results,
            2,
            &input,
            &[
                ("Part 1".to_string(), day02::part1::solve_for_input),
                ("Part 2".to_string(), day02::part2::solve_for_input),
            ],
        );
    }

    update_readme(&results);
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(200);
    targets = benchmark_all
}
criterion_main!(benches);