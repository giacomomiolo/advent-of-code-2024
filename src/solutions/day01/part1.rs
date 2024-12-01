use anyhow::Result;

pub fn solve() -> Result<i64> {
    let input = super::read_input()?;
    solve_for_input(&input)
}

pub fn solve_for_input(input: &str) -> Result<i64> {
    let mut left_numbers = Vec::new();
    let mut right_numbers = Vec::new();

    for line in input.lines() {
        let mut iter = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i64>().ok());

        if let (Some(left), Some(right)) = (iter.next(), iter.next()) {
            left_numbers.push(left);
            right_numbers.push(right);
        }
    }

    left_numbers.sort_unstable();
    right_numbers.sort_unstable();

    Ok(left_numbers
        .iter()
        .zip(right_numbers.iter())
        .fold(0, |acc, (a, b)| acc + (a - b).abs()))
}
