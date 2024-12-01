use anyhow::Result;
use std::collections::HashMap;

pub fn solve() -> Result<i64> {
    let input = super::read_input()?;
    solve_for_input(&input)
}

pub fn solve_for_input(input: &str) -> Result<i64> {
    let mut left_numbers = Vec::new();
    let mut right_numbers = Vec::new();

    for line in input.lines() {
        let numbers: Vec<i64> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if numbers.len() == 2 {
            left_numbers.push(numbers[0]);
            right_numbers.push(numbers[1]);
        }
    }

    let right_freq: HashMap<i64, usize> =
        right_numbers.iter().fold(HashMap::new(), |mut map, &num| {
            *map.entry(num).or_insert(0) += 1;
            map
        });

    let similarity_score: i64 = left_numbers
        .iter()
        .map(|&num| {
            let freq = right_freq.get(&num).copied().unwrap_or(0);
            num * (freq as i64)
        })
        .sum();

    Ok(similarity_score)
}
