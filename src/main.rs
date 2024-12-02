use advent_of_code_2024::solutions::{day01, day02};
use anyhow::Result;

fn main() -> Result<()> {
    println!("Advent of Code 2024");
    println!("==================");
    
    println!("\nDay 1:");
    println!("  Part 1: {}", day01::part1::solve()?);
    println!("  Part 2: {}", day01::part2::solve()?);
    
    println!("\nDay 2:");
    println!("  Part 1: {}", day02::part1::solve()?);
    println!("  Part 1: {}", day02::part2::solve()?);
    
    Ok(())
}