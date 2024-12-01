use anyhow::Result;

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

    left_numbers.sort_unstable();
    right_numbers.sort_unstable();

    let total_distance: i64 = left_numbers
        .iter()
        .zip(right_numbers.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    Ok(total_distance)
}
