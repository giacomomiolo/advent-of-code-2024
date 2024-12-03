use anyhow::Result;

pub fn solve() -> Result<i64> {
    let input = super::read_input()?;
    solve_for_input(&input)
}

#[inline(always)]
fn is_safe_sequence(numbers: &[i64]) -> bool {
    let len = numbers.len();
    if len < 2 {
        return true;
    }

    let first_diff = numbers[1] - numbers[0];
    if first_diff == 0 || first_diff.abs() > 3 {
        return false;
    }

    let increasing = first_diff > 0;
    let mut prev = numbers[1];

    numbers[2..].iter().all(|&curr| {
        let diff = curr - prev;
        prev = curr;
        diff.abs() <= 3 && diff.abs() >= 1 && 
        ((increasing && diff > 0) || (!increasing && diff < 0))
    })
}

pub fn solve_for_input(input: &str) -> Result<i64> {
    let mut numbers = Vec::with_capacity(8);
    let mut count = 0;

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        numbers.clear();
        numbers.extend(
            line.split_whitespace()
                .filter_map(|n| n.parse::<i64>().ok())
        );

        if is_safe_sequence(&numbers) {
            count += 1;
        }
    }

    Ok(count)
}