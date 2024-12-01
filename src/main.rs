use advent_of_code_2024::solutions::day01;
use anyhow::Result;

fn main() -> Result<()> {
    println!("Day 1:");
    println!("- Part 1: {}", day01::part1::solve()?);
    println!("- Part 2: {}", day01::part2::solve()?);
    Ok(())
}
