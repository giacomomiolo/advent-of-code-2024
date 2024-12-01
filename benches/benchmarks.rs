use advent_of_code_2024::solutions::day01;
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

fn update_readme(benchmarks: &[(String, String)]) {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("README.md");
    let content = fs::read_to_string(&d).unwrap();

    let parts: Vec<&str> = content.split("<!-- BENCHMARKS -->").collect();
    let end_parts: Vec<&str> = parts[1].split("<!-- BENCHMARKS_END -->").collect();

    let mut benchmark_section = String::from("\n");
    benchmark_section.push_str("| Solution | Time |\n");
    benchmark_section.push_str("|----------|------|\n");

    let mut sorted_benchmarks = benchmarks.to_vec();
    sorted_benchmarks.sort_by(|a, b| a.0.cmp(&b.0));

    for (name, result) in sorted_benchmarks {
        benchmark_section.push_str(&format!("| {} | {} |\n", name, result));
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

fn benchmark_day01(c: &mut Criterion, results: &mut Vec<(String, String)>) {
    if let Ok(input) = day01::read_input() {
        let mut group = c.benchmark_group("day01");
        group.sample_size(100);

        let mut duration = Duration::ZERO;
        group.bench_function("part1", |b| {
            b.iter_custom(|iters| {
                let start = std::time::Instant::now();
                for _ in 0..iters {
                    black_box(day01::part1::solve_for_input(black_box(&input))).unwrap();
                }
                duration = start.elapsed() / iters as u32;
                start.elapsed()
            });
        });
        results.push(("Day 01, Part 1".to_string(), format_duration(duration)));

        let mut duration = Duration::ZERO;
        group.bench_function("part2", |b| {
            b.iter_custom(|iters| {
                let start = std::time::Instant::now();
                for _ in 0..iters {
                    black_box(day01::part2::solve_for_input(black_box(&input))).unwrap();
                }
                duration = start.elapsed() / iters as u32;
                start.elapsed()
            });
        });
        results.push(("Day 01, Part 2".to_string(), format_duration(duration)));

        group.finish();
    }
}

fn benchmark_all(c: &mut Criterion) {
    let mut results = Vec::new();
    benchmark_day01(c, &mut results);
    update_readme(&results);
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(100);
    targets = benchmark_all
}
criterion_main!(benches);
