use anyhow::Result;

pub fn solve() -> Result<i64> {
    let input = super::read_input()?;
    solve_for_input(&input)
}

#[inline]
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

#[inline]
fn is_safe_with_dampener(numbers: &[i64]) -> bool {
    if is_safe_sequence(numbers) {
        return true;
    }

    let len = numbers.len();
    if len < 2 {
        return true;
    }

    for i in 0..len-1 {
        let diff = numbers[i+1] - numbers[i];
        if diff.abs() <= 3 && diff.abs() >= 1 {
            continue;
        }
        
        if (i > 0 && i < len-1 && check_sequence_without_index(numbers, i)) ||
           (i+1 < len && check_sequence_without_index(numbers, i+1)) {
            return true;
        }
    }

    false
}

#[inline]
fn check_sequence_without_index(numbers: &[i64], skip_idx: usize) -> bool {
    if skip_idx >= numbers.len() {
        return false;
    }

    let mut first_number = true;
    let mut prev_number = 0i64;
    let mut first_diff = 0i64;
    let mut found_first_diff = false;

    for (idx, &num) in numbers.iter().enumerate() {
        if idx == skip_idx {
            continue;
        }

        if first_number {
            first_number = false;
            prev_number = num;
            continue;
        }

        let diff = num - prev_number;
        
        if !found_first_diff {
            first_diff = diff;
            found_first_diff = true;
            if diff == 0 || diff.abs() > 3 {
                return false;
            }
        } else {
            if diff.abs() > 3 || diff.abs() < 1 ||
               (first_diff > 0 && diff <= 0) || 
               (first_diff < 0 && diff >= 0) {
                return false;
            }
        }
        
        prev_number = num;
    }

    found_first_diff
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

        if is_safe_with_dampener(&numbers) {
            count += 1;
        }
    }

    Ok(count)
}